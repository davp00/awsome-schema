//! Pure SurrealQL fragments: where, projection, order, and group-by.

use std::fmt::Write as _;

use serde_json::{Map, Value};

use crate::error::{ClientError, err};
use crate::id::normalize_thing;
use crate::jsonutil::{JsonMap, entries, number_of};
use crate::meta::{FieldSelectMeta, ScalarFilter, select_meta};

/// Bindings accumulated while compiling a `where` clause.
#[derive(Debug, Default)]
pub struct WhereCtx {
    /// Bound values, named `w0`, `w1`, ...
    pub vars: JsonMap,
    /// Next variable index.
    pub n: usize,
}

impl WhereCtx {
    /// Empty context.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

/// Allocate `$wN` and store `value`.
pub fn next_where_var(ctx: &mut WhereCtx, value: Value) -> String {
    let key = format!("w{}", ctx.n);
    ctx.n += 1;
    ctx.vars.insert(key.clone(), value);
    format!("${key}")
}

fn has_filter_key(map: &JsonMap, key: &str) -> bool {
    map.contains_key(key)
}

/// True when `value` is a filter object (`equals`, `in`, `contains`, comparisons).
#[must_use]
pub fn is_scalar_filter_object(value: &Value) -> bool {
    let Some(map) = value.as_object() else {
        return false;
    };
    ["equals", "in", "contains", "gt", "gte", "lt", "lte"]
        .iter()
        .any(|key| has_filter_key(map, key))
}

/// Compile one scalar comparison. Record ids use `type::record`.
pub fn scalar_predicate(
    path: &str,
    value: &Value,
    filter: &str,
    ctx: &mut WhereCtx,
    id_table: Option<&str>,
) -> Result<Vec<String>, ClientError> {
    let record_identity = filter == "id";
    let bind_record = |ctx: &mut WhereCtx, bound: &Value| -> String {
        let raw = match (bound.as_str(), id_table) {
            (Some(text), Some(table)) => Value::String(normalize_thing(table, text)),
            _ => bound.clone(),
        };
        format!("type::record({})", next_where_var(ctx, raw))
    };
    let eq = |ctx: &mut WhereCtx, bound: &Value| -> String {
        if record_identity {
            format!("{path} = {}", bind_record(ctx, bound))
        } else {
            format!("{path} = {}", next_where_var(ctx, bound.clone()))
        }
    };

    if !is_scalar_filter_object(value) {
        return Ok(vec![eq(ctx, value)]);
    }
    let Some(map) = value.as_object() else {
        return Ok(vec![eq(ctx, value)]);
    };
    let mut parts = Vec::new();
    if let Some(equals) = map.get("equals") {
        parts.push(eq(ctx, equals));
    }
    if let Some(list) = map.get("in") {
        if record_identity {
            let Value::Array(items) = list else {
                return Err(err(format!("{path}.in must be an array of record ids")));
            };
            let rendered =
                items.iter().map(|item| bind_record(ctx, item)).collect::<Vec<_>>().join(", ");
            parts.push(format!("{path} IN [{rendered}]"));
        } else {
            parts.push(format!("{path} IN {}", next_where_var(ctx, list.clone())));
        }
    }
    if let Some(contains) = map.get("contains") {
        if filter == "array" {
            parts.push(format!("{path} CONTAINS {}", next_where_var(ctx, contains.clone())));
        } else {
            parts.push(format!(
                "string::contains({path}, {})",
                next_where_var(ctx, contains.clone())
            ));
        }
    }
    if let Some(gt) = map.get("gt") {
        parts.push(format!("{path} > {}", next_where_var(ctx, gt.clone())));
    }
    if let Some(gte) = map.get("gte") {
        parts.push(format!("{path} >= {}", next_where_var(ctx, gte.clone())));
    }
    if let Some(lt) = map.get("lt") {
        parts.push(format!("{path} < {}", next_where_var(ctx, lt.clone())));
    }
    if let Some(lte) = map.get("lte") {
        parts.push(format!("{path} <= {}", next_where_var(ctx, lte.clone())));
    }
    Ok(parts)
}

/// `{ id: "…" }` or `{ id: { equals: "…" } }` on a link filter.
#[must_use]
pub fn record_id_filter_value(value: &Value) -> Option<String> {
    let map = value.as_object()?;
    let keys = present_defined(map);
    if keys.len() != 1 || keys[0] != "id" {
        return None;
    }
    let id = map.get("id")?;
    if let Some(text) = id.as_str() {
        return Some(text.to_owned());
    }
    let equals = id.as_object()?.get("equals")?.as_str()?;
    Some(equals.to_owned())
}

fn present_defined(map: &JsonMap) -> Vec<&str> {
    map.keys().map(String::as_str).collect()
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum RelationQuantifier {
    Some,
    Every,
    None,
    Is,
}

pub(crate) struct RelationMode<'a> {
    pub mode: RelationQuantifier,
    pub nested: &'a Value,
}

pub(crate) fn relation_mode(value: &Value, list: bool) -> RelationMode<'_> {
    let Some(map) = value.as_object() else {
        return RelationMode {
            mode: if list { RelationQuantifier::Some } else { RelationQuantifier::Is },
            nested: value,
        };
    };
    if let Some(nested) = map.get("some") {
        return RelationMode { mode: RelationQuantifier::Some, nested };
    }
    if let Some(nested) = map.get("every") {
        return RelationMode { mode: RelationQuantifier::Every, nested };
    }
    if let Some(nested) = map.get("none") {
        return RelationMode { mode: RelationQuantifier::None, nested };
    }
    if let Some(nested) = map.get("is") {
        return RelationMode { mode: RelationQuantifier::Is, nested };
    }
    RelationMode {
        mode: if list { RelationQuantifier::Some } else { RelationQuantifier::Is },
        nested: value,
    }
}

