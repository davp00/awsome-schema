//! JSON helpers for the query builders.

use serde_json::{Map, Value};

pub type JsonMap = Map<String, Value>;

pub(crate) fn entries(map: &JsonMap) -> Vec<(&str, &Value)> {
    map.iter().map(|(key, value)| (key.as_str(), value)).collect()
}

pub(crate) fn as_rows(result: Value) -> Vec<Value> {
    match result {
        Value::Array(items) => match items.first() {
            Some(Value::Array(_)) => {
                if let Some(Value::Array(inner)) = items.into_iter().next() {
                    inner
                } else {
                    Vec::new()
                }
            }
            _ => items,
        },
        Value::Null => Vec::new(),
        other => vec![other],
    }
}

pub(crate) fn first_row(result: Value) -> Option<Value> {
    match result {
        Value::Array(items) => {
            let mut items = items.into_iter();
            let first = items.next()?;
            match first {
                Value::Array(inner) => inner.into_iter().next(),
                other => Some(other),
            }
        }
        Value::Null => None,
        other => Some(other),
    }
}

pub(crate) fn id_string(value: &Value) -> Option<String> {
    match value {
        Value::String(text) => Some(text.clone()),
        Value::Object(map) => map.get("id").and_then(id_string),
        _ => None,
    }
}

pub(crate) fn row_id(row: &Value) -> Option<String> {
    row.as_object().and_then(|map| map.get("id")).and_then(id_string)
}

#[allow(clippy::cast_precision_loss)]
pub(crate) fn number_of(value: &Value) -> Option<f64> {
    value
        .as_f64()
        .or_else(|| value.as_i64().map(|number| number as f64))
        .or_else(|| value.as_u64().map(|number| number as f64))
}
