//! One-hop nested writes. SQL is rendered once, then executed by the caller.

use std::collections::BTreeMap;

use serde_json::Value;

use crate::error::{ClientError, err};
use crate::id::normalize_thing;
use crate::jsonutil::{JsonMap, as_rows, entries};
use crate::meta::{FieldWriteMeta, write_meta};
use crate::ops::DbHandle;

const NESTED_WRITE_KEYS: &[&str] =
    &["create", "connect", "disconnect", "set", "update", "delete", "upsert", "connectOrCreate"];

/// A nested create/connect/update bag.
#[must_use]
pub fn is_nested_write_bag(value: &Value) -> bool {
    let Some(map) = value.as_object() else {
        return false;
    };
    NESTED_WRITE_KEYS.iter().any(|key| map.contains_key(*key))
}

/// One statement in a nested-write script.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NestedOp {
    /// `INSERT INTO`.
    Insert {
        /// Table.
        table: String,
        /// Rows.
        rows: Vec<Value>,
    },
    /// `UPDATE ... MERGE`.
    Merge {
        /// Record ids.
        ids: Vec<String>,
        /// Patch.
        patch: JsonMap,
    },
    /// `UPDATE ... UNSET`.
    Unset {
        /// Record ids.
        ids: Vec<String>,
        /// Field name.
        field: String,
    },
    /// `INSERT RELATION`.
    Relate {
        /// Edge table.
        table: String,
        /// Edge rows.
        rows: Vec<Value>,
    },
    /// `DELETE` by id.
    DeleteIds {
        /// Record ids.
        ids: Vec<String>,
    },
    /// Delete edges by `in`/`out` pair.
    DeletePairs {
        /// Edge table.
        table: String,
        /// `{ from, to }` pairs.
        pairs: Vec<Value>,
    },
    /// Create the far record and relate it.
    RelateNewFar {
        /// Edge table.
        edge_table: String,
        /// Far table.
        far_table: String,
        /// Parent is `in` and the far record is `out`.
        far_is_out: bool,
        /// Far payload, edge content, and parent id.
        items: Vec<Value>,
    },
}

/// Bind `$nwN`.
pub fn bind_nested(vars: &mut JsonMap, value: Value) -> String {
    let name = format!("nw{}", vars.len());
    vars.insert(name.clone(), value);
    format!("${name}")
}

/// Render nested ops as one script.
#[must_use]
pub fn render_nested_ops(ops: &[NestedOp]) -> (String, JsonMap) {
    let mut vars = JsonMap::new();
    let mut parts = Vec::new();
    for op in ops {
        match op {
            NestedOp::Insert { table, rows } => {
                parts.push(format!(
                    "INSERT INTO {table} {} RETURN NONE",
                    bind_nested(&mut vars, Value::Array(rows.clone()))
                ));
            }
            NestedOp::Merge { ids, patch } => {
                parts.push(format!(
                    "UPDATE {} MERGE {}",
                    bind_nested(
                        &mut vars,
                        Value::Array(ids.iter().cloned().map(Value::String).collect())
                    ),
                    bind_nested(&mut vars, Value::Object(patch.clone()))
                ));
            }
            NestedOp::Unset { ids, field } => {
                parts.push(format!(
                    "UPDATE {} UNSET {field}",
                    bind_nested(
                        &mut vars,
                        Value::Array(ids.iter().cloned().map(Value::String).collect())
                    )
                ));
            }
            NestedOp::Relate { table, rows } => {
                parts.push(format!(
                    "INSERT RELATION INTO {table} {} RETURN NONE",
                    bind_nested(&mut vars, Value::Array(rows.clone()))
                ));
            }
            NestedOp::DeleteIds { ids } => {
                parts.push(format!(
                    "DELETE {}",
                    bind_nested(
                        &mut vars,
                        Value::Array(ids.iter().cloned().map(Value::String).collect())
                    )
                ));
            }
            NestedOp::DeletePairs { table, pairs } => {
                parts.push(format!(
                    "FOR $pair IN {} {{ DELETE {table} WHERE in = type::record($pair.from) AND out = type::record($pair.to) }}",
                    bind_nested(&mut vars, Value::Array(pairs.clone()))
                ));
            }
            NestedOp::RelateNewFar { edge_table, far_table, far_is_out, items } => {
                let relate = if *far_is_out {
                    format!("$as_parent->{edge_table}->$far")
                } else {
                    format!("$far->{edge_table}->$as_parent")
                };
                parts.push(format!(
                    "FOR $item IN {} {{ LET $far = CREATE ONLY {far_table} CONTENT $item.far; LET $as_parent = type::record($item.as_parent); RELATE {relate} CONTENT $item.content }}",
                    bind_nested(&mut vars, Value::Array(items.clone()))
                ));
            }
        }
    }
    (parts.join(";\n"), vars)
}

pub(crate) async fn run_nested_ops(db: &DbHandle, ops: &[NestedOp]) -> Result<(), ClientError> {
    if ops.is_empty() {
        return Ok(());
    }
    let (sql, vars) = render_nested_ops(ops);
    query_script(db, &sql, &vars).await.map(|_| ())
}

pub(crate) async fn query_script(
    db: &DbHandle,
    sql: &str,
    vars: &JsonMap,
) -> Result<Vec<Value>, ClientError> {
    let result = db.query(sql, vars).await?;
    Ok(match result {
        Value::Array(items) => items,
        other => vec![other],
    })
}

pub(crate) fn statement_rows(statement: &Value) -> Vec<Value> {
    match statement {
        Value::Array(items) => items.clone(),
        Value::Null => Vec::new(),
        other => vec![other.clone()],
    }
}

async fn flush_ops(db: &DbHandle, ops: &mut Vec<NestedOp>) -> Result<(), ClientError> {
    if ops.is_empty() {
        return Ok(());
    }
    run_nested_ops(db, ops).await?;
    ops.clear();
    Ok(())
}

