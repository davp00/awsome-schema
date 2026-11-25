//! Adapter from the official `surrealdb` client to [`crate::SurrealOps`].

use std::sync::Arc;

use async_trait::async_trait;
use futures::StreamExt;
use serde_json::Value as JsonValue;
use surrealdb::Surreal;
use surrealdb::method::Transaction;
use surrealdb::opt::auth::Root;
use surrealdb::types::{Number, Object, RecordId, Value};
use surrealdb::{Connection, IndexedResults, Notification};
use tokio::sync::{Mutex, mpsc, oneshot};

use crate::error::{ClientError, err};
use crate::jsonutil::JsonMap;
use crate::ops::{DbHandle, SurrealOps};

/// Live notification action.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LiveAction {
    /// Record created.
    Create,
    /// Record updated.
    Update,
    /// Record deleted.
    Delete,
    /// Live query evaluation error.
    Error,
}

/// One live-query event.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LiveEvent {
    /// What changed.
    pub action: LiveAction,
    /// Record payload.
    pub value: JsonValue,
}

/// Subscription returned by `live` / `live_sql`.
pub struct LiveHandle {
    events: mpsc::UnboundedReceiver<LiveEvent>,
    shutdown: Option<oneshot::Sender<()>>,
    task: Option<tokio::task::JoinHandle<()>>,
}

impl LiveHandle {
    /// Next event, or `None` when the subscription ends.
    pub async fn recv(&mut self) -> Option<LiveEvent> {
        self.events.recv().await
    }

    /// Stop the live query.
    pub async fn kill(mut self) -> Result<(), ClientError> {
        if let Some(shutdown) = self.shutdown.take() {
            let _ = shutdown.send(());
        }
        if let Some(task) = self.task.take() {
            task.await.map_err(|error| err(error.to_string()))?;
        }
        Ok(())
    }
}

impl Drop for LiveHandle {
    fn drop(&mut self) {
        if let Some(shutdown) = self.shutdown.take() {
            let _ = shutdown.send(());
        }
    }
}

/// Connected Surreal session. Clone the [`DbHandle`] for generated delegates.
pub struct Session<C: Connection> {
    db: Surreal<C>,
}

impl<C: Connection> Session<C> {
    /// Wrap an already signed-in client that has selected a namespace and database.
    #[must_use]
    pub fn new(db: Surreal<C>) -> Self {
        Self { db }
    }

    /// Sign in over an existing connection and select the namespace and database.
    pub async fn sign_in(
        db: Surreal<C>,
        namespace: &str,
        database: &str,
        username: &str,
        password: &str,
    ) -> Result<Self, ClientError> {
        db.signin(Root { username: username.to_owned(), password: password.to_owned() })
            .await
            .map_err(|error| err(error.to_string()))?;
        db.use_ns(namespace).use_db(database).await.map_err(|error| err(error.to_string()))?;
        Ok(Self { db })
    }

    /// Operations handle for generated delegates.
    #[must_use]
    pub fn handle(self: &Arc<Self>) -> DbHandle {
        self.clone()
    }

    /// Run `f` in a transaction. Cancel errors after a failure are ignored.
    pub async fn transaction<T, F, Fut>(&self, f: F) -> Result<T, ClientError>
    where
        F: FnOnce(DbHandle) -> Fut,
        Fut: std::future::Future<Output = Result<T, ClientError>> + Send,
        T: Send,
    {
        let txn = SurrealOps::begin_transaction(self).await?;
        match f(txn.clone()).await {
            Ok(value) => {
                txn.commit().await?;
                Ok(value)
            }
            Err(error) => {
                let _ = txn.cancel().await;
                Err(error)
            }
        }
    }

