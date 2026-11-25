//! Surreal operations and the query/write API generated clients call.

use std::future::Future;
use std::sync::Arc;

use async_trait::async_trait;
use serde::de::DeserializeOwned;
use serde_json::{Map, Value};

use crate::error::{ClientError, err};
use crate::id::normalize_thing;
use crate::jsonutil::{JsonMap, as_rows, first_row, row_id};
use crate::nested::{
    apply_nested_writes, is_nested_write_bag, partition_null_link_clears, row_has_nested_write,
    run_deferred_deletes, split_and_resolve_write_data, unset_record_fields,
};
use crate::sql::{
    GroupByAlias, WhereCtx, append_limit_start, assert_non_empty_where,
    assert_return_select_exclusive, build_group_by_order_by, build_group_having, build_order_by,
    build_projection, build_where, collect_order_by_count_projections, project_row,
    reshape_group_by_row, strip_order_by_count_fields, unique_where_id,
};

/// Handle shared by generated delegates. Cheap to clone.
pub type DbHandle = Arc<dyn SurrealOps>;

/// Operations the runtime needs from a session or an open transaction.
#[async_trait]
pub trait SurrealOps: Send + Sync {
    /// Run SurrealQL. The value is an array of per-statement results.
    async fn query(&self, sql: &str, vars: &JsonMap) -> Result<Value, ClientError>;

    /// Whether this handle is already an interactive transaction.
    fn is_transaction(&self) -> bool;

    /// Open a transaction. Sessions implement this; transactions do not.
    async fn begin_transaction(&self) -> Result<DbHandle, ClientError> {
        let _ = self;
        Err(err("nested writes require a session with beginTransaction()"))
    }

    /// Commit when this handle is a transaction.
    async fn commit(&self) -> Result<(), ClientError> {
        Err(err("not a transaction"))
    }

    /// Cancel when this handle is a transaction.
    async fn cancel(&self) -> Result<(), ClientError> {
        Err(err("not a transaction"))
    }
}

/// Run `f` inside a transaction when `needs_txn` and `db` is not already one.
pub async fn with_write_transaction<T, F, Fut>(
    db: &DbHandle,
    needs_txn: bool,
    f: F,
) -> Result<T, ClientError>
where
    F: FnOnce(DbHandle) -> Fut,
    Fut: Future<Output = Result<T, ClientError>> + Send,
    T: Send,
{
    if !needs_txn || db.is_transaction() {
        return f(db.clone()).await;
    }
    let txn = db.begin_transaction().await?;
    match f(txn.clone()).await {
        Ok(value) => {
            txn.commit().await?;
            Ok(value)
        }
        Err(error) => {
            txn.cancel().await?;
            Err(error)
        }
    }
}

/// Rows from one query. Unwraps the statement-result envelope.
pub async fn query_rows(
    db: &DbHandle,
    sql: &str,
    vars: &JsonMap,
) -> Result<Vec<Value>, ClientError> {
    Ok(as_rows(db.query(sql, vars).await?))
}

/// `SELECT * FROM type::record`.
pub async fn select_record<T: DeserializeOwned>(
    db: &DbHandle,
    table: &str,
    id: &str,
) -> Result<Option<T>, ClientError> {
    let mut vars = JsonMap::new();
    vars.insert("thing".to_owned(), Value::String(normalize_thing(table, id)));
    let row = first_row(db.query("SELECT * FROM type::record($thing)", &vars).await?);
    match row {
        Some(row) => Ok(Some(from_json(row)?)),
        None => Ok(None),
    }
}

/// `SELECT * ... FETCH`.
pub async fn select_record_related<T: DeserializeOwned>(
    db: &DbHandle,
    table: &str,
    id: &str,
    fetch: &[String],
) -> Result<Option<T>, ClientError> {
    if fetch.is_empty() {
        return select_record(db, table, id).await;
    }
    let mut vars = JsonMap::new();
    vars.insert("thing".to_owned(), Value::String(normalize_thing(table, id)));
    let sql = format!("SELECT * FROM type::record($thing) FETCH {}", fetch.join(", "));
    let row = first_row(db.query(&sql, &vars).await?);
    match row {
        Some(row) => Ok(Some(from_json(row)?)),
        None => Ok(None),
    }
}

/// `CREATE table CONTENT`.
pub async fn create_record<T: DeserializeOwned>(
    db: &DbHandle,
    table: &str,
    data: &JsonMap,
) -> Result<T, ClientError> {
    let mut vars = JsonMap::new();
    vars.insert("data".to_owned(), Value::Object(data.clone()));
    let row =
        first_row(db.query(&format!("CREATE {table} CONTENT $data RETURN AFTER"), &vars).await?);
    let Some(row) = row else {
        return Err(err(format!("create {table} returned no row")));
    };
    from_json(row)
}

/// `UPDATE ... MERGE` returning the row.
pub async fn update_record<T: DeserializeOwned>(
    db: &DbHandle,
    table: &str,
    id: &str,
    data: &JsonMap,
) -> Result<Option<T>, ClientError> {
    let mut vars = JsonMap::new();
    vars.insert("thing".to_owned(), Value::String(normalize_thing(table, id)));
    vars.insert("data".to_owned(), Value::Object(data.clone()));
    let row =
        first_row(db.query("UPDATE type::record($thing) MERGE $data RETURN AFTER", &vars).await?);
    match row {
        Some(row) => Ok(Some(from_json(row)?)),
        None => Ok(None),
    }
}