fn as_record(value: &Value) -> Option<&JsonMap> {
    value.as_object()
}

fn equality_where(
    where_value: &Value,
    unique_fields: &[String],
    label: &str,
) -> Result<(String, String), ClientError> {
    let Some(record) = as_record(where_value) else {
        return Err(err(format!("{label}: where accepts exactly one equality key")));
    };
    let keys = record.keys().map(String::as_str).collect::<Vec<_>>();
    if keys.len() != 1 {
        return Err(err(format!("{label}: where accepts exactly one equality key")));
    }
    let field = keys[0];
    if field != "id" && !unique_fields.iter().any(|unique| unique == field) {
        return Err(err(format!("{label}: where field {field} is not unique")));
    }
    let Some(value) = record.get(field).and_then(Value::as_str) else {
        return Err(err(format!("{label}: where.{field} must be a string")));
    };
    Ok((field.to_owned(), value.to_owned()))
}

fn record_eq(field: &str, var_name: &str) -> String {
    format!("{field} = type::record(${var_name})")
}

async fn query_rows(db: &DbHandle, sql: &str, vars: &JsonMap) -> Result<Vec<Value>, ClientError> {
    Ok(as_rows(db.query(sql, vars).await?))
}

async fn find_id_by_where(
    db: &DbHandle,
    table: &str,
    where_value: &Value,
    unique_fields: &[String],
    label: &str,
) -> Result<Option<String>, ClientError> {
    let (field, value) = equality_where(where_value, unique_fields, label)?;
    let bound = if field == "id" { normalize_thing(table, &value) } else { value };
    let predicate = if field == "id" { record_eq("id", "v") } else { format!("{field} = $v") };
    let mut vars = JsonMap::new();
    vars.insert("v".to_owned(), Value::String(bound));
    let rows =
        query_rows(db, &format!("SELECT id FROM {table} WHERE {predicate} LIMIT 1"), &vars).await?;
    Ok(rows.first().and_then(crate::jsonutil::row_id))
}

async fn select_column(
    db: &DbHandle,
    table: &str,
    id: &str,
    field: &str,
) -> Result<Option<Value>, ClientError> {
    let mut vars = JsonMap::new();
    vars.insert("thing".to_owned(), Value::String(normalize_thing(table, id)));
    let rows = query_rows(db, &format!("SELECT {field} FROM type::record($thing)"), &vars).await?;
    Ok(rows.first().and_then(|row| row.get(field)).cloned())
}

async fn select_ids_where(
    db: &DbHandle,
    table: &str,
    field: &str,
    value: &str,
) -> Result<Vec<String>, ClientError> {
    let mut vars = JsonMap::new();
    vars.insert("v".to_owned(), Value::String(value.to_owned()));
    let rows =
        query_rows(db, &format!("SELECT id FROM {table} WHERE {}", record_eq(field, "v")), &vars)
            .await?;
    Ok(rows.iter().filter_map(crate::jsonutil::row_id).collect())
}

async fn merge_record(
    db: &DbHandle,
    table: &str,
    id: &str,
    patch: &JsonMap,
) -> Result<(), ClientError> {
    let mut vars = JsonMap::new();
    vars.insert("thing".to_owned(), Value::String(normalize_thing(table, id)));
    vars.insert("patch".to_owned(), Value::Object(patch.clone()));
    query_rows(db, "UPDATE type::record($thing) MERGE $patch RETURN NONE", &vars).await?;
    Ok(())
}

/// How a stored link write resolves before the parent row is written.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StoredNestedResolved {
    /// Scalar value to store, when not creating.
    pub value: Option<Value>,
    /// Related row to insert.
    pub create: Option<JsonMap>,
    /// Skip the parent column (already updated in place).
    pub skip: bool,
    /// Related record to delete after the parent link is cleared.
    pub delete_after: Option<String>,
}

/// Interpret a stored-link value that is not a full nested bag.
pub fn read_stored_link(
    table: &str,
    field: &str,
    value: &Value,
    meta: &FieldWriteMeta,
) -> Result<StoredNestedResolved, ClientError> {
    let FieldWriteMeta::Stored { optional, target_table, .. } = meta else {
        return Ok(StoredNestedResolved {
            value: Some(value.clone()),
            create: None,
            skip: false,
            delete_after: None,
        });
    };
    let Some(bag) = value.as_object() else {
        return Ok(StoredNestedResolved {
            value: Some(value.clone()),
            create: None,
            skip: false,
            delete_after: None,
        });
    };
    if bag.contains_key("disconnect") {
        if !optional {
            return Err(err(format!("update {table}: cannot disconnect required link {field}")));
        }
        return Ok(StoredNestedResolved {
            value: Some(Value::Null),
            create: None,
            skip: false,
            delete_after: None,
        });
    }
    if bag.contains_key("connect") {
        let id = bag
            .get("connect")
            .and_then(Value::as_object)
            .and_then(|map| map.get("id"))
            .and_then(Value::as_str);
        let Some(id) = id else {
            return Err(err(format!("update {table}: {field}.connect.id required")));
        };
        return Ok(StoredNestedResolved {
            value: Some(Value::String(normalize_thing(target_table, id))),
            create: None,
            skip: false,
            delete_after: None,
        });
    }
    if bag.contains_key("create") {
        let create = bag.get("create").and_then(Value::as_object).cloned().unwrap_or_default();
        return Ok(StoredNestedResolved {
            value: None,
            create: Some(create),
            skip: false,
            delete_after: None,
        });
    }
    Ok(StoredNestedResolved {
        value: Some(value.clone()),
        create: None,
        skip: false,
        delete_after: None,
    })
}

fn stored_update_patch(
    update: &Value,
    label: &str,
) -> Result<(Option<Value>, JsonMap), ClientError> {
    let Some(record) = update.as_object() else {
        return Err(err(format!("{label}.update data required")));
    };
    if record.contains_key("where") && record.contains_key("data") {
        let Some(data) = record.get("data").and_then(Value::as_object).cloned() else {
            return Err(err(format!("{label}.update data required")));
        };
        return Ok((record.get("where").cloned(), data));
    }
    Ok((None, record.clone()))
}

