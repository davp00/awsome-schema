//! Nested-write behavior shared with the TypeScript runtime tests.

use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use awesome_schema_surreal::{
    ClientError, DbHandle, FieldWriteMeta, NestedOp, RelationDir, SurrealOps, bind_schema_meta,
    collect_edge, create_many_via, is_nested_write_bag, render_nested_ops,
    split_and_resolve_write_data, update_many_records, update_with_nested, with_write_transaction,
};
use serde_json::{Value, json};

struct Fake {
    queries: Mutex<Vec<String>>,
    reply: Mutex<fn(&str) -> Value>,
    transactional: bool,
}

impl Fake {
    fn new(reply: fn(&str) -> Value) -> (Arc<Self>, DbHandle) {
        Self::with_transaction(reply, false)
    }

    fn with_transaction(reply: fn(&str) -> Value, transactional: bool) -> (Arc<Self>, DbHandle) {
        let fake = Arc::new(Self {
            queries: Mutex::new(Vec::new()),
            reply: Mutex::new(reply),
            transactional,
        });
        (fake.clone(), fake)
    }

    fn script(&self) -> String {
        self.queries.lock().expect("queries").join("\n")
    }
}

#[async_trait]
impl SurrealOps for Fake {
    async fn query(
        &self,
        sql: &str,
        _vars: &serde_json::Map<String, Value>,
    ) -> Result<Value, ClientError> {
        self.queries.lock().expect("queries").push(sql.to_owned());
        let reply = *self.reply.lock().expect("reply");
        Ok(reply(sql))
    }

    fn is_transaction(&self) -> bool {
        self.transactional
    }
}

fn lock_meta() -> std::sync::MutexGuard<'static, ()> {
    static LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
    LOCK.lock().expect("schema meta lock")
}

fn liked() -> FieldWriteMeta {
    FieldWriteMeta::Edge {
        edge_table: "likes".to_owned(),
        dir: RelationDir::Out,
        list: true,
        in_table: "user".to_owned(),
        out_table: "post".to_owned(),
        payload_fields: vec!["score".to_owned()],
    }
}

fn rows(value: Value) -> Value {
    Value::Array(vec![value])
}

#[test]
fn nested_write_bag_detects_operation_keys() {
    assert!(is_nested_write_bag(&json!({"create": []})));
    assert!(!is_nested_write_bag(&json!({"id": "user:1"})));
}

#[test]
fn renders_relate_new_far_with_as_parent() {
    let (sql, _) = render_nested_ops(&[NestedOp::RelateNewFar {
        edge_table: "likes".to_owned(),
        far_table: "post".to_owned(),
        far_is_out: true,
        items: vec![json!({"far": {}, "content": {}, "as_parent": "user:1"})],
    }]);
    assert!(sql.contains("LET $as_parent = type::record($item.as_parent)"), "{sql}");
    assert!(sql.contains("$as_parent->likes->$far"), "{sql}");
    assert!(!sql.contains("$parent"), "{sql}");
}

#[tokio::test]
async fn sets_and_upserts_edges_with_record_lookups() {
    let (fake, db) = Fake::new(|sql| {
        if sql.contains("WHERE in = type::record($v)") {
            return rows(json!([{"id": "likes:old"}]));
        }
        if sql.contains("out = type::record($far)") {
            return rows(json!([{"id": "likes:edge"}]));
        }
        Value::Array(vec![])
    });
    let mut ops = Vec::new();
    collect_edge(
        &db,
        &mut ops,
        "user",
        "user:1",
        "liked",
        &json!({
            "set": [{"out": "post:a", "score": 1}],
            "upsert": [{
                "where": {"out": "post:a"},
                "update": {"score": 4},
                "create": {"out": "post:a", "score": 4}
            }]
        })
        .as_object()
        .expect("bag")
        .clone(),
        &liked(),
    )
    .await
    .expect("collect");
    let sql = fake.script();
    assert!(sql.contains("in = type::record($v)"), "{sql}");
    assert!(sql.contains("out = type::record($far)"), "{sql}");
    assert!(sql.contains("DELETE"), "{sql}");
    assert!(sql.contains("INSERT RELATION INTO likes"), "{sql}");
    assert!(sql.contains("MERGE"), "{sql}");
}

#[tokio::test]
async fn skips_duplicate_edge_connect() {
    let (fake, db) = Fake::new(|sql| {
        if sql.contains("out = type::record($far)") {
            return rows(json!([{"id": "likes:existing"}]));
        }
        Value::Array(vec![])
    });
    let mut ops = Vec::new();
    collect_edge(
        &db,
        &mut ops,
        "user",
        "user:1",
        "liked",
        &json!({"connect": [{"out": "post:a", "score": 3}]}).as_object().expect("bag").clone(),
        &liked(),
    )
    .await
    .expect("collect");
    let sql = fake.script();
    assert!(sql.contains("MERGE"), "{sql}");
    assert!(!sql.contains("INSERT RELATION"), "{sql}");
}