/// Result of a single-row mutation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SingleResult {
    /// JS `undefined` / `void`.
    Empty,
    /// A row, or absence when the record was missing.
    Row(Option<Value>),
    /// `RETURN DIFF`.
    Diff(Value),
}

impl SingleResult {
    /// JSON shape returned to generated clients.
    #[must_use]
    pub fn into_value(self) -> Value {
        match self {
            Self::Empty => Value::Null,
            Self::Row(row) => row.unwrap_or(Value::Null),
            Self::Diff(value) => value,
        }
    }
}

/// Result of a many-row mutation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ManyResult {
    /// `{ count }` when `return` is omitted or `NONE` without a projection.
    Count {
        /// Affected rows.
        count: u64,
    },
    /// Returned or projected rows, including flattened `DIFF` parts.
    Rows(Vec<Value>),
}

impl ManyResult {
    /// JSON shape returned to generated clients: `{ count }` or a row array.
    #[must_use]
    pub fn into_value(self) -> Value {
        match self {
            Self::Count { count } => {
                let mut map = Map::new();
                map.insert("count".to_owned(), Value::from(count));
                Value::Object(map)
            }
            Self::Rows(rows) => Value::Array(rows),
        }
    }
}

/// `RETURN` mode. Absent means the runtime default for that operation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MutationReturn {
    /// `RETURN NONE`.
    None,
    /// `RETURN BEFORE`.
    Before,
    /// `RETURN AFTER`.
    After,
    /// `RETURN DIFF`.
    Diff,
}

impl MutationReturn {
    /// Parse the TypeScript string.
    #[must_use]
    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "NONE" => Some(Self::None),
            "BEFORE" => Some(Self::Before),
            "AFTER" => Some(Self::After),
            "DIFF" => Some(Self::Diff),
            _ => None,
        }
    }

    const fn as_sql(self) -> &'static str {
        match self {
            Self::None => "NONE",
            Self::Before => "BEFORE",
            Self::After => "AFTER",
            Self::Diff => "DIFF",
        }
    }
}

fn return_mode(value: Option<&Value>) -> Result<Option<MutationReturn>, ClientError> {
    let Some(value) = value else {
        return Ok(None);
    };
    if value.is_null() {
        return Ok(None);
    }
    let Some(text) = value.as_str() else {
        return Err(err("return must be NONE, BEFORE, AFTER, or DIFF"));
    };
    MutationReturn::parse(text)
        .map(Some)
        .ok_or_else(|| err("return must be NONE, BEFORE, AFTER, or DIFF"))
}

/// Update one record, honoring `select` / `return`.
pub async fn update_one_record(
    db: &DbHandle,
    table: &str,
    id: &str,
    data: &JsonMap,
    opts: Option<&Value>,
) -> Result<SingleResult, ClientError> {
    let select = opts.and_then(|value| value.get("select"));
    let ret = return_mode(opts.and_then(|value| value.get("return")))?;
    assert_return_select_exclusive("update", table, select, ret.map(MutationReturn::as_sql))?;
    let thing = normalize_thing(table, id);
    let mut vars = JsonMap::new();
    vars.insert("__data".to_owned(), Value::Object(data.clone()));
    vars.insert("thing".to_owned(), Value::String(thing));
    if select.is_none() && ret.is_none() {
        let row = first_row(
            db.query("UPDATE type::record($thing) MERGE $__data RETURN AFTER", &vars).await?,
        );
        return Ok(SingleResult::Row(row));
    }
    if let Some(select) = select {
        let projection = build_projection(Some(select), table)?;
        let rows = query_rows(
            db,
            &format!("UPDATE type::record($thing) MERGE $__data RETURN {projection}"),
            &vars,
        )
        .await?;
        return Ok(SingleResult::Row(rows.into_iter().next()));
    }
    match ret {
        Some(MutationReturn::None) => {
            query_rows(db, "UPDATE type::record($thing) MERGE $__data RETURN NONE", &vars).await?;
            Ok(SingleResult::Empty)
        }
        Some(MutationReturn::Diff) => {
            let rows =
                query_rows(db, "UPDATE type::record($thing) MERGE $__data RETURN DIFF", &vars)
                    .await?;
            Ok(SingleResult::Diff(rows.into_iter().next().unwrap_or(Value::Null)))
        }
        Some(mode) => {
            let rows = query_rows(
                db,
                &format!("UPDATE type::record($thing) MERGE $__data RETURN {}", mode.as_sql()),
                &vars,
            )
            .await?;
            Ok(SingleResult::Row(rows.into_iter().next()))
        }
        None => Ok(SingleResult::Empty),
    }
}

/// Delete one record. The default is `Empty`.
pub async fn delete_record(db: &DbHandle, table: &str, id: &str) -> Result<(), ClientError> {
    delete_one_record(db, table, id, None).await.map(|_| ())
}