/// Reject nested writes inside an already-nested payload.
pub fn assert_no_nested_writes(
    data: &JsonMap,
    table: &str,
    context: &str,
) -> Result<(), ClientError> {
    let meta = write_meta();
    let table_meta = meta.get(table);
    for (key, value) in entries(data) {
        let Some(field_meta) = table_meta.and_then(|fields| fields.get(key)) else {
            continue;
        };
        let nested = match field_meta {
            FieldWriteMeta::Stored { .. } => is_nested_write_bag(value),
            FieldWriteMeta::Computed { .. } | FieldWriteMeta::Edge { .. } => !value.is_null(),
            FieldWriteMeta::Scalar => false,
        };
        if nested {
            return Err(err(format!("{context}: nested writes are one hop only ({table}.{key})")));
        }
    }
    Ok(())
}

async fn resolve_stored_nested(
    db: &DbHandle,
    table: &str,
    field: &str,
    bag: &JsonMap,
    meta: &FieldWriteMeta,
    parent_id: Option<&str>,
) -> Result<StoredNestedResolved, ClientError> {
    let FieldWriteMeta::Stored { optional, target_table, unique_fields, .. } = meta else {
        return Err(err(format!("{table}.{field}: expected a stored link")));
    };
    let label = format!("{table}.{field}");
    if bag.contains_key("delete") {
        if !optional {
            return Err(err(format!("update {table}: cannot delete required link {field}")));
        }
        let Some(parent_id) = parent_id else {
            return Err(err(format!("{label}.delete requires an existing parent")));
        };
        let current = select_column(db, table, parent_id, field).await?;
        if let Some(current) = current
            && !current.is_null()
            && current.as_str() != Some("")
        {
            return Ok(StoredNestedResolved {
                value: Some(Value::Null),
                create: None,
                skip: false,
                delete_after: Some(normalize_thing(target_table, &value_to_string(&current))),
            });
        }
        return Ok(StoredNestedResolved {
            value: Some(Value::Null),
            create: None,
            skip: false,
            delete_after: None,
        });
    }
    if bag.contains_key("update") {
        let (where_value, data) =
            stored_update_patch(bag.get("update").unwrap_or(&Value::Null), &label)?;
        let target_id = if let Some(where_value) = where_value {
            let id =
                find_id_by_where(db, target_table, &where_value, unique_fields, &label).await?;
            id.ok_or_else(|| err(format!("{label}.update where matched no record")))?
        } else {
            let Some(parent_id) = parent_id else {
                return Err(err(format!("{label}.update requires an existing parent")));
            };
            let current = select_column(db, table, parent_id, field).await?;
            match current {
                Some(current) if !current.is_null() && current.as_str() != Some("") => {
                    value_to_string(&current)
                }
                _ => return Err(err(format!("{label}.update has no linked record"))),
            }
        };
        assert_no_nested_writes(&data, target_table, &format!("update {label}"))?;
        merge_record(db, target_table, &target_id, &data).await?;
        return Ok(StoredNestedResolved {
            value: None,
            create: None,
            skip: true,
            delete_after: None,
        });
    }
    if bag.contains_key("connectOrCreate") {
        let Some(spec) = bag.get("connectOrCreate").and_then(Value::as_object) else {
            return Err(err(format!("{label}.connectOrCreate required")));
        };
        let Some(create) = spec.get("create").and_then(Value::as_object).cloned() else {
            return Err(err(format!("{label}.connectOrCreate.create required")));
        };
        assert_no_nested_writes(&create, target_table, &format!("create {label}"))?;
        let where_value = spec.get("where").cloned().unwrap_or(Value::Null);
        if let Some(existing) =
            find_id_by_where(db, target_table, &where_value, unique_fields, &label).await?
        {
            return Ok(StoredNestedResolved {
                value: Some(Value::String(existing)),
                create: None,
                skip: false,
                delete_after: None,
            });
        }
        return Ok(StoredNestedResolved {
            value: None,
            create: Some(create),
            skip: false,
            delete_after: None,
        });
    }
    if bag.contains_key("upsert") {
        let Some(spec) = bag.get("upsert").and_then(Value::as_object) else {
            return Err(err(format!("{label}.upsert required")));
        };
        let Some(update) = spec.get("update").and_then(Value::as_object).cloned() else {
            return Err(err(format!("{label}.upsert.update required")));
        };
        let Some(create) = spec.get("create").and_then(Value::as_object).cloned() else {
            return Err(err(format!("{label}.upsert.create required")));
        };
        assert_no_nested_writes(&update, target_table, &format!("update {label}"))?;
        assert_no_nested_writes(&create, target_table, &format!("create {label}"))?;
        let where_value = spec.get("where").cloned().unwrap_or(Value::Null);
        if let Some(existing) =
            find_id_by_where(db, target_table, &where_value, unique_fields, &label).await?
        {
            merge_record(db, target_table, &existing, &update).await?;
            return Ok(StoredNestedResolved {
                value: Some(Value::String(existing)),
                create: None,
                skip: false,
                delete_after: None,
            });
        }
        return Ok(StoredNestedResolved {
            value: None,
            create: Some(create),
            skip: false,
            delete_after: None,
        });
    }
    read_stored_link(table, field, &Value::Object(bag.clone()), meta)
}

fn value_to_string(value: &Value) -> String {
    match value {
        Value::String(text) => text.clone(),
        other => other.to_string(),
    }
}

struct StoredCreate {
    key: String,
    table: String,
    data: JsonMap,
}