#[tokio::test]
#[allow(clippy::await_holding_lock)]
async fn rejects_diff_nested_writes_and_missing_transaction() {
    let _guard = lock_meta();
    let mut user = std::collections::BTreeMap::new();
    user.insert(
        "posts".to_owned(),
        FieldWriteMeta::Computed {
            target_table: "post".to_owned(),
            list: true,
            back_link_field: "author".to_owned(),
            back_link_optional: false,
            unique_fields: Vec::new(),
        },
    );
    let mut write = std::collections::BTreeMap::new();
    write.insert("user".to_owned(), user);
    bind_schema_meta(std::collections::BTreeMap::new(), write);

    let (_fake, db) = Fake::new(|_| Value::Array(vec![]));
    let error = create_many_via(
        &db,
        "user",
        &json!({
            "data": [{"posts": {"create": [{"title": "x"}]}}],
            "return": "DIFF"
        }),
        false,
        |_row| async { Ok(json!({"id": "user:1"})) },
    )
    .await
    .expect_err("diff");
    assert!(error.message().contains("DIFF is not supported with nested writes"), "{error}");

    let error = update_many_records(
        &db,
        "user",
        &json!({
            "where": {"email": "a@b.c"},
            "data": {"posts": {"create": [{"title": "x"}]}},
            "return": "DIFF"
        }),
    )
    .await
    .expect_err("update diff");
    assert!(error.message().contains("return DIFF is not supported with nested writes"), "{error}");

    let error = with_write_transaction(&db, true, |_| async { Ok("ok") }).await.expect_err("txn");
    assert!(error.message().contains("beginTransaction"), "{error}");
}

#[tokio::test]
#[allow(clippy::await_holding_lock)]
async fn resolves_stored_connect_or_create_and_optional_delete() {
    let _guard = lock_meta();
    let mut post = std::collections::BTreeMap::new();
    post.insert(
        "author".to_owned(),
        FieldWriteMeta::Stored {
            target_table: "user".to_owned(),
            list: false,
            optional: false,
            unique_fields: vec!["email".to_owned()],
        },
    );
    post.insert("title".to_owned(), FieldWriteMeta::Scalar);
    let mut write = std::collections::BTreeMap::new();
    write.insert("post".to_owned(), post);
    bind_schema_meta(std::collections::BTreeMap::new(), write);

    let (found_fake, found) = Fake::new(|sql| {
        if sql.contains("WHERE email = $v") {
            rows(json!([{"id": "user:9"}]))
        } else {
            Value::Array(vec![])
        }
    });
    let data = json!({
        "title": "t",
        "author": {
            "connectOrCreate": {
                "where": {"email": "ada@example.com"},
                "create": {"email": "ada@example.com"}
            }
        }
    });
    let (scalars, _, _) =
        split_and_resolve_write_data(&found, "post", data.as_object().expect("data"), None)
            .await
            .expect("resolve");
    assert_eq!(scalars.get("author"), Some(&json!("user:9")));
    assert_eq!(scalars.get("title"), Some(&json!("t")));
    let sql = found_fake.script();
    assert!(sql.contains("email = $v"), "{sql}");
    assert!(!sql.contains("email = type::record"), "{sql}");

    let error = split_and_resolve_write_data(
        &Fake::new(|_| Value::Array(vec![])).1,
        "post",
        json!({"author": {"delete": true}}).as_object().expect("data"),
        Some("post:1"),
    )
    .await
    .expect_err("required delete");
    assert!(error.message().contains("cannot delete required link"), "{error}");

    let mut post = std::collections::BTreeMap::new();
    post.insert(
        "author".to_owned(),
        FieldWriteMeta::Stored {
            target_table: "user".to_owned(),
            list: false,
            optional: true,
            unique_fields: vec!["email".to_owned()],
        },
    );
    post.insert("title".to_owned(), FieldWriteMeta::Scalar);
    let mut write = std::collections::BTreeMap::new();
    write.insert("post".to_owned(), post);
    bind_schema_meta(std::collections::BTreeMap::new(), write);

    let (fake, db) = Fake::with_transaction(
        |sql| {
            if sql.contains("SELECT author FROM type::record($thing)") {
                rows(json!([{"author": "user:old"}]))
            } else if sql.contains("SELECT * FROM") {
                rows(json!([{"id": "post:1", "author": null}]))
            } else {
                Value::Array(vec![])
            }
        },
        true,
    );
    update_with_nested(&db, "post", "post:1", &json!({"author": {"delete": true}}), None)
        .await
        .expect("update");
    let sql = fake.script();
    let unset = sql.find("UNSET author").expect("unset");
    let delete = sql.rfind("DELETE type::record").expect("delete");
    assert!(unset < delete, "{sql}");
    assert!(!sql.contains("MERGE"), "{sql}");
}