pub(crate) fn list_relation_predicate(
    collection: &str,
    mode: RelationQuantifier,
    nested_clause: &str,
) -> Option<String> {
    match mode {
        RelationQuantifier::Is => None,
        RelationQuantifier::Some => Some(if nested_clause.is_empty() {
            format!("array::len({collection}) > 0")
        } else {
            format!("array::len({collection}[WHERE {nested_clause}]) > 0")
        }),
        RelationQuantifier::Every => Some(if nested_clause.is_empty() {
            "true".to_owned()
        } else {
            format!("array::len({collection}[WHERE {nested_clause}]) = array::len({collection})")
        }),
        RelationQuantifier::None => Some(if nested_clause.is_empty() {
            format!("array::len({collection}) = 0")
        } else {
            format!("array::len({collection}[WHERE {nested_clause}]) = 0")
        }),
    }
}

/// Compile a `where` object into a SurrealQL predicate. Empty when nothing matches.
pub fn build_where(
    where_value: &Value,
    table: &str,
    ctx: &mut WhereCtx,
) -> Result<String, ClientError> {
    let meta = select_meta();
    build_where_with(where_value, table, meta.as_ref(), ctx, "")
}

pub(crate) fn build_where_with(
    where_value: &Value,
    table: &str,
    all_meta: &crate::meta::SelectMeta,
    ctx: &mut WhereCtx,
    path_prefix: &str,
) -> Result<String, ClientError> {
    let Some(map) = where_value.as_object() else {
        return Ok(String::new());
    };
    let table_meta = all_meta.get(table);
    let mut parts = Vec::new();
    for (key, value) in entries(map) {
        if key == "AND" {
            push_logic(&mut parts, value, table, all_meta, ctx, path_prefix, " AND ", false)?;
            continue;
        }
        if key == "OR" {
            push_logic(&mut parts, value, table, all_meta, ctx, path_prefix, " OR ", false)?;
            continue;
        }
        if key == "NOT" {
            push_logic(&mut parts, value, table, all_meta, ctx, path_prefix, " AND ", true)?;
            continue;
        }
        let Some(field_meta) = table_meta.and_then(|meta| meta.get(key)) else {
            continue;
        };
        let path =
            if path_prefix.is_empty() { key.to_owned() } else { format!("{path_prefix}.{key}") };
        match field_meta {
            FieldSelectMeta::Scalar { filter } => {
                let id_table = if *filter == ScalarFilter::Id { Some(table) } else { None };
                parts.extend(scalar_predicate(&path, value, filter.as_str(), ctx, id_table)?);
            }
            FieldSelectMeta::Object { fields } => {
                let clause = compile_object_where(value, fields, &path, ctx)?;
                if !clause.is_empty() {
                    parts.push(clause);
                }
            }
            FieldSelectMeta::Edge { edge_table, dir, list, .. } => {
                if !value.is_object() {
                    continue;
                }
                let mode = relation_mode(value, *list);
                let arrow = match dir {
                    crate::meta::RelationDir::Out => format!("->{edge_table}"),
                    crate::meta::RelationDir::In => format!("<-{edge_table}"),
                };
                let nested = if mode.nested.is_object() {
                    build_where_with(mode.nested, edge_table, all_meta, ctx, "")?
                } else {
                    String::new()
                };
                if let Some(pred) = list_relation_predicate(&arrow, mode.mode, &nested) {
                    parts.push(pred);
                }
            }
            FieldSelectMeta::Stored { target_table, list }
            | FieldSelectMeta::Computed { target_table, list } => {
                if !value.is_object() {
                    continue;
                }
                let mode = relation_mode(value, *list);
                if *list {
                    let nested = if mode.nested.is_object() {
                        build_where_with(mode.nested, target_table, all_meta, ctx, "")?
                    } else {
                        String::new()
                    };
                    if let Some(pred) = list_relation_predicate(&path, mode.mode, &nested) {
                        parts.push(pred);
                    }
                    continue;
                }
                if let Some(link_id) = record_id_filter_value(mode.nested) {
                    let thing = normalize_thing(target_table, &link_id);
                    parts.push(format!(
                        "{path} = type::record({})",
                        next_where_var(ctx, Value::String(thing))
                    ));
                    continue;
                }
                if mode.nested.is_object() {
                    let nested = build_where_with(mode.nested, target_table, all_meta, ctx, &path)?;
                    if !nested.is_empty() {
                        parts.push(nested);
                    }
                }
            }
        }
    }
    Ok(parts.join(" AND "))
}