/// Split a write payload into scalar columns, nested bags, and deferred deletes.
pub async fn split_and_resolve_write_data(
    db: &DbHandle,
    table: &str,
    data: &JsonMap,
    parent_id: Option<&str>,
) -> Result<(JsonMap, JsonMap, Vec<String>), ClientError> {
    let meta = write_meta();
    let table_meta = meta.get(table);
    let mut scalars = JsonMap::new();
    let mut nested = JsonMap::new();
    let mut delete_after = Vec::new();
    let mut stored_creates = Vec::new();
    for (key, value) in entries(data) {
        let field_meta = table_meta.and_then(|fields| fields.get(key));
        match field_meta {
            None | Some(FieldWriteMeta::Scalar) => {
                scalars.insert(key.to_owned(), value.clone());
            }
            Some(stored @ FieldWriteMeta::Stored { .. }) => {
                let resolved = if is_nested_write_bag(value) {
                    let bag = value.as_object().cloned().unwrap_or_default();
                    resolve_stored_nested(db, table, key, &bag, stored, parent_id).await?
                } else {
                    read_stored_link(table, key, value, stored)?
                };
                if resolved.skip {
                    continue;
                }
                if let Some(id) = resolved.delete_after {
                    delete_after.push(id);
                }
                if let Some(create) = resolved.create {
                    let FieldWriteMeta::Stored { target_table, .. } = stored else { continue };
                    assert_no_nested_writes(
                        &create,
                        target_table,
                        &format!("create {table}.{key}"),
                    )?;
                    stored_creates.push(StoredCreate {
                        key: key.to_owned(),
                        table: target_table.clone(),
                        data: create,
                    });
                } else if let Some(value) = resolved.value {
                    scalars.insert(key.to_owned(), value);
                }
            }
            Some(FieldWriteMeta::Computed { .. } | FieldWriteMeta::Edge { .. }) => {
                if !is_nested_write_bag(value) {
                    return Err(err(format!("{table}.{key}: expected a nested write")));
                }
                nested.insert(key.to_owned(), value.clone());
            }
        }
    }
    if !stored_creates.is_empty() {
        let ids = insert_stored_creates(db, &stored_creates).await?;
        for create in &stored_creates {
            if let Some(id) = ids.get(&create.key) {
                scalars.insert(create.key.clone(), Value::String(id.clone()));
            }
        }
    }
    Ok((scalars, nested, delete_after))
}

async fn insert_stored_creates(
    db: &DbHandle,
    creates: &[StoredCreate],
) -> Result<BTreeMap<String, String>, ClientError> {
    struct Group {
        table: String,
        keys: Vec<String>,
        rows: Vec<Value>,
    }
    let mut groups: Vec<Group> = Vec::new();
    for create in creates {
        if let Some(last) = groups.last_mut()
            && last.table == create.table
        {
            last.keys.push(create.key.clone());
            last.rows.push(Value::Object(create.data.clone()));
            continue;
        }
        groups.push(Group {
            table: create.table.clone(),
            keys: vec![create.key.clone()],
            rows: vec![Value::Object(create.data.clone())],
        });
    }
    let mut vars = JsonMap::new();
    let sql = groups
        .iter()
        .enumerate()
        .map(|(index, group)| {
            vars.insert(format!("sc{index}"), Value::Array(group.rows.clone()));
            format!("INSERT INTO {} $sc{index} RETURN AFTER", group.table)
        })
        .collect::<Vec<_>>()
        .join(";\n");
    let statements = query_script(db, &sql, &vars).await?;
    let mut ids = BTreeMap::new();
    for (index, group) in groups.iter().enumerate() {
        let rows = statement_rows(statements.get(index).unwrap_or(&Value::Null));
        for (row_index, key) in group.keys.iter().enumerate() {
            let id = rows.get(row_index).and_then(crate::jsonutil::row_id);
            let Some(id) = id else {
                return Err(err(format!("create {} returned no row", group.table)));
            };
            ids.insert(key.clone(), id);
        }
    }
    Ok(ids)
}

pub(crate) async fn run_deferred_deletes(db: &DbHandle, ids: &[String]) -> Result<(), ClientError> {
    for id in ids {
        let mut vars = JsonMap::new();
        vars.insert("thing".to_owned(), Value::String(id.clone()));
        query_rows(db, "DELETE type::record($thing) RETURN NONE", &vars).await?;
    }
    Ok(())
}

/// Surreal `option<record>` rejects JSON null. Clear those links with `UNSET`.
#[must_use]
pub fn partition_null_link_clears(table: &str, data: &JsonMap) -> (JsonMap, Vec<String>) {
    let meta = write_meta();
    let table_meta = meta.get(table);
    let mut patch = JsonMap::new();
    let mut unset_fields = Vec::new();
    for (key, value) in entries(data) {
        let field_meta = table_meta.and_then(|fields| fields.get(key));
        if value.is_null()
            && let Some(FieldWriteMeta::Stored { optional: true, .. }) = field_meta
        {
            unset_fields.push(key.to_owned());
            continue;
        }
        patch.insert(key.to_owned(), value.clone());
    }
    (patch, unset_fields)
}

pub(crate) async fn unset_record_fields(
    db: &DbHandle,
    table: &str,
    id: &str,
    fields: &[String],
) -> Result<(), ClientError> {
    if fields.is_empty() {
        return Ok(());
    }
    let mut vars = JsonMap::new();
    vars.insert("thing".to_owned(), Value::String(normalize_thing(table, id)));
    query_rows(
        db,
        &format!("UPDATE type::record($thing) UNSET {} RETURN NONE", fields.join(", ")),
        &vars,
    )
    .await?;
    Ok(())
}

fn required_thing(item: &Value, table: &str, label: &str) -> Result<String, ClientError> {
    let Some(id) = item.as_object().and_then(|map| map.get("id")).and_then(Value::as_str) else {
        return Err(err(format!("{label}.id required")));
    };
    Ok(normalize_thing(table, id))
}

