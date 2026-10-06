//! Map SurrealDB `INFO` define strings into the domain model.

use std::collections::{BTreeMap};

use core::{
    DatabaseSchema, DomainError, Edge, Field, FieldType, Index, Model, NamingConvention, TableMode,
};

/// Per-table payload from `INFO FOR TABLE`.
#[derive(Debug, Clone, Default)]
pub struct TableInfo {
    pub fields: BTreeMap<String, String>,
    pub indexes: BTreeMap<String, String>,
}

/// Build models and edges from database INFO maps.
pub fn map_database_info(
    table_defines: &BTreeMap<String, String>,
    table_infos: &BTreeMap<String, TableInfo>,
    preserve: &DatabaseSchema,
) -> Result<DatabaseSchema, DomainError> {
    let mut models = Vec::new();
    let mut edges = Vec::new();

    for (table_name, table_define) in table_defines {
        let info = table_infos.get(table_name).cloned().unwrap_or_default();
        if is_relation_table(table_define) {
            let (in_table, out_table) = parse_relation_endpoints(table_define)?;
            let domain_name = edge_name_for_table(table_name, preserve);
            let edge = Edge {
                name: domain_name,
                in_model: model_name_for_table(&in_table, preserve),
                out_model: model_name_for_table(&out_table, preserve),
                fields: map_fields(table_name, &info, preserve, None)?,
                table_mode: parse_table_mode(table_define),
                permissions: parse_permissions(table_define),
                attributes: BTreeMap::new(),
            };
            edges.push(edge);
        } else {
            let domain_name = model_name_for_table(table_name, preserve);
            let mut fields = map_fields(table_name, &info, preserve, Some(table_name))?;
            let mut indexes = map_indexes(table_name, &info, &mut fields, preserve)?;
            indexes.sort_by(|a, b| a.resolved_name(table_name, &preserve.naming).cmp(&b.resolved_name(table_name, &preserve.naming)));

            models.push(Model {
                name: domain_name,
                fields,
                table_mode: parse_table_mode(table_define),
                permissions: parse_permissions(table_define),
                indexes,
                attributes: BTreeMap::new(),
            });
        }
    }

    models.sort_by(|a, b| a.name.cmp(&b.name));
    edges.sort_by(|a, b| a.name.cmp(&b.name));

    Ok(DatabaseSchema {
        datasource: DatabaseSchema::empty().datasource,
        naming: NamingConvention::default(),
        generators: Vec::new(),
        object_types: Vec::new(),
        models,
        edges,
    })
}

fn is_relation_table(define: &str) -> bool {
    normalize(define).contains(" type relation ")
}

fn parse_relation_endpoints(define: &str) -> Result<(String, String), DomainError> {
    let tokens = tokenize(define);
    let in_idx = tokens.iter().position(|t| t.eq_ignore_ascii_case("in")).ok_or_else(|| {
        DomainError::DatabaseError(format!("relation table define missing IN: {define}"))
    })?;
    let out_idx = tokens.iter().position(|t| t.eq_ignore_ascii_case("out")).ok_or_else(|| {
        DomainError::DatabaseError(format!("relation table define missing OUT: {define}"))
    })?;
    let in_table = tokens.get(in_idx + 1).ok_or_else(|| {
        DomainError::DatabaseError(format!("relation table define missing IN target: {define}"))
    })?;
    let out_table = tokens.get(out_idx + 1).ok_or_else(|| {
        DomainError::DatabaseError(format!("relation table define missing OUT target: {define}"))
    })?;
    Ok((in_table.clone(), out_table.clone()))
}

fn parse_table_mode(define: &str) -> TableMode {
    let normalized = normalize(define);
    if normalized.contains(" schemaless") {
        TableMode::Schemaless
    } else {
        TableMode::Schemafull
    }
}