/// Delete one record, honoring `select` / `return`.
pub async fn delete_one_record(
    db: &DbHandle,
    table: &str,
    id: &str,
    opts: Option<&Value>,
) -> Result<SingleResult, ClientError> {
    let select = opts.and_then(|value| value.get("select"));
    let ret = return_mode(opts.and_then(|value| value.get("return")))?;
    assert_return_select_exclusive("delete", table, select, ret.map(MutationReturn::as_sql))?;
    if ret == Some(MutationReturn::After) {
        return Err(err(format!("delete {table}: return AFTER is not supported (use BEFORE)")));
    }
    let thing = normalize_thing(table, id);
    let mut vars = JsonMap::new();
    vars.insert("thing".to_owned(), Value::String(thing.clone()));
    if select.is_none() && ret.is_none() {
        db.query("DELETE type::record($thing) RETURN NONE", &vars).await?;
        return Ok(SingleResult::Empty);
    }
    if let Some(select) = select {
        let rows = query_rows(db, "DELETE type::record($thing) RETURN BEFORE", &vars).await?;
        let Some(row) = rows.into_iter().next() else {
            return Err(err(format!("delete {table}: no row returned for {thing}")));
        };
        return Ok(SingleResult::Row(Some(project_row(&row, Some(select)))));
    }
    match ret {
        Some(MutationReturn::None) => {
            query_rows(db, "DELETE type::record($thing) RETURN NONE", &vars).await?;
            Ok(SingleResult::Empty)
        }
        Some(MutationReturn::Diff) => {
            let rows = query_rows(db, "DELETE type::record($thing) RETURN DIFF", &vars).await?;
            Ok(SingleResult::Diff(rows.into_iter().next().unwrap_or(Value::Null)))
        }
        Some(MutationReturn::Before) | None => {
            let rows = query_rows(db, "DELETE type::record($thing) RETURN BEFORE", &vars).await?;
            Ok(SingleResult::Row(rows.into_iter().next()))
        }
        Some(MutationReturn::After) => unreachable!("rejected above"),
    }
}

/// `RELATE` an edge row.
pub async fn relate_edge(
    db: &DbHandle,
    edge_table: &str,
    in_table: &str,
    out_table: &str,
    in_id: &str,
    out_id: &str,
    content: &JsonMap,
) -> Result<Value, ClientError> {
    let mut vars = JsonMap::new();
    vars.insert("in".to_owned(), Value::String(normalize_thing(in_table, in_id)));
    vars.insert("out".to_owned(), Value::String(normalize_thing(out_table, out_id)));
    vars.insert("content".to_owned(), Value::Object(content.clone()));
    let row = first_row(
        db.query(&format!("RELATE $in->{edge_table}->$out CONTENT $content"), &vars).await?,
    );
    row.ok_or_else(|| err(format!("relate {edge_table} returned no row")))
}

/// Create a record and apply one-hop nested writes.
pub async fn create_with_nested<T: DeserializeOwned + Send>(
    db: &DbHandle,
    table: &str,
    data: &Value,
) -> Result<T, ClientError> {
    let data_map = data.as_object().cloned().unwrap_or_default();
    let has_nested = row_has_nested_write(table, data);
    let row = with_write_transaction(db, has_nested, |ops| {
        let data_map = data_map.clone();
        let table = table.to_owned();
        async move {
            let (scalars, nested, delete_after) =
                split_and_resolve_write_data(&ops, &table, &data_map, None).await?;
            let created = create_record::<Value>(&ops, &table, &scalars).await?;
            let id = row_id(&created).unwrap_or_default();
            apply_nested_writes(&ops, &table, &id, &nested).await?;
            run_deferred_deletes(&ops, &delete_after).await?;
            Ok(created)
        }
    })
    .await?;
    from_json(row)
}

/// Update a record and apply one-hop nested writes.
pub async fn update_with_nested(
    db: &DbHandle,
    table: &str,
    id: &str,
    data: &Value,
    opts: Option<&Value>,
) -> Result<SingleResult, ClientError> {
    let data_map = data.as_object().cloned().unwrap_or_default();
    let has_nested = row_has_nested_write(table, data);
    let opts = opts.cloned();
    let id = id.to_owned();
    let table = table.to_owned();
    with_write_transaction(db, has_nested, |ops| {
        let data_map = data_map.clone();
        let opts = opts.clone();
        let id = id.clone();
        let table = table.clone();
        async move {
            let (scalars, nested, delete_after) =
                split_and_resolve_write_data(&ops, &table, &data_map, Some(&id)).await?;
            let (patch, unset_fields) = partition_null_link_clears(&table, &scalars);
            unset_record_fields(&ops, &table, &id, &unset_fields).await?;
            let ret = opts.as_ref().and_then(|value| value.get("return"));
            let select = opts.as_ref().and_then(|value| value.get("select"));
            let updated = if patch.is_empty()
                && !unset_fields.is_empty()
                && select.is_none()
                && ret.is_none()
            {
                let row = select_record::<Value>(&ops, &table, &id).await?;
                SingleResult::Row(row)
            } else {
                update_one_record(&ops, &table, &id, &patch, opts.as_ref()).await?
            };
            apply_nested_writes(&ops, &table, &id, &nested).await?;
            run_deferred_deletes(&ops, &delete_after).await?;
            Ok(updated)
        }
    })
    .await
}