    /// `LIVE SELECT * FROM table`.
    pub async fn live_table(&self, table: &str) -> Result<LiveHandle, ClientError> {
        let mut stream =
            self.db.select(table).live().await.map_err(|error| err(error.to_string()))?;
        let (tx, rx) = mpsc::unbounded_channel();
        let (shutdown_tx, mut shutdown_rx) = oneshot::channel();
        let task = tokio::spawn(async move {
            loop {
                tokio::select! {
                    _ = &mut shutdown_rx => break,
                    item = stream.next() => {
                        let Some(item) = item else { break };
                        let Ok(notification) = item else { continue };
                        if let Some(event) = live_event(notification)
                            && tx.send(event).is_err()
                        {
                            break;
                        }
                    }
                }
            }
        });
        Ok(LiveHandle { events: rx, shutdown: Some(shutdown_tx), task: Some(task) })
    }

    /// Run a `LIVE SELECT` statement and stream its results.
    pub async fn live_sql(
        &self,
        sql: &str,
        vars: Option<&JsonMap>,
    ) -> Result<LiveHandle, ClientError> {
        let trimmed = sql.trim();
        if !is_live_select(trimmed) {
            return Err(err("$live: sql must be a LIVE SELECT statement"));
        }
        let empty = JsonMap::new();
        let encoded = encode_vars(vars.unwrap_or(&empty));
        let mut response =
            self.db.query(trimmed).bind(encoded).await.map_err(|error| err(error.to_string()))?;
        let mut stream = response
            .stream::<Notification<JsonValue>>(0)
            .map_err(|error| err(error.to_string()))?;
        let (tx, rx) = mpsc::unbounded_channel();
        let (shutdown_tx, mut shutdown_rx) = oneshot::channel();
        let task = tokio::spawn(async move {
            loop {
                tokio::select! {
                    _ = &mut shutdown_rx => break,
                    item = stream.next() => {
                        let Some(item) = item else { break };
                        let Ok(notification) = item else { continue };
                        if let Some(event) = live_event_json(notification)
                            && tx.send(event).is_err()
                        {
                            break;
                        }
                    }
                }
            }
        });
        Ok(LiveHandle { events: rx, shutdown: Some(shutdown_tx), task: Some(task) })
    }
}

fn is_live_select(sql: &str) -> bool {
    let mut parts = sql.split_whitespace();
    let Some(live) = parts.next() else { return false };
    let Some(select) = parts.next() else { return false };
    live.eq_ignore_ascii_case("LIVE") && select.eq_ignore_ascii_case("SELECT")
}

fn live_event(notification: Notification<Value>) -> Option<LiveEvent> {
    let action = match notification.action {
        surrealdb::types::Action::Create => LiveAction::Create,
        surrealdb::types::Action::Update => LiveAction::Update,
        surrealdb::types::Action::Delete => LiveAction::Delete,
        surrealdb::types::Action::Error => LiveAction::Error,
        surrealdb::types::Action::Killed => return None,
    };
    Some(LiveEvent { action, value: notification.data.into_json_value() })
}

fn live_event_json(notification: Notification<JsonValue>) -> Option<LiveEvent> {
    let action = match notification.action {
        surrealdb::types::Action::Create => LiveAction::Create,
        surrealdb::types::Action::Update => LiveAction::Update,
        surrealdb::types::Action::Delete => LiveAction::Delete,
        surrealdb::types::Action::Error => LiveAction::Error,
        surrealdb::types::Action::Killed => return None,
    };
    Some(LiveEvent { action, value: notification.data })
}

struct OpenTxn<C: Connection> {
    inner: Mutex<Option<Transaction<C>>>,
}

#[async_trait]
impl<C: Connection> SurrealOps for Session<C> {
    async fn query(&self, sql: &str, vars: &JsonMap) -> Result<JsonValue, ClientError> {
        let encoded = encode_vars(vars);
        let response =
            self.db.query(sql).bind(encoded).await.map_err(|error| err(error.to_string()))?;
        statements_to_json(response)
    }

    fn is_transaction(&self) -> bool {
        false
    }

    async fn begin_transaction(&self) -> Result<DbHandle, ClientError> {
        let txn = self.db.clone().begin().await.map_err(|error| err(error.to_string()))?;
        Ok(Arc::new(OpenTxn { inner: Mutex::new(Some(txn)) }))
    }
}