fn parse_permissions(define: &str) -> Option<String> {
    let tokens = tokenize(define);
    let pos = tokens.iter().position(|t| t.eq_ignore_ascii_case("permissions"))?;
    let rest = tokens.get(pos + 1..)?.join(" ");
    if rest.is_empty() { None } else { Some(rest) }
}

fn map_fields(
    table: &str,
    info: &TableInfo,
    preserve: &DatabaseSchema,
    id_table: Option<&str>,
) -> Result<Vec<Field>, DomainError> {
    let mut fields = Vec::new();
    for define in info.fields.values() {
        let mut field = parse_field_define(define, table, preserve)?;
        if field.name == "id" && id_table.is_some_and(|t| t == table) {
            field.is_id = true;
        }
        fields.push(field);
    }
    fields.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(fields)
}

fn map_indexes(
    table: &str,
    info: &TableInfo,
    fields: &mut [Field],
    preserve: &DatabaseSchema,
) -> Result<Vec<Index>, DomainError> {
    let mut indexes = Vec::new();
    for define in info.indexes.values() {
        let index = parse_index_define(define)?;
        if index.fields.len() == 1 && index.unique {
            let field_name = &index.fields[0];
            let expected = format!("{table}_{field_name}_unique");
            if index.resolved_name(table, &preserve.naming) == expected
                || index.name.as_deref() == Some(expected.as_str())
            {
                if let Some(field) = fields.iter_mut().find(|f| f.name == *field_name) {
                    field.unique = true;
                    continue;
                }
            }
        }
        indexes.push(index);
    }
    Ok(indexes)
}

fn parse_field_define(
    define: &str,
    table: &str,
    preserve: &DatabaseSchema,
) -> Result<Field, DomainError> {
    let tokens = tokenize(define);
    if tokens.len() < 6 || !tokens[0].eq_ignore_ascii_case("define") || !tokens[1].eq_ignore_ascii_case("field") {
        return Err(DomainError::DatabaseError(format!("unexpected field define: {define}")));
    }
    let name = tokens[2].clone();
    let on_pos = tokens.iter().position(|t| t.eq_ignore_ascii_case("on")).ok_or_else(|| {
        DomainError::DatabaseError(format!("field define missing ON: {define}"))
    })?;
    if tokens.get(on_pos + 1).map(String::as_str) != Some(table) {
        return Err(DomainError::DatabaseError(format!(
            "field define table mismatch: expected `{table}` in `{define}`"
        )));
    }
    let type_pos = tokens.iter().position(|t| t.eq_ignore_ascii_case("type")).ok_or_else(|| {
        DomainError::DatabaseError(format!("field define missing TYPE: {define}"))
    })?;

    let mut idx = type_pos + 1;
    let (field_type, optional, flexible) = parse_type_tokens(&tokens, &mut idx)?;
    let mut default_value = None;
    let mut default_always = false;
    let mut value_expression = None;
    let mut readonly = false;

    while idx < tokens.len() {
        let token = tokens[idx].to_ascii_lowercase();
        match token.as_str() {
            "default" => {
                idx += 1;
                if tokens.get(idx).is_some_and(|t| t.eq_ignore_ascii_case("always")) {
                    default_always = true;
                    idx += 1;
                }
                let (value, next) = read_expression(&tokens, idx);
                default_value = Some(value);
                idx = next;
            }
            "value" => {
                idx += 1;
                let (value, next) = read_expression(&tokens, idx);
                value_expression = Some(value);
                idx = next;
            }
            "readonly" => {
                readonly = true;
                idx += 1;
            }
            "flexible" => {
                idx += 1;
            }
            "assert" | "permissions" => {
                let (_, next) = read_expression(&tokens, idx + 1);
                idx = next;
            }
            _ => idx += 1,
        }
    }

    let (mut field_type, mut link_target) = map_record_links(field_type, preserve);
    if name == "id" {
        if let FieldType::Model(model) = field_type {
            field_type = FieldType::RecordId(model);
            link_target = None;
        }
    }

    Ok(Field {
        name,
        field_type,
        optional,
        unique: false,
        is_id: false,
        default_value,
        default_always,
        value_expression,
        readonly,
        flexible,
        link_target,
        relation_name: None,
        attributes: BTreeMap::new(),
    })
}