/// Find one row. An id-only where reads the record directly.
pub async fn find_unique_record<T: DeserializeOwned>(
    db: &DbHandle,
    table: &str,
    args: &Value,
) -> Result<Option<T>, ClientError> {
    let where_value = args.get("where").cloned().unwrap_or(Value::Null);
    let select = args.get("select");
    let id_only = unique_where_id(Some(&where_value));
    if let Some(id_only) = id_only {
        if select.is_none() {
            return select_record(db, table, &id_only).await;
        }
        let mut vars = JsonMap::new();
        vars.insert("thing".to_owned(), Value::String(normalize_thing(table, &id_only)));
        let projection = build_projection(select, table)?;
        let row = first_row(
            db.query(&format!("SELECT {projection} FROM type::record($thing)"), &vars).await?,
        );
        return match row {
            Some(row) => Ok(Some(from_json(row)?)),
            None => Ok(None),
        };
    }
    let projection =
        if select.is_some() { build_projection(select, table)? } else { "*".to_owned() };
    let mut vars = args.get("vars").and_then(Value::as_object).cloned().unwrap_or_default();
    let mut ctx = WhereCtx { vars, n: 0 };
    let where_clause = build_where(&where_value, table, &mut ctx)?;
    if where_clause.is_empty() {
        return Ok(None);
    }
    vars = ctx.vars;
    let rows = query_rows(
        db,
        &format!("SELECT {projection} FROM {table} WHERE {where_clause} LIMIT 1"),
        &vars,
    )
    .await?;
    match rows.into_iter().next() {
        Some(row) => Ok(Some(from_json(row)?)),
        None => Ok(None),
    }
}

/// Find many rows.
pub async fn find_many_records<T: DeserializeOwned>(
    db: &DbHandle,
    table: &str,
    args: &Value,
) -> Result<Vec<T>, ClientError> {
    let select = args.get("select");
    let mut projection =
        if select.is_some() { build_projection(select, table)? } else { "*".to_owned() };
    let count_proj = collect_order_by_count_projections(args.get("orderBy"), table);
    if !count_proj.is_empty() {
        projection = if projection == "*" {
            format!("*, {}", count_proj.join(", "))
        } else {
            format!("{projection}, {}", count_proj.join(", "))
        };
    }
    let mut vars = args.get("vars").and_then(Value::as_object).cloned().unwrap_or_default();
    let sql = if let Some(where_value) = args.get("where") {
        let mut ctx = WhereCtx { vars: vars.clone(), n: 0 };
        let where_clause = build_where(where_value, table, &mut ctx)?;
        vars = ctx.vars;
        if where_clause.is_empty() {
            format!("SELECT {projection} FROM {table}")
        } else {
            format!("SELECT {projection} FROM {table} WHERE {where_clause}")
        }
    } else if let Some(where_sql) = args.get("whereSql").and_then(Value::as_str) {
        replace_select_star(where_sql, &projection)
    } else {
        format!("SELECT {projection} FROM {table}")
    };
    let mut sql = sql;
    let mut order = build_order_by(args.get("orderBy"), table);
    if order.is_empty() && (args.get("take").is_some() || args.get("skip").is_some()) {
        order = String::from("ORDER BY id");
    }
    if !order.is_empty() {
        sql = format!("{sql} {order}");
    }
    sql = append_limit_start(&sql, args.get("take"), args.get("skip"), &mut vars)?;
    let rows = strip_order_by_count_fields(query_rows(db, &sql, &vars).await?);
    rows.into_iter().map(from_json).collect()
}

fn replace_select_star(sql: &str, projection: &str) -> String {
    let trimmed = sql.trim_start();
    let lower = trimmed.to_ascii_lowercase();
    if let Some(rest) = lower.strip_prefix("select") {
        let rest_orig = &trimmed["select".len().min(trimmed.len())..];
        let after = rest.trim_start();
        if after.starts_with('*') {
            let star_at = rest_orig.find('*').unwrap_or(0);
            return format!("SELECT {projection}{}", &rest_orig[star_at + 1..]);
        }
    }
    sql.to_owned()
}

/// `count()`.
pub async fn count_records(db: &DbHandle, table: &str, args: &Value) -> Result<u64, ClientError> {
    let mut vars = args.get("vars").and_then(Value::as_object).cloned().unwrap_or_default();
    let mut sql = format!("SELECT count() AS count FROM {table}");
    if let Some(where_value) = args.get("where") {
        let mut ctx = WhereCtx { vars: vars.clone(), n: 0 };
        let clause = build_where(where_value, table, &mut ctx)?;
        vars = ctx.vars;
        if !clause.is_empty() {
            sql.push_str(" WHERE ");
            sql.push_str(&clause);
        }
    }
    sql.push_str(" GROUP ALL");
    let rows = query_rows(db, &sql, &vars).await?;
    Ok(rows
        .first()
        .and_then(|row| row.get("count"))
        .and_then(crate::jsonutil::number_of)
        .unwrap_or(0.0) as u64)
}

async fn count_matching(
    db: &DbHandle,
    table: &str,
    where_clause: &str,
    vars: &JsonMap,
) -> Result<u64, ClientError> {
    let rows = query_rows(
        db,
        &format!("SELECT count() AS count FROM {table} WHERE {where_clause} GROUP ALL"),
        vars,
    )
    .await?;
    Ok(rows
        .first()
        .and_then(|row| row.get("count"))
        .and_then(crate::jsonutil::number_of)
        .unwrap_or(0.0) as u64)
}

