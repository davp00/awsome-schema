//! Record id helpers shared with generated clients.

/// `table:key` split of a record id string.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ParsedRecordId {
    /// Table name.
    pub table: String,
    /// Id key, without the table prefix.
    pub key: String,
}

/// Build `table:key`.
#[must_use]
pub fn record_id(table: &str, key: &str) -> String {
    format!("{table}:{key}")
}

/// Split `table:key`. Returns `None` when there is no table prefix.
#[must_use]
pub fn parse_record_id(value: &str) -> Option<ParsedRecordId> {
    let index = value.find(':')?;
    if index == 0 {
        return None;
    }
    Some(ParsedRecordId { table: value[..index].to_owned(), key: value[index + 1..].to_owned() })
}

/// Prefix `value` with `table:` when it is not already a thing id.
#[must_use]
pub fn as_record_id(table: &str, value: &str) -> String {
    if value.contains(':') { value.to_owned() } else { record_id(table, value) }
}

/// Same as [`as_record_id`].
#[must_use]
pub fn normalize_thing(table: &str, id: &str) -> String {
    as_record_id(table, id)
}