fn push_logic(
    parts: &mut Vec<String>,
    value: &Value,
    table: &str,
    all_meta: &crate::meta::SelectMeta,
    ctx: &mut WhereCtx,
    path_prefix: &str,
    joiner: &str,
    negate: bool,
) -> Result<(), ClientError> {
    let items = match value {
        Value::Array(items) => items.iter().collect::<Vec<_>>(),
        other => vec![other],
    };
    let mut inner = Vec::new();
    for item in items {
        let clause = build_where_with(item, table, all_meta, ctx, path_prefix)?;
        if !clause.is_empty() {
            inner.push(clause);
        }
    }
    if inner.is_empty() {
        return Ok(());
    }
    if negate {
        if inner.len() == 1 {
            parts.push(format!("!({})", inner[0]));
        } else {
            parts.push(format!("!(({}))", inner.join(" AND ")));
        }
    } else {
        parts.push(format!("({})", inner.join(joiner)));
    }
    Ok(())
}

/// Compile a typed object filter. `equals` is the whole value.
pub fn compile_object_where(
    value: &Value,
    fields: &std::collections::BTreeMap<String, FieldSelectMeta>,
    path: &str,
    ctx: &mut WhereCtx,
) -> Result<String, ClientError> {
    let Some(record) = value.as_object() else {
        return Ok(scalar_predicate(path, value, "json", ctx, None)?.join(" AND "));
    };
    let mut parts = Vec::new();
    for (key, child) in entries(record) {
        if key == "AND" || key == "OR" {
            let joiner = if key == "OR" { " OR " } else { " AND " };
            let items = match child {
                Value::Array(items) => items.iter().collect::<Vec<_>>(),
                other => vec![other],
            };
            let mut inner = Vec::new();
            for item in items {
                let clause = compile_object_where(item, fields, path, ctx)?;
                if !clause.is_empty() {
                    inner.push(clause);
                }
            }
            if !inner.is_empty() {
                parts.push(format!("({})", inner.join(joiner)));
            }
            continue;
        }
        if key == "NOT" {
            let items = match child {
                Value::Array(items) => items.iter().collect::<Vec<_>>(),
                other => vec![other],
            };
            let mut inner = Vec::new();
            for item in items {
                let clause = compile_object_where(item, fields, path, ctx)?;
                if !clause.is_empty() {
                    inner.push(clause);
                }
            }
            if inner.len() == 1 {
                parts.push(format!("!({})", inner[0]));
            } else if inner.len() > 1 {
                parts.push(format!("!(({}))", inner.join(" AND ")));
            }
            continue;
        }
        if key == "equals" {
            parts.push(format!("{path} = {}", next_where_var(ctx, child.clone())));
            continue;
        }
        let Some(child_meta) = fields.get(key) else {
            continue;
        };
        let child_path = format!("{path}.{key}");
        match child_meta {
            FieldSelectMeta::Object { fields } => {
                let inner = compile_object_where(child, fields, &child_path, ctx)?;
                if !inner.is_empty() {
                    parts.push(inner);
                }
            }
            FieldSelectMeta::Scalar { filter } => {
                parts.extend(scalar_predicate(&child_path, child, filter.as_str(), ctx, None)?);
            }
            FieldSelectMeta::Stored { .. }
            | FieldSelectMeta::Computed { .. }
            | FieldSelectMeta::Edge { .. } => {}
        }
    }
    Ok(parts.join(" AND "))
}

fn order_items(order_by: &Value) -> Vec<&JsonMap> {
    match order_by {
        Value::Array(items) => items.iter().filter_map(Value::as_object).collect(),
        Value::Object(map) => vec![map],
        _ => Vec::new(),
    }
}

/// `array::len(...)` for ordering a list relation by `_count`.
#[must_use]
pub fn relation_count_expr(key: &str, field_meta: &FieldSelectMeta) -> Option<String> {
    if !field_meta.list() {
        return None;
    }
    match field_meta {
        FieldSelectMeta::Edge { edge_table, dir, .. } => {
            let arrow = match dir {
                crate::meta::RelationDir::Out => format!("->{edge_table}"),
                crate::meta::RelationDir::In => format!("<-{edge_table}"),
            };
            Some(format!("array::len({arrow})"))
        }
        FieldSelectMeta::Stored { .. } | FieldSelectMeta::Computed { .. } => {
            Some(format!("array::len({key})"))
        }
        FieldSelectMeta::Scalar { .. } | FieldSelectMeta::Object { .. } => None,
    }
}

/// Alias used for relation `_count` ordering.
#[must_use]
pub fn order_by_count_alias(key: &str) -> String {
    format!("__ob_{key}")
}

/// Extra select expressions required by relation `_count` ordering.
#[must_use]
pub fn collect_order_by_count_projections(order_by: Option<&Value>, table: &str) -> Vec<String> {
    let Some(order_by) = order_by else {
        return Vec::new();
    };
    let meta = select_meta();
    collect_order_by_count_projections_with(order_by, table, meta.as_ref())
}