/// Apply computed-link nested writes onto `ops`.
pub async fn collect_computed(
    db: &DbHandle,
    ops: &mut Vec<NestedOp>,
    parent_table: &str,
    parent_id: &str,
    field: &str,
    bag: &JsonMap,
    meta: &FieldWriteMeta,
) -> Result<(), ClientError> {
    let FieldWriteMeta::Computed {
        target_table,
        back_link_field,
        back_link_optional,
        unique_fields,
        ..
    } = meta
    else {
        return Err(err("collect_computed requires computed meta"));
    };
    let parent_thing = normalize_thing(parent_table, parent_id);
    let label = format!("{parent_table}.{field}");

    if let Some(Value::Array(items)) = bag.get("set") {
        let next = items
            .iter()
            .map(|item| required_thing(item, target_table, &format!("{label}.set")))
            .collect::<Result<Vec<_>, _>>()?;
        let remove = select_ids_where(db, target_table, back_link_field, &parent_thing)
            .await?
            .into_iter()
            .map(|id| normalize_thing(target_table, &id))
            .filter(|id| !next.iter().any(|next_id| next_id == id))
            .collect::<Vec<_>>();
        if !remove.is_empty() && !back_link_optional {
            return Err(err(format!(
                "{label}: cannot disconnect required back-link {back_link_field}"
            )));
        }
        if !remove.is_empty() {
            ops.push(NestedOp::Unset { ids: remove, field: back_link_field.clone() });
        }
        if !next.is_empty() {
            let mut patch = JsonMap::new();
            patch.insert(back_link_field.clone(), Value::String(parent_thing.clone()));
            ops.push(NestedOp::Merge { ids: next, patch });
        }
    }

    if let Some(Value::Array(items)) = bag.get("disconnect") {
        let mut ids = Vec::new();
        for item in items {
            let Some(id) = item.as_object().and_then(|map| map.get("id")).and_then(Value::as_str)
            else {
                return Err(err(format!("{label}.disconnect.id required")));
            };
            if !back_link_optional {
                return Err(err(format!(
                    "{label}: cannot disconnect required back-link {back_link_field}"
                )));
            }
            ids.push(normalize_thing(target_table, id));
        }
        if !ids.is_empty() {
            ops.push(NestedOp::Unset { ids, field: back_link_field.clone() });
        }
    }

    if let Some(Value::Array(items)) = bag.get("delete") {
        let ids = items
            .iter()
            .map(|item| required_thing(item, target_table, &format!("{label}.delete")))
            .collect::<Result<Vec<_>, _>>()?;
        if !ids.is_empty() {
            ops.push(NestedOp::DeleteIds { ids });
        }
    }

    if let Some(Value::Array(items)) = bag.get("connect") {
        let mut ids = Vec::new();
        for item in items {
            let Some(id) = item.as_object().and_then(|map| map.get("id")).and_then(Value::as_str)
            else {
                return Err(err(format!("{label}.connect.id required")));
            };
            ids.push(normalize_thing(target_table, id));
        }
        if !ids.is_empty() {
            let mut patch = JsonMap::new();
            patch.insert(back_link_field.clone(), Value::String(parent_thing.clone()));
            ops.push(NestedOp::Merge { ids, patch });
        }
    }
    flush_ops(db, ops).await?;

    if let Some(Value::Array(items)) = bag.get("connectOrCreate") {
        let mut link = Vec::new();
        let mut rows = Vec::new();
        for item in items {
            let Some(spec) = item.as_object() else {
                return Err(err(format!("{label}.connectOrCreate required")));
            };
            let Some(create) = spec.get("create").and_then(Value::as_object).cloned() else {
                return Err(err(format!("{label}.connectOrCreate.create required")));
            };
            let where_value = spec.get("where").cloned().unwrap_or(Value::Null);
            if let Some(existing) =
                find_id_by_where(db, target_table, &where_value, unique_fields, &label).await?
            {
                link.push(normalize_thing(target_table, &existing));
            } else {
                let mut data = create;
                data.insert(back_link_field.clone(), Value::String(parent_thing.clone()));
                assert_no_nested_writes(&data, target_table, &format!("create {label}"))?;
                rows.push(Value::Object(data));
            }
        }
        if !link.is_empty() {
            let mut patch = JsonMap::new();
            patch.insert(back_link_field.clone(), Value::String(parent_thing.clone()));
            ops.push(NestedOp::Merge { ids: link, patch });
        }
        if !rows.is_empty() {
            ops.push(NestedOp::Insert { table: target_table.clone(), rows });
        }
    }

    if let Some(Value::Array(items)) = bag.get("create") {
        let mut rows = Vec::new();
        for item in items {
            let Some(record) = item.as_object() else { continue };
            let mut data = record.clone();
            data.insert(back_link_field.clone(), Value::String(parent_thing.clone()));
            assert_no_nested_writes(&data, target_table, &format!("create {label}"))?;
            rows.push(Value::Object(data));
        }
        if !rows.is_empty() {
            ops.push(NestedOp::Insert { table: target_table.clone(), rows });
        }
    }
    flush_ops(db, ops).await?;

    if let Some(Value::Array(items)) = bag.get("update") {
        for item in items {
            let Some(spec) = item.as_object() else {
                return Err(err(format!("{label}.update required")));
            };
            let Some(data) = spec.get("data").and_then(Value::as_object).cloned() else {
                return Err(err(format!("{label}.update data required")));
            };
            let where_value = spec.get("where").cloned().unwrap_or(Value::Null);
            let Some(id) =
                find_id_by_where(db, target_table, &where_value, unique_fields, &label).await?
            else {
                return Err(err(format!("{label}.update where matched no record")));
            };
            assert_no_nested_writes(&data, target_table, &format!("update {label}"))?;
            ops.push(NestedOp::Merge {
                ids: vec![normalize_thing(target_table, &id)],
                patch: data,
            });
        }
    }
    flush_ops(db, ops).await?;

    if let Some(Value::Array(items)) = bag.get("upsert") {
        let mut created = Vec::new();
        for item in items {
            let Some(spec) = item.as_object() else {
                return Err(err(format!("{label}.upsert required")));
            };
            let Some(update) = spec.get("update").and_then(Value::as_object).cloned() else {
                return Err(err(format!("{label}.upsert.update required")));
            };
            let Some(create) = spec.get("create").and_then(Value::as_object).cloned() else {
                return Err(err(format!("{label}.upsert.create required")));
            };
            let where_value = spec.get("where").cloned().unwrap_or(Value::Null);
            if let Some(existing) =
                find_id_by_where(db, target_table, &where_value, unique_fields, &label).await?
            {
                assert_no_nested_writes(&update, target_table, &format!("update {label}"))?;
                ops.push(NestedOp::Merge {
                    ids: vec![normalize_thing(target_table, &existing)],
                    patch: update,
                });
            } else {
                let mut data = create;
                data.insert(back_link_field.clone(), Value::String(parent_thing.clone()));
                assert_no_nested_writes(&data, target_table, &format!("create {label}"))?;
                created.push(Value::Object(data));
            }
        }
        if !created.is_empty() {
            ops.push(NestedOp::Insert { table: target_table.clone(), rows: created });
        }
    }
    flush_ops(db, ops).await?;
    Ok(())
}