/// Grouped aggregates. The shape matches the TypeScript client.
pub async fn group_by_records(
    db: &DbHandle,
    table: &str,
    args: &Value,
) -> Result<Vec<Value>, ClientError> {
    let by = args
        .get("by")
        .and_then(Value::as_array)
        .map(|items| {
            items.iter().filter_map(|item| item.as_str().map(ToOwned::to_owned)).collect::<Vec<_>>()
        })
        .unwrap_or_default();
    if by.is_empty() {
        return Err(err(format!("groupBy {table}: by must not be empty")));
    }
    let count_spec = match args.get("_count") {
        Some(Value::Bool(true)) => Some(json_count_all()),
        Some(Value::Object(map)) => Some(map.clone()),
        _ => None,
    };
    let has_count = count_spec
        .as_ref()
        .is_some_and(|spec| spec.values().any(|value| value.as_bool() == Some(true)));
    let has_math = ["_sum", "_avg", "_min", "_max"].iter().any(|key| {
        args.get(*key)
            .and_then(Value::as_object)
            .is_some_and(|spec| spec.values().any(|value| value.as_bool() == Some(true)))
    });
    if !has_count && !has_math {
        return Err(err(format!(
            "groupBy {table}: at least one aggregate (_count/_sum/_avg/_min/_max) is required"
        )));
    }
    let mut vars = args.get("vars").and_then(Value::as_object).cloned().unwrap_or_default();
    let mut ctx = WhereCtx { vars: vars.clone(), n: 0 };
    let mut select_parts = by.clone();
    let mut aliases = Vec::new();
    if let Some(count_spec) = &count_spec {
        if count_spec.get("_all").and_then(Value::as_bool) == Some(true) {
            select_parts.push("count() AS __count_all".to_owned());
            aliases.push(GroupByAlias {
                kind: "_count".to_owned(),
                field: "_all".to_owned(),
                alias: "__count_all".to_owned(),
            });
        }
        for (field, on) in count_spec {
            if field == "_all" || on.as_bool() != Some(true) {
                continue;
            }
            let alias = format!("__count_{field}");
            select_parts.push(format!("count({field}) AS {alias}"));
            aliases.push(GroupByAlias { kind: "_count".to_owned(), field: field.clone(), alias });
        }
    }
    let math = [
        ("_sum", "math::sum"),
        ("_avg", "math::mean"),
        ("_min", "math::min"),
        ("_max", "math::max"),
    ];
    for (kind, function) in math {
        let Some(spec) = args.get(kind).and_then(Value::as_object) else { continue };
        for (field, on) in spec {
            if on.as_bool() != Some(true) {
                continue;
            }
            let alias = format!("__{}_{field}", &kind[1..]);
            select_parts.push(format!("{function}({field}) AS {alias}"));
            aliases.push(GroupByAlias { kind: kind.to_owned(), field: field.clone(), alias });
        }
    }
    let mut sql = format!("SELECT {} FROM {table}", select_parts.join(", "));
    if let Some(where_value) = args.get("where") {
        let clause = build_where(where_value, table, &mut ctx)?;
        if !clause.is_empty() {
            sql.push_str(" WHERE ");
            sql.push_str(&clause);
        }
    }
    sql.push_str(" GROUP BY ");
    sql.push_str(&by.join(", "));
    let having = build_group_having(table, args.get("having"), &by, &aliases, &mut ctx)?;
    if !having.is_empty() {
        sql = format!("SELECT * FROM ({sql}) WHERE {having}");
    }
    vars = ctx.vars;
    let order = build_group_by_order_by(args.get("orderBy"), &by);
    if !order.is_empty() {
        sql.push(' ');
        sql.push_str(&order);
    }
    sql = append_limit_start(&sql, args.get("take"), args.get("skip"), &mut vars)?;
    let rows = query_rows(db, &sql, &vars).await?;
    Ok(rows.iter().map(|row| reshape_group_by_row(row, &by, &aliases)).collect())
}

fn json_count_all() -> Map<String, Value> {
    let mut map = Map::new();
    map.insert("_all".to_owned(), Value::Bool(true));
    map
}

async fn insert_many_rows(
    db: &DbHandle,
    table: &str,
    rows: &[Value],
    relation: bool,
    returning: &str,
) -> Result<Vec<Value>, ClientError> {
    if rows.is_empty() {
        return Ok(Vec::new());
    }
    let keyword = if relation { "INSERT RELATION INTO" } else { "INSERT INTO" };
    let mut vars = JsonMap::new();
    vars.insert("rows".to_owned(), Value::Array(rows.to_vec()));
    query_rows(db, &format!("{keyword} {table} $rows RETURN {returning}"), &vars).await
}