pub(crate) fn collect_order_by_count_projections_with(
    order_by: &Value,
    table: &str,
    all_meta: &crate::meta::SelectMeta,
) -> Vec<String> {
    let table_meta = all_meta.get(table);
    let mut parts = Vec::new();
    let mut seen = Vec::new();
    for item in order_items(order_by) {
        for (key, value) in entries(item) {
            let Some(map) = value.as_object() else {
                continue;
            };
            if !map.contains_key("_count") {
                continue;
            }
            let dir = map.get("_count").and_then(Value::as_str);
            if dir != Some("asc") && dir != Some("desc") {
                continue;
            }
            let Some(field_meta) = table_meta.and_then(|meta| meta.get(key)) else {
                continue;
            };
            let Some(expr) = relation_count_expr(key, field_meta) else {
                continue;
            };
            if seen.iter().any(|seen_key: &String| seen_key == key) {
                continue;
            }
            seen.push(key.to_owned());
            parts.push(format!("{expr} AS {}", order_by_count_alias(key)));
        }
    }
    parts
}

/// Remove `__ob_*` fields injected for ordering.
pub fn strip_order_by_count_fields(rows: Vec<Value>) -> Vec<Value> {
    rows.into_iter()
        .map(|row| {
            let Value::Object(mut map) = row else {
                return row;
            };
            map.retain(|key, _| !key.starts_with("__ob_"));
            Value::Object(map)
        })
        .collect()
}

/// `ORDER BY ...` or empty.
#[must_use]
pub fn build_order_by(order_by: Option<&Value>, table: &str) -> String {
    let Some(order_by) = order_by else {
        return String::new();
    };
    let meta = select_meta();
    build_order_by_with(order_by, table, meta.as_ref())
}

pub(crate) fn build_order_by_with(
    order_by: &Value,
    table: &str,
    all_meta: &crate::meta::SelectMeta,
) -> String {
    let table_meta = all_meta.get(table);
    let mut parts = Vec::new();
    for item in order_items(order_by) {
        for (key, value) in entries(item) {
            let Some(field_meta) = table_meta.and_then(|meta| meta.get(key)) else {
                continue;
            };
            if let Some(dir) = value.as_str()
                && (dir == "asc" || dir == "desc")
                && matches!(field_meta, FieldSelectMeta::Scalar { .. })
            {
                parts.push(format!("{key} {}", dir.to_ascii_uppercase()));
                continue;
            }
            if let Some(map) = value.as_object()
                && map.contains_key("_count")
            {
                let dir = map.get("_count").and_then(Value::as_str);
                if dir != Some("asc") && dir != Some("desc") {
                    continue;
                }
                if relation_count_expr(key, field_meta).is_none() {
                    continue;
                }
                let dir = dir.unwrap_or("asc");
                parts.push(format!("{} {}", order_by_count_alias(key), dir.to_ascii_uppercase()));
            }
        }
    }
    if parts.is_empty() { String::new() } else { format!("ORDER BY {}", parts.join(", ")) }
}

/// `LIMIT` / `START` literals for a nested subquery.
pub fn nested_limit_clause(
    take: Option<&Value>,
    skip: Option<&Value>,
) -> Result<String, ClientError> {
    let mut clause = String::new();
    if let Some(take) = take {
        let take = non_negative(take, "take")?;
        let _ = write!(clause, " LIMIT {take}");
    }
    if let Some(skip) = skip {
        let skip = non_negative(skip, "skip")?;
        let _ = write!(clause, " START {skip}");
    }
    Ok(clause)
}

fn non_negative(value: &Value, name: &str) -> Result<u64, ClientError> {
    let Some(number) = number_of(value) else {
        return Err(err(format!("{name} must be a non-negative number")));
    };
    if !number.is_finite() || number < 0.0 {
        return Err(err(format!("{name} must be a non-negative number")));
    }
    Ok(number.floor() as u64)
}

/// Append bound `LIMIT` / `START` to a statement.
pub fn append_limit_start(
    sql: &str,
    take: Option<&Value>,
    skip: Option<&Value>,
    vars: &mut JsonMap,
) -> Result<String, ClientError> {
    let mut out = sql.to_owned();
    if let Some(take) = take {
        let take = non_negative(take, "take")?;
        vars.insert("__take".to_owned(), Value::from(take));
        out.push_str(" LIMIT $__take");
    }
    if let Some(skip) = skip {
        let skip = non_negative(skip, "skip")?;
        vars.insert("__skip".to_owned(), Value::from(skip));
        out.push_str(" START $__skip");
    }
    Ok(out)
}