/// Edge payload columns copied from a write row.
#[must_use]
pub fn edge_content(row: &JsonMap, payload_fields: &[String]) -> JsonMap {
    let mut content = JsonMap::new();
    for key in payload_fields {
        if let Some(value) = row.get(key) {
            content.insert(key.clone(), value.clone());
        }
    }
    content
}

/// Edge row with `in` and `out` filled from the parent direction.
#[must_use]
pub fn relation_row(
    meta: &FieldWriteMeta,
    parent_id: &str,
    far_id: &str,
    content: JsonMap,
) -> JsonMap {
    let FieldWriteMeta::Edge { dir, in_table, out_table, .. } = meta else {
        return content;
    };
    let (in_id, out_id) = match dir {
        crate::meta::RelationDir::Out => (parent_id, far_id),
        crate::meta::RelationDir::In => (far_id, parent_id),
    };
    let mut row = content;
    row.insert("in".to_owned(), Value::String(normalize_thing(in_table, in_id)));
    row.insert("out".to_owned(), Value::String(normalize_thing(out_table, out_id)));
    row
}

/// Copy the parent id into a stored link on a newly created far record.
#[must_use]
pub fn edge_far_data(
    parent_table: &str,
    parent_id: &str,
    far_table: &str,
    create_data: JsonMap,
) -> JsonMap {
    let meta = write_meta();
    let mut data = create_data;
    if let Some(parent_meta) = meta.get(far_table) {
        for (key, field_meta) in parent_meta {
            if let FieldWriteMeta::Stored { target_table, .. } = field_meta
                && target_table == parent_table
                && data.get(key).is_none_or(Value::is_null)
            {
                data.insert(key.clone(), Value::String(normalize_thing(parent_table, parent_id)));
            }
        }
    }
    data
}

fn edge_parent_field(meta: &FieldWriteMeta) -> &'static str {
    match meta {
        FieldWriteMeta::Edge { dir: crate::meta::RelationDir::Out, .. } => "in",
        _ => "out",
    }
}

fn edge_connect_rows(
    meta: &FieldWriteMeta,
    parent_id: &str,
    items: &[Value],
    label: &str,
) -> Result<Vec<Value>, ClientError> {
    let FieldWriteMeta::Edge { dir, payload_fields, .. } = meta else {
        return Ok(Vec::new());
    };
    let far_key = dir.as_str();
    let mut rows = Vec::new();
    for item in items {
        let Some(row) = item.as_object() else { continue };
        let Some(far_id) = row.get(far_key).and_then(Value::as_str) else {
            return Err(err(format!("{label}.{far_key} required")));
        };
        rows.push(Value::Object(relation_row(
            meta,
            parent_id,
            far_id,
            edge_content(row, payload_fields),
        )));
    }
    Ok(rows)
}

fn queue_edge_removal(
    ops: &mut Vec<NestedOp>,
    meta: &FieldWriteMeta,
    parent_table: &str,
    parent_id: &str,
    field: &str,
    items: &[Value],
    op: &str,
) -> Result<(), ClientError> {
    let FieldWriteMeta::Edge { dir, edge_table, .. } = meta else {
        return Ok(());
    };
    let far_key = dir.as_str();
    let mut delete_ids = Vec::new();
    let mut delete_pairs = Vec::new();
    let flush_ids = |ops: &mut Vec<NestedOp>, delete_ids: &mut Vec<String>| {
        if delete_ids.is_empty() {
            return;
        }
        ops.push(NestedOp::DeleteIds { ids: std::mem::take(delete_ids) });
    };
    let flush_pairs = |ops: &mut Vec<NestedOp>, delete_pairs: &mut Vec<Value>, edge_table: &str| {
        if delete_pairs.is_empty() {
            return;
        }
        ops.push(NestedOp::DeletePairs {
            table: edge_table.to_owned(),
            pairs: std::mem::take(delete_pairs),
        });
    };
    for item in items {
        let Some(row) = item.as_object() else { continue };
        if let Some(id) = row.get("id").and_then(Value::as_str) {
            flush_pairs(ops, &mut delete_pairs, edge_table);
            delete_ids.push(normalize_thing(edge_table, id));
            continue;
        }
        let Some(far_id) = row.get(far_key).and_then(Value::as_str) else {
            return Err(err(format!("{parent_table}.{field}.{op}.{far_key} required")));
        };
        flush_ids(ops, &mut delete_ids);
        let ends = relation_row(meta, parent_id, far_id, JsonMap::new());
        let mut pair = JsonMap::new();
        pair.insert("from".to_owned(), ends.get("in").cloned().unwrap_or(Value::Null));
        pair.insert("to".to_owned(), ends.get("out").cloned().unwrap_or(Value::Null));
        delete_pairs.push(Value::Object(pair));
    }
    flush_ids(ops, &mut delete_ids);
    flush_pairs(ops, &mut delete_pairs, edge_table);
    Ok(())
}