/// Bulk create. `create_one` is used when a row contains nested writes.
pub async fn create_many_via<F, Fut>(
    db: &DbHandle,
    table: &str,
    args: &Value,
    relation: bool,
    mut create_one: F,
) -> Result<ManyResult, ClientError>
where
    F: FnMut(JsonMap) -> Fut,
    Fut: Future<Output = Result<Value, ClientError>>,
{
    let data = args.get("data").and_then(Value::as_array).cloned().unwrap_or_default();
    let select = args.get("select");
    let ret = return_mode(args.get("return"))?;
    assert_return_select_exclusive("createMany", table, select, ret.map(MutationReturn::as_sql))?;
    if ret == Some(MutationReturn::Before) {
        return Err(err(format!(
            "createMany {table}: return BEFORE is not supported (no prior row)"
        )));
    }
    let has_nested = data.iter().any(|row| row_has_nested_write(table, row));
    if has_nested {
        if ret == Some(MutationReturn::Diff) {
            return Err(err(format!(
                "createMany {table}: return DIFF is not supported with nested writes (use create / createMany without DIFF)"
            )));
        }
        if let Some(select) = select {
            let mut rows = Vec::new();
            for item in &data {
                let created = create_one(item.as_object().cloned().unwrap_or_default()).await?;
                let Some(id) = row_id(&created) else {
                    return Err(err(format!("createMany {table}: created row missing id")));
                };
                let mut reload = Map::new();
                let mut where_value = Map::new();
                where_value.insert("id".to_owned(), Value::String(id.clone()));
                reload.insert("where".to_owned(), Value::Object(where_value));
                reload.insert("select".to_owned(), select.clone());
                let Some(projected) =
                    find_unique_record::<Value>(db, table, &Value::Object(reload)).await?
                else {
                    return Err(err(format!("createMany {table}: could not reload {id}")));
                };
                rows.push(projected);
            }
            return Ok(ManyResult::Rows(rows));
        }
        if ret == Some(MutationReturn::After) {
            let mut rows = Vec::new();
            for item in &data {
                rows.push(create_one(item.as_object().cloned().unwrap_or_default()).await?);
            }
            return Ok(ManyResult::Rows(rows));
        }
        for item in &data {
            create_one(item.as_object().cloned().unwrap_or_default()).await?;
        }
        return Ok(ManyResult::Count { count: data.len() as u64 });
    }
    if ret == Some(MutationReturn::Diff) {
        let parts = insert_many_rows(db, table, &data, relation, "DIFF").await?;
        let flat = parts
            .into_iter()
            .flat_map(|part| match part {
                Value::Array(items) => items,
                other => vec![other],
            })
            .collect();
        return Ok(ManyResult::Rows(flat));
    }
    if let Some(select) = select {
        let projection = build_projection(Some(select), table)?;
        return Ok(ManyResult::Rows(
            insert_many_rows(db, table, &data, relation, &projection).await?,
        ));
    }
    if ret == Some(MutationReturn::After) {
        return Ok(ManyResult::Rows(insert_many_rows(db, table, &data, relation, "AFTER").await?));
    }
    insert_many_rows(db, table, &data, relation, "NONE").await?;
    Ok(ManyResult::Count { count: data.len() as u64 })
}

/// Update every row matching `where`.
pub async fn update_many_records(
    db: &DbHandle,
    table: &str,
    args: &Value,
) -> Result<ManyResult, ClientError> {
    let where_value = args.get("where");
    assert_non_empty_where(where_value, "updateMany", table)?;
    let select = args.get("select");
    let ret = return_mode(args.get("return"))?;
    assert_return_select_exclusive("updateMany", table, select, ret.map(MutationReturn::as_sql))?;
    let data = args.get("data").cloned().unwrap_or_else(|| Value::Object(Map::new()));
    if row_has_nested_write(table, &data) {
        if matches!(ret, Some(MutationReturn::Diff | MutationReturn::Before)) {
            return Err(err(format!(
                "updateMany {table}: return {} is not supported with nested writes",
                ret.map(MutationReturn::as_sql).unwrap_or("")
            )));
        }
        let mut find_args = Map::new();
        find_args.insert("where".to_owned(), where_value.cloned().unwrap_or(Value::Null));
        let mut id_select = Map::new();
        id_select.insert("id".to_owned(), Value::Bool(true));
        find_args.insert("select".to_owned(), Value::Object(id_select));
        let matched = find_many_records::<Value>(db, table, &Value::Object(find_args)).await?;
        let ids = matched
            .iter()
            .map(|row| {
                row_id(row)
                    .ok_or_else(|| err(format!("updateMany {table}: matched row missing id")))
            })
            .collect::<Result<Vec<_>, _>>()?;
        let select = select.cloned();
        let data = data.clone();
        let table = table.to_owned();
        return with_write_transaction(db, true, |ops| {
            let ids = ids.clone();
            let select = select.clone();
            let data = data.clone();
            let table = table.clone();
            async move {
                if let Some(select) = select {
                    let mut rows = Vec::new();
                    let mut opts = Map::new();
                    opts.insert("select".to_owned(), select);
                    for id in &ids {
                        if let SingleResult::Row(Some(row)) = update_with_nested(
                            &ops,
                            &table,
                            id,
                            &data,
                            Some(&Value::Object(opts.clone())),
                        )
                        .await?
                        {
                            rows.push(row);
                        }
                    }
                    return Ok(ManyResult::Rows(rows));
                }
                if ret == Some(MutationReturn::After) {
                    let mut rows = Vec::new();
                    let mut opts = Map::new();
                    opts.insert("return".to_owned(), Value::String("AFTER".to_owned()));
                    for id in &ids {
                        if let SingleResult::Row(Some(row)) = update_with_nested(
                            &ops,
                            &table,
                            id,
                            &data,
                            Some(&Value::Object(opts.clone())),
                        )
                        .await?
                        {
                            rows.push(row);
                        }
                    }
                    return Ok(ManyResult::Rows(rows));
                }
                for id in &ids {
                    let opts = if ret == Some(MutationReturn::None) {
                        let mut opts = Map::new();
                        opts.insert("return".to_owned(), Value::String("NONE".to_owned()));
                        Some(Value::Object(opts))
                    } else {
                        None
                    };
                    update_with_nested(&ops, &table, id, &data, opts.as_ref()).await?;
                }
                Ok(ManyResult::Count { count: ids.len() as u64 })
            }
        })
        .await;
    }
    let mut vars = JsonMap::new();
    vars.insert("__data".to_owned(), data);
    let mut ctx = WhereCtx { vars: vars.clone(), n: 0 };
    let where_clause = build_where(where_value.unwrap_or(&Value::Null), table, &mut ctx)?;
    vars = ctx.vars;
    if where_clause.is_empty() {
        return Err(err(format!("updateMany {table}: where must not be empty")));
    }
    if let Some(select) = select {
        let projection = build_projection(Some(select), table)?;
        let rows = query_rows(
            db,
            &format!("UPDATE {table} MERGE $__data WHERE {where_clause} RETURN {projection}"),
            &vars,
        )
        .await?;
        return Ok(ManyResult::Rows(rows));
    }
    match ret {
        Some(MutationReturn::None) => {
            let count = count_matching(db, table, &where_clause, &vars).await?;
            query_rows(
                db,
                &format!("UPDATE {table} MERGE $__data WHERE {where_clause} RETURN NONE"),
                &vars,
            )
            .await?;
            Ok(ManyResult::Count { count })
        }
        Some(MutationReturn::Diff) => {
            let rows = query_rows(
                db,
                &format!("UPDATE {table} MERGE $__data WHERE {where_clause} RETURN DIFF"),
                &vars,
            )
            .await?;
            Ok(ManyResult::Rows(rows))
        }
        Some(MutationReturn::Before | MutationReturn::After) => {
            let mode = ret.map(MutationReturn::as_sql).unwrap_or("AFTER");
            let rows = query_rows(
                db,
                &format!("UPDATE {table} MERGE $__data WHERE {where_clause} RETURN {mode}"),
                &vars,
            )
            .await?;
            Ok(ManyResult::Rows(rows))
        }
        None => {
            let rows = query_rows(
                db,
                &format!("UPDATE {table} MERGE $__data WHERE {where_clause} RETURN AFTER"),
                &vars,
            )
            .await?;
            Ok(ManyResult::Count { count: rows.len() as u64 })
        }
    }
}