fn parse_index_define(define: &str) -> Result<Index, DomainError> {
    let tokens = tokenize(define);
    if tokens.len() < 8 {
        return Err(DomainError::DatabaseError(format!("unexpected index define: {define}")));
    }
    let name = Some(tokens[2].clone());
    let fields_pos = tokens.iter().position(|t| t.eq_ignore_ascii_case("fields")).ok_or_else(|| {
        DomainError::DatabaseError(format!("index define missing FIELDS: {define}"))
    })?;
    let mut fields = Vec::new();
    let mut idx = fields_pos + 1;
    while idx < tokens.len() {
        let token = &tokens[idx];
        if token.eq_ignore_ascii_case("unique")
            || token.eq_ignore_ascii_case("fulltext")
            || token.eq_ignore_ascii_case("vector")
        {
            break;
        }
        fields.push(token.clone());
        idx += 1;
    }
    let unique = tokens.iter().any(|t| t.eq_ignore_ascii_case("unique"));
    let fulltext = tokens.iter().any(|t| t.eq_ignore_ascii_case("fulltext"));
    let vector = tokens.iter().any(|t| t.eq_ignore_ascii_case("vector"));
    Ok(Index { name, fields, unique, fulltext, vector })
}

fn parse_type_tokens(
    tokens: &[String],
    idx: &mut usize,
) -> Result<(FieldType, bool, bool), DomainError> {
    let raw = tokens.get(*idx).ok_or_else(|| DomainError::DatabaseError("missing type".into()))?;
    *idx += 1;
    let (field_type, optional) = if raw.starts_with("option<") && raw.ends_with('>') {
        (parse_inner_option_type(raw)?, true)
    } else {
        (parse_type_name(raw)?, false)
    };
    let mut flexible = false;
    while *idx < tokens.len() {
        let token = tokens[*idx].to_ascii_lowercase();
        if token == "flexible" {
            flexible = true;
            *idx += 1;
        } else {
            break;
        }
    }
    Ok((field_type, optional, flexible))
}

fn parse_type_name(raw: &str) -> Result<FieldType, DomainError> {
    Ok(match raw {
        "string" => FieldType::String,
        "int" => FieldType::Int,
        "float" => FieldType::Float,
        "bool" => FieldType::Bool,
        "datetime" => FieldType::Datetime,
        "object" => FieldType::Object,
        other if other.starts_with("array<") && other.ends_with('>') => {
            let inner = &other[6..other.len() - 1];
            FieldType::Array(Box::new(map_scalar_type_name(inner)))
        }
        other if other.starts_with("record<") && other.ends_with('>') => {
            let inner = &other[7..other.len() - 1];
            FieldType::RecordId(inner.to_owned())
        }
        other => FieldType::Custom(other.to_owned()),
    })
}

fn parse_inner_option_type(raw: &str) -> Result<FieldType, DomainError> {
    let inner = raw.strip_prefix("option<").and_then(|s| s.strip_suffix('>')).ok_or_else(|| {
        DomainError::DatabaseError(format!("invalid option type `{raw}`"))
    })?;
    Ok(map_scalar_type_name(inner))
}

fn map_scalar_type_name(name: &str) -> FieldType {
    match name {
        "string" => FieldType::String,
        "int" => FieldType::Int,
        "float" => FieldType::Float,
        "bool" => FieldType::Bool,
        "datetime" => FieldType::Datetime,
        "object" => FieldType::Object,
        other if other.starts_with("array<") => {
            let inner = &other[6..other.len() - 1];
            FieldType::Array(Box::new(map_scalar_type_name(inner)))
        }
        other if other.starts_with("record<") => {
            let inner = &other[7..other.len() - 1];
            FieldType::RecordId(inner.to_owned())
        }
        other => FieldType::Custom(other.to_owned()),
    }
}