/// One selected field expression, or `None` when the key is skipped.
pub fn project_field(
    key: &str,
    value: &Value,
    field_meta: &FieldSelectMeta,
    all_meta: &crate::meta::SelectMeta,
) -> Result<Option<String>, ClientError> {
    if value.is_null() || value.as_bool() == Some(false) {
        return Ok(None);
    }
    if matches!(field_meta, FieldSelectMeta::Scalar { .. } | FieldSelectMeta::Object { .. }) {
        return Ok(Some(key.to_owned()));
    }
    if value.as_bool() == Some(true) {
        if let FieldSelectMeta::Edge { edge_table, dir, .. } = field_meta {
            let arrow = match dir {
                crate::meta::RelationDir::Out => format!("->{edge_table}"),
                crate::meta::RelationDir::In => format!("<-{edge_table}"),
            };
            return Ok(Some(format!("{arrow}.* AS {key}")));
        }
        return Ok(Some(format!("{key}.*")));
    }
    let Some(bag) = value.as_object() else {
        return Ok(None);
    };
    if !(bag.contains_key("select")
        || bag.contains_key("orderBy")
        || bag.contains_key("take")
        || bag.contains_key("skip"))
    {
        return Ok(None);
    }
    let nested_table = match field_meta {
        FieldSelectMeta::Edge { edge_table, .. } => edge_table.as_str(),
        FieldSelectMeta::Stored { target_table, .. }
        | FieldSelectMeta::Computed { target_table, .. } => target_table.as_str(),
        FieldSelectMeta::Scalar { .. } | FieldSelectMeta::Object { .. } => return Ok(None),
    };
    let mut nested = build_projection_with(bag.get("select"), nested_table, all_meta)?;
    let count_proj = bag
        .get("orderBy")
        .map(|order| collect_order_by_count_projections_with(order, nested_table, all_meta))
        .unwrap_or_default();
    if !count_proj.is_empty() {
        nested = format!("{nested}, {}", count_proj.join(", "));
    }
    let order_clause = bag
        .get("orderBy")
        .map(|order| build_order_by_with(order, nested_table, all_meta))
        .unwrap_or_default();
    let order_clause =
        if order_clause.is_empty() && (bag.contains_key("take") || bag.contains_key("skip")) {
            "ORDER BY id".to_owned()
        } else {
            order_clause
        };
    let page = nested_limit_clause(bag.get("take"), bag.get("skip"))?;
    if !order_clause.is_empty() || !page.is_empty() {
        let tail = format!(
            "{}{page}",
            if order_clause.is_empty() { String::new() } else { format!(" {order_clause}") }
        );
        if let FieldSelectMeta::Edge { edge_table, dir, .. } = field_meta {
            let arrow = match dir {
                crate::meta::RelationDir::Out => "->",
                crate::meta::RelationDir::In => "<-",
            };
            return Ok(Some(format!("{arrow}(SELECT {nested} FROM {edge_table}{tail}) AS {key}")));
        }
        return Ok(Some(format!("(SELECT {nested} FROM $parent.{key}{tail}) AS {key}")));
    }
    if let FieldSelectMeta::Edge { edge_table, dir, .. } = field_meta {
        let arrow = match dir {
            crate::meta::RelationDir::Out => format!("->{edge_table}"),
            crate::meta::RelationDir::In => format!("<-{edge_table}"),
        };
        return Ok(Some(format!("{arrow}.{{ {nested} }} AS {key}")));
    }
    Ok(Some(format!("{key}.{{ {nested} }}")))
}

/// `SELECT` projection list, or `*`.
pub fn build_projection(select: Option<&Value>, table: &str) -> Result<String, ClientError> {
    let meta = select_meta();
    build_projection_with(select, table, meta.as_ref())
}

pub(crate) fn build_projection_with(
    select: Option<&Value>,
    table: &str,
    all_meta: &crate::meta::SelectMeta,
) -> Result<String, ClientError> {
    let Some(select) = select else {
        return Ok("*".to_owned());
    };
    let Some(map) = select.as_object() else {
        return Ok("*".to_owned());
    };
    let table_meta = all_meta.get(table);
    let mut parts = Vec::new();
    for (key, value) in entries(map) {
        let Some(field_meta) = table_meta.and_then(|meta| meta.get(key)) else {
            continue;
        };
        if let Some(part) = project_field(key, value, field_meta, all_meta)? {
            parts.push(part);
        }
    }
    if parts.is_empty() { Ok("*".to_owned()) } else { Ok(parts.join(", ")) }
}

/// Project a fetched row the way delete+select does in memory.
#[must_use]
pub fn project_row(row: &Value, select: Option<&Value>) -> Value {
    let Some(select) = select.and_then(Value::as_object) else {
        return row.clone();
    };
    let Some(record) = row.as_object() else {
        return row.clone();
    };
    let mut out = Map::new();
    for (key, value) in entries(select) {
        if value.is_null() || value.as_bool() == Some(false) {
            continue;
        }
        if value.as_bool() == Some(true) {
            if let Some(found) = record.get(key) {
                out.insert(key.to_owned(), found.clone());
            }
            continue;
        }
        let Some(bag) = value.as_object() else {
            continue;
        };
        if !bag.contains_key("select") {
            continue;
        }
        let nested_select = bag.get("select");
        match record.get(key) {
            Some(Value::Array(items)) => {
                let projected = items
                    .iter()
                    .map(|item| {
                        if item.is_object() {
                            project_row(item, nested_select)
                        } else {
                            item.clone()
                        }
                    })
                    .collect();
                out.insert(key.to_owned(), Value::Array(projected));
            }
            Some(nested) if nested.is_object() => {
                out.insert(key.to_owned(), project_row(nested, nested_select));
            }
            Some(nested) => {
                out.insert(key.to_owned(), nested.clone());
            }
            None => {}
        }
    }
    Value::Object(out)
}