/// Delete every row matching `where`.
pub async fn delete_many_records(
    db: &DbHandle,
    table: &str,
    args: &Value,
) -> Result<ManyResult, ClientError> {
    let where_value = args.get("where");
    assert_non_empty_where(where_value, "deleteMany", table)?;
    let select = args.get("select");
    let ret = return_mode(args.get("return"))?;
    assert_return_select_exclusive("deleteMany", table, select, ret.map(MutationReturn::as_sql))?;
    if ret == Some(MutationReturn::After) {
        return Err(err(format!("deleteMany {table}: return AFTER is not supported (use BEFORE)")));
    }
    let mut vars = JsonMap::new();
    let mut ctx = WhereCtx { vars: vars.clone(), n: 0 };
    let where_clause = build_where(where_value.unwrap_or(&Value::Null), table, &mut ctx)?;
    vars = ctx.vars;
    if where_clause.is_empty() {
        return Err(err(format!("deleteMany {table}: where must not be empty")));
    }
    if let Some(select) = select {
        let rows =
            query_rows(db, &format!("DELETE {table} WHERE {where_clause} RETURN BEFORE"), &vars)
                .await?;
        return Ok(ManyResult::Rows(
            rows.iter().map(|row| project_row(row, Some(select))).collect(),
        ));
    }
    match ret {
        Some(MutationReturn::None) => {
            let count = count_matching(db, table, &where_clause, &vars).await?;
            query_rows(db, &format!("DELETE {table} WHERE {where_clause} RETURN NONE"), &vars)
                .await?;
            Ok(ManyResult::Count { count })
        }
        Some(MutationReturn::Diff) => {
            let rows =
                query_rows(db, &format!("DELETE {table} WHERE {where_clause} RETURN DIFF"), &vars)
                    .await?;
            Ok(ManyResult::Rows(rows))
        }
        Some(MutationReturn::Before) => {
            let rows = query_rows(
                db,
                &format!("DELETE {table} WHERE {where_clause} RETURN BEFORE"),
                &vars,
            )
            .await?;
            Ok(ManyResult::Rows(rows))
        }
        None => {
            let rows = query_rows(
                db,
                &format!("DELETE {table} WHERE {where_clause} RETURN BEFORE"),
                &vars,
            )
            .await?;
            Ok(ManyResult::Count { count: rows.len() as u64 })
        }
        Some(MutationReturn::After) => unreachable!("rejected above"),
    }
}