#[async_trait]
impl<C: Connection> SurrealOps for OpenTxn<C> {
    async fn query(&self, sql: &str, vars: &JsonMap) -> Result<JsonValue, ClientError> {
        let encoded = encode_vars(vars);
        let guard = self.inner.lock().await;
        let Some(txn) = guard.as_ref() else {
            return Err(err("transaction already finished"));
        };
        let response =
            txn.query(sql).bind(encoded).await.map_err(|error| err(error.to_string()))?;
        drop(guard);
        statements_to_json(response)
    }

    fn is_transaction(&self) -> bool {
        true
    }

    async fn commit(&self) -> Result<(), ClientError> {
        let txn =
            self.inner.lock().await.take().ok_or_else(|| err("transaction already finished"))?;
        txn.commit().await.map(|_| ()).map_err(|error| err(error.to_string()))
    }

    async fn cancel(&self) -> Result<(), ClientError> {
        let txn =
            self.inner.lock().await.take().ok_or_else(|| err("transaction already finished"))?;
        txn.cancel().await.map(|_| ()).map_err(|error| err(error.to_string()))
    }
}

fn statements_to_json(mut response: IndexedResults) -> Result<JsonValue, ClientError> {
    let errors = response.take_errors();
    if let Some((_, error)) = errors.into_iter().min_by_key(|(index, _)| *index) {
        return Err(err(error.to_string()));
    }
    let mut statements = Vec::new();
    loop {
        match response.take::<Value>(statements.len()) {
            Ok(value) if value.is_none() => break,
            Ok(value) => statements.push(value.into_json_value()),
            Err(error) => return Err(err(error.to_string())),
        }
    }
    Ok(JsonValue::Array(statements))
}

fn encode_vars(vars: &JsonMap) -> Value {
    let mut object = Object::new();
    for (key, value) in vars {
        object.insert(key.clone(), encode_json(value));
    }
    Value::Object(object)
}

fn encode_json(value: &JsonValue) -> Value {
    match value {
        JsonValue::Null => Value::Null,
        JsonValue::Bool(flag) => Value::Bool(*flag),
        JsonValue::Number(number) => {
            if let Some(integer) = number.as_i64() {
                Value::Number(Number::Int(integer))
            } else if let Some(unsigned) = number.as_u64() {
                Value::Number(Number::Int(i64::try_from(unsigned).unwrap_or(i64::MAX)))
            } else if let Some(float) = number.as_f64() {
                Value::Number(Number::Float(float))
            } else {
                Value::String(number.to_string())
            }
        }
        JsonValue::String(text) => {
            if let Some(record) = encode_record_id(text) {
                Value::RecordId(record)
            } else {
                Value::String(text.clone())
            }
        }
        JsonValue::Array(items) => {
            Value::Array(items.iter().map(encode_json).collect::<Vec<_>>().into())
        }
        JsonValue::Object(map) => {
            let mut object = Object::new();
            for (key, nested) in map {
                object.insert(key.clone(), encode_json(nested));
            }
            Value::Object(object)
        }
    }
}

fn encode_record_id(text: &str) -> Option<RecordId> {
    if !is_record_id_string(text) {
        return None;
    }
    let (table, key) = text.split_once(':')?;
    let key = key.strip_prefix('⟨').and_then(|rest| rest.strip_suffix('⟩')).unwrap_or(key);
    Some(RecordId::new(table, key))
}

fn is_record_id_string(text: &str) -> bool {
    let Some((table, key)) = text.split_once(':') else {
        return false;
    };
    if table.is_empty() || key.is_empty() || key.chars().any(char::is_whitespace) {
        return false;
    }
    let mut chars = table.chars();
    let Some(first) = chars.next() else { return false };
    if !(first.is_ascii_alphabetic() || first == '_') {
        return false;
    }
    chars.all(|ch| ch.is_ascii_alphanumeric() || ch == '_')
}