/// `{ id: "…" }` or `{ id: { equals } }` as the only where key.
#[must_use]
pub fn unique_where_id(where_value: Option<&Value>) -> Option<String> {
    let map = where_value?.as_object()?;
    let keys = present_defined(map);
    if keys.len() != 1 || keys[0] != "id" {
        return None;
    }
    let id = map.get("id")?;
    if let Some(text) = id.as_str() {
        return Some(text.to_owned());
    }
    id.as_object()?.get("equals")?.as_str().map(ToOwned::to_owned)
}

/// Group-by aggregate alias.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GroupByAlias {
    /// `_count`, `_sum`, `_avg`, `_min`, or `_max`.
    pub kind: String,
    /// Field or `_all`.
    pub field: String,
    /// SQL alias.
    pub alias: String,
}

/// `ORDER BY` for a grouped query.
#[must_use]
pub fn build_group_by_order_by(order_by: Option<&Value>, by: &[String]) -> String {
    let Some(order_by) = order_by else {
        return String::new();
    };
    let mut parts = Vec::new();
    for item in order_items(order_by) {
        for (key, value) in entries(item) {
            if key == "_count" && value.is_object() {
                let all = value.as_object().and_then(|map| map.get("_all")).and_then(Value::as_str);
                if all == Some("asc") || all == Some("desc") {
                    parts
                        .push(format!("__count_all {}", all.unwrap_or("asc").to_ascii_uppercase()));
                }
                continue;
            }
            if let Some(dir) = value.as_str()
                && (dir == "asc" || dir == "desc")
                && by.iter().any(|field| field == key)
            {
                parts.push(format!("{key} {}", dir.to_ascii_uppercase()));
            }
        }
    }
    if parts.is_empty() { String::new() } else { format!("ORDER BY {}", parts.join(", ")) }
}

/// Fold SQL aggregate aliases back into `{ _count: { field: n } }` buckets.
#[must_use]
pub fn reshape_group_by_row(row: &Value, by: &[String], aliases: &[GroupByAlias]) -> Value {
    let record = row.as_object();
    let mut out = Map::new();
    for field in by {
        if let Some(value) = record.and_then(|map| map.get(field)) {
            out.insert(field.clone(), value.clone());
        }
    }
    for alias in aliases {
        let bucket = out.entry(alias.kind.clone()).or_insert_with(|| Value::Object(Map::new()));
        let Value::Object(bucket_map) = bucket else {
            continue;
        };
        let raw = record.and_then(|map| map.get(&alias.alias));
        if alias.kind == "_count" {
            let number = raw.and_then(number_of).unwrap_or(0.0);
            bucket_map.insert(alias.field.clone(), Value::from(number));
        } else {
            bucket_map.insert(alias.field.clone(), raw.cloned().unwrap_or(Value::Null));
        }
    }
    Value::Object(out)
}

/// Reject unsupported aggregate comparison operators.
pub fn assert_aggregate_filter(
    table: &str,
    kind: &str,
    field: &str,
    filter: &Value,
) -> Result<(), ClientError> {
    let Some(map) = filter.as_object() else {
        return Ok(());
    };
    for op in map.keys() {
        if matches!(op.as_str(), "equals" | "gt" | "gte" | "lt" | "lte") {
            continue;
        }
        return Err(err(format!("groupBy {table}: having {kind}.{field} does not support {op}")));
    }
    Ok(())
}

/// `WHERE` predicate for a grouped subquery, or empty.
pub fn build_group_having(
    table: &str,
    having: Option<&Value>,
    by: &[String],
    aliases: &[GroupByAlias],
    ctx: &mut WhereCtx,
) -> Result<String, ClientError> {
    let Some(having) = having.and_then(Value::as_object) else {
        return Ok(String::new());
    };
    let meta = select_meta();
    let mut parts = Vec::new();
    for (key, value) in entries(having) {
        if value.is_null() {
            continue;
        }
        if matches!(key, "_count" | "_sum" | "_avg" | "_min" | "_max") {
            let Some(spec) = value.as_object() else {
                return Err(err(format!("groupBy {table}: having {key} must be an object")));
            };
            for (field, filter) in entries(spec) {
                if filter.is_null() {
                    continue;
                }
                let Some(alias) =
                    aliases.iter().find(|item| item.kind == key && item.field == field)
                else {
                    return Err(err(format!(
                        "groupBy {table}: having {key}.{field} was not selected"
                    )));
                };
                assert_aggregate_filter(table, key, field, filter)?;
                parts.extend(scalar_predicate(&alias.alias, filter, "number", ctx, None)?);
            }
            continue;
        }
        if !by.iter().any(|field| field == key) {
            return Err(err(format!("groupBy {table}: having {key} was not selected")));
        }
        let field_meta = meta.get(table).and_then(|table_meta| table_meta.get(key));
        if let Some(FieldSelectMeta::Object { fields }) = field_meta {
            let clause = compile_object_where(value, fields, key, ctx)?;
            if !clause.is_empty() {
                parts.push(clause);
            }
            continue;
        }
        let filter_kind =
            field_meta.and_then(FieldSelectMeta::filter_kind).unwrap_or(ScalarFilter::String);
        parts.extend(scalar_predicate(key, value, filter_kind.as_str(), ctx, None)?);
    }
    Ok(parts.join(" AND "))
}