/// Update the unique match, or create it.
pub async fn upsert_record<F, Fut>(
    db: &DbHandle,
    table: &str,
    args: &Value,
    mut create_one: F,
) -> Result<SingleResult, ClientError>
where
    F: FnMut(JsonMap) -> Fut,
    Fut: Future<Output = Result<Value, ClientError>>,
{
    let where_value = args.get("where");
    assert_non_empty_where(where_value, "upsert", table)?;
    let select = args.get("select");
    let ret = return_mode(args.get("return"))?;
    assert_return_select_exclusive("upsert", table, select, ret.map(MutationReturn::as_sql))?;
    let existing = find_unique_record::<Value>(
        db,
        table,
        &Value::Object({
            let mut map = Map::new();
            map.insert("where".to_owned(), where_value.cloned().unwrap_or(Value::Null));
            map
        }),
    )
    .await?;
    if let Some(existing) = existing.as_ref().and_then(row_id) {
        let update = args.get("update").and_then(Value::as_object).cloned().unwrap_or_default();
        if ret == Some(MutationReturn::Before) {
            let before = find_unique_record::<Value>(
                db,
                table,
                &Value::Object({
                    let mut map = Map::new();
                    let mut where_id = Map::new();
                    where_id.insert("id".to_owned(), Value::String(existing.clone()));
                    map.insert("where".to_owned(), Value::Object(where_id));
                    map
                }),
            )
            .await?;
            let mut opts = Map::new();
            opts.insert("return".to_owned(), Value::String("NONE".to_owned()));
            update_one_record(db, table, &existing, &update, Some(&Value::Object(opts))).await?;
            return Ok(SingleResult::Row(before));
        }
        if ret == Some(MutationReturn::None) {
            let mut opts = Map::new();
            opts.insert("return".to_owned(), Value::String("NONE".to_owned()));
            update_one_record(db, table, &existing, &update, Some(&Value::Object(opts))).await?;
            return Ok(SingleResult::Empty);
        }
        if ret == Some(MutationReturn::Diff) {
            let mut opts = Map::new();
            opts.insert("return".to_owned(), Value::String("DIFF".to_owned()));
            return update_one_record(db, table, &existing, &update, Some(&Value::Object(opts)))
                .await;
        }
        if let Some(select) = select {
            let mut opts = Map::new();
            opts.insert("select".to_owned(), select.clone());
            return update_one_record(db, table, &existing, &update, Some(&Value::Object(opts)))
                .await;
        }
        let mut opts = Map::new();
        opts.insert("return".to_owned(), Value::String("AFTER".to_owned()));
        let updated =
            update_one_record(db, table, &existing, &update, Some(&Value::Object(opts))).await?;
        return match updated {
            SingleResult::Row(None) | SingleResult::Empty => {
                Err(err(format!("upsert {table}: update returned no row")))
            }
            other => Ok(other),
        };
    }
    let create = args.get("create").and_then(Value::as_object).cloned().unwrap_or_default();
    if ret == Some(MutationReturn::Before) {
        create_one(create).await?;
        return Ok(SingleResult::Row(None));
    }
    if ret == Some(MutationReturn::None) {
        create_one(create).await?;
        return Ok(SingleResult::Empty);
    }
    if ret == Some(MutationReturn::Diff) {
        let mut vars = JsonMap::new();
        vars.insert("__row".to_owned(), Value::Object(create));
        let rows =
            query_rows(db, &format!("CREATE {table} CONTENT $__row RETURN DIFF"), &vars).await?;
        return Ok(SingleResult::Diff(rows.into_iter().next().unwrap_or(Value::Null)));
    }
    let created = create_one(create).await?;
    if select.is_none() {
        return Ok(SingleResult::Row(Some(created)));
    }
    let Some(id) = row_id(&created) else {
        return Err(err(format!("upsert {table}: created row missing id")));
    };
    let mut reload = Map::new();
    let mut where_id = Map::new();
    where_id.insert("id".to_owned(), Value::String(id));
    reload.insert("where".to_owned(), Value::Object(where_id));
    if let Some(select) = select {
        reload.insert("select".to_owned(), select.clone());
    }
    let projected = find_unique_record::<Value>(db, table, &Value::Object(reload)).await?;
    projected
        .map(|row| SingleResult::Row(Some(row)))
        .ok_or_else(|| err(format!("upsert {table}: could not reload after create")))
}

pub fn decode<T: DeserializeOwned>(value: Value) -> Result<T, ClientError> {
    from_json(value)
}

pub(crate) fn from_json<T: DeserializeOwned>(value: Value) -> Result<T, ClientError> {
    serde_json::from_value(value).map_err(|error| err(error.to_string()))
}

/// Execute SQL and return statement rows. Used by `$queryRaw`.
pub async fn execute_query<T: DeserializeOwned>(
    db: &DbHandle,
    sql: &str,
    vars: Option<&JsonMap>,
) -> Result<Vec<T>, ClientError> {
    let empty = JsonMap::new();
    let rows = query_rows(db, sql, vars.unwrap_or(&empty)).await?;
    rows.into_iter().map(from_json).collect()
}

/// Execute SQL and return the raw statement envelope. Used by `$executeRaw`.
pub async fn execute_raw(
    db: &DbHandle,
    sql: &str,
    vars: Option<&JsonMap>,
) -> Result<Value, ClientError> {
    let empty = JsonMap::new();
    db.query(sql, vars.unwrap_or(&empty)).await
}

#[allow(dead_code)]
fn _keep_nested_symbol(value: &Value) -> bool {
    is_nested_write_bag(value)
}