fn map_record_links(field_type: FieldType, preserve: &DatabaseSchema) -> (FieldType, Option<String>) {
    match field_type {
        FieldType::RecordId(target) => {
            let model = model_name_for_table(&target, preserve);
            (FieldType::Model(model.clone()), Some(model))
        }
        other => (other, None),
    }
}

fn model_name_for_table(table: &str, preserve: &DatabaseSchema) -> String {
    for model in &preserve.models {
        if model.table_name(&preserve.naming) == table {
            return model.name.clone();
        }
    }
    for edge in &preserve.edges {
        if edge.table_name(&preserve.naming) == table {
            return edge.name.clone();
        }
    }
    table_name_to_pascal(table)
}

fn edge_name_for_table(table: &str, preserve: &DatabaseSchema) -> String {
    for edge in &preserve.edges {
        if edge.table_name(&preserve.naming) == table {
            return edge.name.clone();
        }
    }
    table_name_to_pascal(table)
}

fn table_name_to_pascal(table: &str) -> String {
    table
        .split('_')
        .filter(|part| !part.is_empty())
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_ascii_uppercase().to_string() + chars.as_str(),
            }
        })
        .collect()
}

fn read_expression(tokens: &[String], start: usize) -> (String, usize) {
    if start >= tokens.len() {
        return (String::new(), start);
    }
    let mut parts = Vec::new();
    let mut idx = start;
    while idx < tokens.len() {
        let lower = tokens[idx].to_ascii_lowercase();
        if matches!(
            lower.as_str(),
            "default" | "value" | "readonly" | "flexible" | "assert" | "permissions" | "type"
        ) && idx != start
        {
            break;
        }
        parts.push(tokens[idx].clone());
        idx += 1;
    }
    (parts.join(" "), idx)
}

fn normalize(input: &str) -> String {
    format!(" {} ", input.trim().trim_end_matches(';').to_ascii_lowercase())
}

fn tokenize(input: &str) -> Vec<String> {
    input
        .trim()
        .trim_end_matches(';')
        .split_whitespace()
        .map(ToOwned::to_owned)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    fn preserve_with_user() -> DatabaseSchema {
        let mut schema = DatabaseSchema::empty();
        schema.datasource.provider = "surrealdb".into();
        schema.naming.tables = core::NamingCase::SnakeCase;
        schema.models.push(Model {
            name: "User".into(),
            fields: vec![],
            table_mode: TableMode::Schemafull,
            permissions: None,
            indexes: vec![],
            attributes: BTreeMap::new(),
        });
        schema
    }

    #[test]
    fn maps_user_table_from_fixture() {
        let preserve = preserve_with_user();
        let mut tables = BTreeMap::new();
        tables.insert(
            "user".into(),
            "DEFINE TABLE user SCHEMAFULL;".into(),
        );
        let mut info = TableInfo::default();
        info.fields.insert(
            "email".into(),
            "DEFINE FIELD email ON user TYPE string;".into(),
        );
        info.fields.insert(
            "id".into(),
            "DEFINE FIELD id ON user TYPE record<user>;".into(),
        );
        info.indexes.insert(
            "user_email_unique".into(),
            "DEFINE INDEX user_email_unique ON user FIELDS email UNIQUE;".into(),
        );
        let mut table_infos = BTreeMap::new();
        table_infos.insert("user".into(), info);

        let pulled = map_database_info(&tables, &table_infos, &preserve).expect("map");
        assert_eq!(pulled.models.len(), 1);
        let user = &pulled.models[0];
        assert_eq!(user.name, "User");
        let email = user.fields.iter().find(|f| f.name == "email").unwrap();
        assert!(email.unique);
        let id = user.fields.iter().find(|f| f.name == "id").unwrap();
        assert!(id.is_id);
    }
}