/// `where` must contain at least one key.
pub fn assert_non_empty_where(
    where_value: Option<&Value>,
    op: &str,
    table: &str,
) -> Result<(), ClientError> {
    let Some(map) = where_value.and_then(Value::as_object) else {
        return Err(err(format!("{op} {table}: where must not be empty")));
    };
    if map.is_empty() {
        return Err(err(format!("{op} {table}: where must not be empty")));
    }
    Ok(())
}

/// `select` and `return` cannot be combined.
pub fn assert_return_select_exclusive(
    op: &str,
    table: &str,
    select: Option<&Value>,
    ret: Option<&str>,
) -> Result<(), ClientError> {
    let has_select = select.is_some_and(|value| !value.is_null());
    if has_select && ret.is_some() {
        return Err(err(format!("{op} {table}: select and return are mutually exclusive")));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::meta::{FieldSelectMeta, RelationDir, ScalarFilter, bind_schema_meta};
    use serde_json::json;
    use std::collections::BTreeMap;

    fn sql(value: &serde_json::Value, table: &str, ctx: &mut WhereCtx) -> String {
        build_where(value, table, ctx).expect("where")
    }

    fn user_email() -> crate::meta::SelectMeta {
        let mut user = BTreeMap::new();
        user.insert("email".to_owned(), FieldSelectMeta::scalar(ScalarFilter::String));
        user.insert(
            "posts".to_owned(),
            FieldSelectMeta::Computed { target_table: "post".to_owned(), list: true },
        );
        let mut meta = BTreeMap::new();
        meta.insert("user".to_owned(), user);
        meta
    }

    fn lock_meta() -> std::sync::MutexGuard<'static, ()> {
        static LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
        LOCK.lock().expect("schema meta lock")
    }

    #[test]
    fn builds_string_contains_and_relation_count_order() {
        let _guard = lock_meta();
        bind_schema_meta(user_email(), BTreeMap::new());
        let mut ctx = WhereCtx::new();
        let clause = sql(&json!({"email": {"contains": "ada"}}), "user", &mut ctx);
        assert!(clause.contains("string::contains(email,"), "{clause}");
        let order = build_order_by(Some(&json!({"posts": {"_count": "desc"}})), "user");
        assert_eq!(order, "ORDER BY __ob_posts DESC");
    }

    #[test]
    fn compares_record_identity_via_type_record() {
        let _guard = lock_meta();
        let mut user = BTreeMap::new();
        user.insert("id".to_owned(), FieldSelectMeta::scalar(ScalarFilter::Id));
        user.insert("email".to_owned(), FieldSelectMeta::scalar(ScalarFilter::String));
        let mut post = BTreeMap::new();
        post.insert("id".to_owned(), FieldSelectMeta::scalar(ScalarFilter::Id));
        post.insert(
            "author".to_owned(),
            FieldSelectMeta::Stored { target_table: "user".to_owned(), list: false },
        );
        let mut likes = BTreeMap::new();
        likes.insert("id".to_owned(), FieldSelectMeta::scalar(ScalarFilter::Id));
        likes.insert(
            "out".to_owned(),
            FieldSelectMeta::Stored { target_table: "post".to_owned(), list: false },
        );
        likes.insert("score".to_owned(), FieldSelectMeta::scalar(ScalarFilter::Number));
        let mut meta = BTreeMap::new();
        meta.insert("user".to_owned(), user);
        meta.insert("post".to_owned(), post);
        meta.insert("likes".to_owned(), likes);
        bind_schema_meta(meta, BTreeMap::new());

        let mut id_ctx = WhereCtx::new();
        assert_eq!(sql(&json!({"id": "user:1"}), "user", &mut id_ctx), "id = type::record($w0)");
        assert_eq!(id_ctx.vars.get("w0"), Some(&json!("user:1")));

        let mut bare = WhereCtx::new();
        assert_eq!(sql(&json!({"id": "1"}), "user", &mut bare), "id = type::record($w0)");
        assert_eq!(bare.vars.get("w0"), Some(&json!("user:1")));

        let mut in_ctx = WhereCtx::new();
        assert_eq!(
            sql(&json!({"id": {"in": ["user:a", "b"]}}), "user", &mut in_ctx),
            "id IN [type::record($w0), type::record($w1)]"
        );
        assert_eq!(in_ctx.vars.get("w0"), Some(&json!("user:a")));
        assert_eq!(in_ctx.vars.get("w1"), Some(&json!("user:b")));

        let mut link = WhereCtx::new();
        assert_eq!(
            sql(&json!({"author": {"id": "user:9"}}), "post", &mut link),
            "author = type::record($w0)"
        );
        assert_eq!(link.vars.get("w0"), Some(&json!("user:9")));

        let mut bare_link = WhereCtx::new();
        assert_eq!(
            sql(&json!({"author": {"id": "9"}}), "post", &mut bare_link),
            "author = type::record($w0)"
        );

        let mut edge = WhereCtx::new();
        assert_eq!(
            sql(&json!({"out": {"id": "post:2"}}), "likes", &mut edge),
            "out = type::record($w0)"
        );

        let mut scalar = WhereCtx::new();
        assert_eq!(sql(&json!({"email": "a@b.c"}), "user", &mut scalar), "email = $w0");
        assert_eq!(scalar.vars.get("w0"), Some(&json!("a@b.c")));
    }

    #[test]
    fn compiles_nested_object_filters() {
        let _guard = lock_meta();
        let mut fields = BTreeMap::new();
        fields.insert("source".to_owned(), FieldSelectMeta::scalar(ScalarFilter::String));
        fields.insert("user_id".to_owned(), FieldSelectMeta::scalar(ScalarFilter::Number));
        let mut user = BTreeMap::new();
        user.insert("metadata".to_owned(), FieldSelectMeta::Object { fields });
        user.insert("profile".to_owned(), FieldSelectMeta::scalar(ScalarFilter::Json));
        user.insert("tags".to_owned(), FieldSelectMeta::scalar(ScalarFilter::Array));
        let mut meta = BTreeMap::new();
        meta.insert("user".to_owned(), user);
        bind_schema_meta(meta, BTreeMap::new());

        let mut nested = WhereCtx::new();
        assert_eq!(
            sql(&json!({"metadata": {"source": "where-e2e"}}), "user", &mut nested),
            "metadata.source = $w0"
        );
        assert_eq!(nested.vars.get("w0"), Some(&json!("where-e2e")));

        let mut contains = WhereCtx::new();
        assert_eq!(
            sql(&json!({"metadata": {"source": {"contains": "web"}}}), "user", &mut contains),
            "string::contains(metadata.source, $w0)"
        );

        let mut whole = WhereCtx::new();
        let value = json!({"source": "where-e2e", "user_id": 3});
        assert_eq!(
            sql(&json!({"metadata": {"equals": value}}), "user", &mut whole),
            "metadata = $w0"
        );
        assert_eq!(whole.vars.get("w0"), Some(&value));

        let mut tags = WhereCtx::new();
        assert_eq!(
            sql(&json!({"tags": {"contains": "filter"}}), "user", &mut tags),
            "tags CONTAINS $w0"
        );

        let mut bare = WhereCtx::new();
        assert_eq!(sql(&json!({"profile": {"foo": 1}}), "user", &mut bare), "profile = $w0");

        let mut ignored = WhereCtx::new();
        assert_eq!(
            sql(&json!({"metadata": {"extra": true, "source": "a"}}), "user", &mut ignored),
            "metadata.source = $w0"
        );
    }

    #[test]
    fn compiles_object_filters_in_group_having() {
        let _guard = lock_meta();
        let mut fields = BTreeMap::new();
        fields.insert("source".to_owned(), FieldSelectMeta::scalar(ScalarFilter::String));
        let mut user = BTreeMap::new();
        user.insert("metadata".to_owned(), FieldSelectMeta::Object { fields });
        let mut meta = BTreeMap::new();
        meta.insert("user".to_owned(), user);
        bind_schema_meta(meta, BTreeMap::new());
        let mut ctx = WhereCtx::new();
        let clause = build_group_having(
            "user",
            Some(&json!({"metadata": {"source": "web"}})),
            &["metadata".to_owned()],
            &[],
            &mut ctx,
        )
        .expect("having");
        assert_eq!(clause, "metadata.source = $w0");
        assert_eq!(ctx.vars.get("w0"), Some(&json!("web")));
    }

    #[test]
    fn nested_select_take_orders_by_id() {
        let _guard = lock_meta();
        let mut user = BTreeMap::new();
        user.insert(
            "posts".to_owned(),
            FieldSelectMeta::Computed { target_table: "post".to_owned(), list: true },
        );
        let mut post = BTreeMap::new();
        post.insert("id".to_owned(), FieldSelectMeta::scalar(ScalarFilter::Id));
        let mut meta = BTreeMap::new();
        meta.insert("user".to_owned(), user);
        meta.insert("post".to_owned(), post);
        bind_schema_meta(meta.clone(), BTreeMap::new());
        let projection = build_projection_with(
            Some(&json!({"posts": {"select": {"id": true}, "take": 1}})),
            "user",
            &meta,
        )
        .expect("projection");
        assert!(projection.contains("ORDER BY id"), "{projection}");
        assert!(projection.contains("LIMIT 1"), "{projection}");
        let mut vars = Map::new();
        let sql = append_limit_start("SELECT * FROM user", Some(&json!(3)), None, &mut vars)
            .expect("limit");
        assert!(sql.contains("LIMIT $__take"), "{sql}");
        let _ = RelationDir::Out;
    }
}