async fn find_edge_id(
    db: &DbHandle,
    meta: &FieldWriteMeta,
    parent_thing: &str,
    where_value: &Value,
    label: &str,
) -> Result<Option<String>, ClientError> {
    let FieldWriteMeta::Edge { dir, edge_table, in_table, out_table, .. } = meta else {
        return Ok(None);
    };
    let far_key = dir.as_str().to_owned();
    let (field, value) = equality_where(where_value, std::slice::from_ref(&far_key), label)?;
    let parent_field = edge_parent_field(meta);
    if field == "id" {
        let mut vars = JsonMap::new();
        vars.insert("v".to_owned(), Value::String(normalize_thing(edge_table, &value)));
        vars.insert("as_parent".to_owned(), Value::String(parent_thing.to_owned()));
        let sql = format!(
            "SELECT id FROM {edge_table} WHERE {} AND {} LIMIT 1",
            record_eq("id", "v"),
            record_eq(parent_field, "as_parent")
        );
        let rows = query_rows(db, &sql, &vars).await?;
        return Ok(rows.first().and_then(crate::jsonutil::row_id));
    }
    let far_table = if field == "out" { out_table } else { in_table };
    let mut vars = JsonMap::new();
    vars.insert("as_parent".to_owned(), Value::String(parent_thing.to_owned()));
    vars.insert("far".to_owned(), Value::String(normalize_thing(far_table, &value)));
    let sql = format!(
        "SELECT id FROM {edge_table} WHERE {} AND {} LIMIT 1",
        record_eq(parent_field, "as_parent"),
        record_eq(&field, "far")
    );
    let rows = query_rows(db, &sql, &vars).await?;
    Ok(rows.first().and_then(crate::jsonutil::row_id))
}

/// Apply edge nested writes onto `ops`.
pub async fn collect_edge(
    db: &DbHandle,
    ops: &mut Vec<NestedOp>,
    parent_table: &str,
    parent_id: &str,
    field: &str,
    bag: &JsonMap,
    meta: &FieldWriteMeta,
) -> Result<(), ClientError> {
    let FieldWriteMeta::Edge { dir, edge_table, in_table, out_table, payload_fields, .. } = meta
    else {
        return Err(err("collect_edge requires edge meta"));
    };
    let far_key = dir.as_str();
    let far_table = if matches!(dir, crate::meta::RelationDir::Out) {
        out_table.clone()
    } else {
        in_table.clone()
    };
    let parent_thing = normalize_thing(parent_table, parent_id);
    let label = format!("{parent_table}.{field}");

    if let Some(Value::Array(items)) = bag.get("set") {
        let current = select_ids_where(db, edge_table, edge_parent_field(meta), &parent_thing)
            .await?
            .into_iter()
            .map(|id| normalize_thing(edge_table, &id))
            .collect::<Vec<_>>();
        if !current.is_empty() {
            ops.push(NestedOp::DeleteIds { ids: current });
        }
        let rows = edge_connect_rows(meta, parent_id, items, &format!("{label}.set"))?;
        if !rows.is_empty() {
            ops.push(NestedOp::Relate { table: edge_table.clone(), rows });
        }
    }
    if let Some(Value::Array(items)) = bag.get("disconnect") {
        queue_edge_removal(ops, meta, parent_table, parent_id, field, items, "disconnect")?;
    }
    if let Some(Value::Array(items)) = bag.get("delete") {
        queue_edge_removal(ops, meta, parent_table, parent_id, field, items, "delete")?;
    }

    if let Some(Value::Array(items)) = bag.get("connect") {
        let mut connect_rows = Vec::new();
        for item in items {
            let Some(row) = item.as_object() else { continue };
            if row.get("id").and_then(Value::as_str).is_some() {
                return Err(err(format!(
                    "{parent_table}.{field}.connect by edge id is not supported for connect (use create or out/in)"
                )));
            }
            let Some(far_id) = row.get(far_key).and_then(Value::as_str) else {
                return Err(err(format!("{parent_table}.{field}.connect.{far_key} required")));
            };
            let mut where_value = JsonMap::new();
            where_value.insert(far_key.to_owned(), Value::String(far_id.to_owned()));
            let existing =
                find_edge_id(db, meta, &parent_thing, &Value::Object(where_value), &label).await?;
            let payload = edge_content(row, payload_fields);
            if let Some(existing) = existing {
                if !payload.is_empty() {
                    ops.push(NestedOp::Merge {
                        ids: vec![normalize_thing(edge_table, &existing)],
                        patch: payload,
                    });
                }
                continue;
            }
            connect_rows.push(Value::Object(relation_row(meta, parent_id, far_id, payload)));
        }
        if !connect_rows.is_empty() {
            ops.push(NestedOp::Relate { table: edge_table.clone(), rows: connect_rows });
        }
    }
    flush_ops(db, ops).await?;

    if let Some(Value::Array(items)) = bag.get("connectOrCreate") {
        let mut rows = Vec::new();
        for item in items {
            let Some(spec) = item.as_object() else {
                return Err(err(format!("{label}.connectOrCreate required")));
            };
            let Some(create) = spec.get("create").cloned() else {
                return Err(err(format!("{label}.connectOrCreate.create required")));
            };
            let where_value = spec.get("where").cloned().unwrap_or(Value::Null);
            if find_edge_id(db, meta, &parent_thing, &where_value, &label).await?.is_none() {
                rows.extend(edge_connect_rows(
                    meta,
                    parent_id,
                    &[create],
                    &format!("{label}.connectOrCreate.create"),
                )?);
            }
        }
        if !rows.is_empty() {
            ops.push(NestedOp::Relate { table: edge_table.clone(), rows });
        }
    }

    if let Some(Value::Array(items)) = bag.get("create") {
        let mut relate_rows = Vec::new();
        let mut far_items = Vec::new();
        for item in items {
            let Some(row) = item.as_object() else { continue };
            let content = edge_content(row, payload_fields);
            let far_val = row.get(far_key);
            if let Some(far_id) = far_val.and_then(Value::as_str) {
                if !far_items.is_empty() {
                    ops.push(NestedOp::RelateNewFar {
                        edge_table: edge_table.clone(),
                        far_table: far_table.clone(),
                        far_is_out: matches!(dir, crate::meta::RelationDir::Out),
                        items: std::mem::take(&mut far_items),
                    });
                }
                relate_rows.push(Value::Object(relation_row(meta, parent_id, far_id, content)));
            } else if far_val
                .and_then(Value::as_object)
                .is_some_and(|map| map.contains_key("create"))
            {
                let create_data = far_val
                    .and_then(Value::as_object)
                    .and_then(|map| map.get("create"))
                    .and_then(Value::as_object)
                    .cloned()
                    .unwrap_or_default();
                assert_no_nested_writes(
                    &create_data,
                    &far_table,
                    &format!("create {label}.{far_key}"),
                )?;
                if !relate_rows.is_empty() {
                    ops.push(NestedOp::Relate {
                        table: edge_table.clone(),
                        rows: std::mem::take(&mut relate_rows),
                    });
                }
                let mut item_value = JsonMap::new();
                item_value.insert(
                    "far".to_owned(),
                    Value::Object(edge_far_data(parent_table, parent_id, &far_table, create_data)),
                );
                item_value.insert("content".to_owned(), Value::Object(content));
                item_value.insert("as_parent".to_owned(), Value::String(parent_thing.clone()));
                far_items.push(Value::Object(item_value));
            } else {
                return Err(err(format!("{label}.create.{far_key} required")));
            }
        }
        if !relate_rows.is_empty() {
            ops.push(NestedOp::Relate { table: edge_table.clone(), rows: relate_rows });
        }
        if !far_items.is_empty() {
            ops.push(NestedOp::RelateNewFar {
                edge_table: edge_table.clone(),
                far_table,
                far_is_out: matches!(dir, crate::meta::RelationDir::Out),
                items: far_items,
            });
        }
    }
    flush_ops(db, ops).await?;

    if let Some(Value::Array(items)) = bag.get("update") {
        for item in items {
            let Some(spec) = item.as_object() else {
                return Err(err(format!("{label}.update required")));
            };
            let Some(data) = spec.get("data").and_then(Value::as_object) else {
                return Err(err(format!("{label}.update data required")));
            };
            let where_value = spec.get("where").cloned().unwrap_or(Value::Null);
            let Some(id) = find_edge_id(db, meta, &parent_thing, &where_value, &label).await?
            else {
                return Err(err(format!("{label}.update where matched no record")));
            };
            ops.push(NestedOp::Merge {
                ids: vec![normalize_thing(edge_table, &id)],
                patch: edge_content(data, payload_fields),
            });
        }
    }
    flush_ops(db, ops).await?;

    if let Some(Value::Array(items)) = bag.get("upsert") {
        let mut rows = Vec::new();
        for item in items {
            let Some(spec) = item.as_object() else {
                return Err(err(format!("{label}.upsert required")));
            };
            let Some(update) = spec.get("update").and_then(Value::as_object) else {
                return Err(err(format!("{label}.upsert.update required")));
            };
            let Some(create) = spec.get("create").cloned() else {
                return Err(err(format!("{label}.upsert.create required")));
            };
            let where_value = spec.get("where").cloned().unwrap_or(Value::Null);
            if let Some(existing) =
                find_edge_id(db, meta, &parent_thing, &where_value, &label).await?
            {
                ops.push(NestedOp::Merge {
                    ids: vec![normalize_thing(edge_table, &existing)],
                    patch: edge_content(update, payload_fields),
                });
            } else {
                rows.extend(edge_connect_rows(
                    meta,
                    parent_id,
                    &[create],
                    &format!("{label}.upsert.create"),
                )?);
            }
        }
        if !rows.is_empty() {
            ops.push(NestedOp::Relate { table: edge_table.clone(), rows });
        }
    }
    flush_ops(db, ops).await?;
    Ok(())
}

pub(crate) async fn apply_nested_writes(
    db: &DbHandle,
    table: &str,
    parent_id: &str,
    nested: &JsonMap,
) -> Result<(), ClientError> {
    let meta = write_meta();
    let table_meta = meta.get(table);
    let mut ops = Vec::new();
    for (field, value) in entries(nested) {
        let Some(field_meta) = table_meta.and_then(|fields| fields.get(field)) else {
            continue;
        };
        if !is_nested_write_bag(value) {
            continue;
        }
        let Some(bag) = value.as_object() else { continue };
        match field_meta {
            FieldWriteMeta::Computed { .. } => {
                collect_computed(db, &mut ops, table, parent_id, field, bag, field_meta).await?;
            }
            FieldWriteMeta::Edge { .. } => {
                collect_edge(db, &mut ops, table, parent_id, field, bag, field_meta).await?;
            }
            FieldWriteMeta::Scalar | FieldWriteMeta::Stored { .. } => {}
        }
    }
    run_nested_ops(db, &ops).await
}

/// True when `data` contains a nested write for `table`.
#[must_use]
pub fn row_has_nested_write(table: &str, data: &Value) -> bool {
    let Some(map) = data.as_object() else {
        return false;
    };
    let meta = write_meta();
    let table_meta = meta.get(table);
    map.iter().any(|(key, value)| {
        let Some(field_meta) = table_meta.and_then(|fields| fields.get(key)) else {
            return false;
        };
        match field_meta {
            FieldWriteMeta::Computed { .. } | FieldWriteMeta::Edge { .. } => !value.is_null(),
            FieldWriteMeta::Stored { .. } => is_nested_write_bag(value),
            FieldWriteMeta::Scalar => false,
        }
    })
}
