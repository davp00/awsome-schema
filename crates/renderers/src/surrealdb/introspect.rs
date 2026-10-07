//! Map SurrealDB `INFO` define strings into the domain model.

use std::collections::{BTreeMap};

use core::{
    DatabaseSchema, DomainError, Edge, Field, FieldType, Index, Model, NamingConvention, TableMode,
    VectorDist,
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
        // Skip internal ledger / system tables.
        if table_name.starts_with('_') {
            continue;
        }
        let info = table_infos.get(table_name).cloned().unwrap_or_default();
        if is_relation_table(table_define) {
            let (in_table, out_table) = parse_relation_endpoints(table_define)?;
            let domain_name = edge_name_for_table(table_name, preserve);
            let mut fields = map_fields(table_name, &info, preserve, None)?;
            // RELATION tables expose `in`/`out` as fields in INFO; those are the edge
            // endpoints, not DSL fields — keep only payload fields (e.g. score).
            fields.retain(|field| field.name != "in" && field.name != "out");
            let edge = Edge {
                name: domain_name,
                in_model: model_name_for_table(&in_table, preserve),
                out_model: model_name_for_table(&out_table, preserve),
                fields,
                table_mode: parse_table_mode(table_define),
                permissions: parse_permissions(table_define),
                attributes: BTreeMap::new(),
            };
            edges.push(edge);
        } else {
            let domain_name = model_name_for_table(table_name, preserve);
            let mut fields = map_fields(table_name, &info, preserve, Some(table_name))?;
            ensure_model_id_field(&mut fields, &domain_name);
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

    let mut schema = DatabaseSchema {
        datasource: DatabaseSchema::empty().datasource,
        naming: NamingConvention::default(),
        generators: Vec::new(),
        object_types: Vec::new(),
        models,
        edges,
    };
    pair_pulled_links(&mut schema, preserve);
    Ok(schema)
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
    if rest.is_empty() || rest.eq_ignore_ascii_case("none") {
        None
    } else {
        Some(rest)
    }
}

/// SurrealDB 3.3 `INFO FOR TABLE` omits the primary `id` field even when it was
/// defined as `TYPE record<table>`. Models still require `@id`, so synthesize it.
fn ensure_model_id_field(fields: &mut Vec<Field>, model_name: &str) {
    if let Some(field) = fields.iter_mut().find(|field| field.name == "id") {
        // `parse_field_define` already maps `record<table>` id fields to RecordId.
        field.is_id = true;
        return;
    }

    fields.push(Field {
        name: "id".into(),
        field_type: FieldType::RecordId(model_name.to_owned()),
        optional: false,
        unique: false,
        is_id: true,
        default_value: None,
        default_always: false,
        value_expression: None,
        readonly: false,
        flexible: false,
        link_target: None,
        link_name: None,
        on_delete: None,
        link_storage: None,
        link_opposite_field: None,
        relation_name: None,
        attributes: BTreeMap::new(),
    });
    fields.sort_by(|a, b| a.name.cmp(&b.name));
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
        // Surreal INFO emits array-item schemas as `tags.*` — not valid DSL field paths.
        if field.name.contains(".*") || field.name.ends_with('*') {
            continue;
        }
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
            if index.resolved_name(table, &preserve.naming) == expected {
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

    if let Some(computed) = parse_computed_backlink(define, preserve) {
        let field_type = FieldType::Array(Box::new(FieldType::Model(computed.target_model.clone())));
        return Ok(Field {
            name,
            field_type,
            optional: false,
            unique: false,
            is_id: false,
            default_value: None,
            default_always: false,
            value_expression: None,
            readonly: false,
            flexible: false,
            link_target: Some(computed.target_model),
            link_name: None,
            on_delete: None,
            link_storage: Some(core::LinkStorage::Computed),
            link_opposite_field: if computed.opposite_field.is_empty() {
                None
            } else {
                Some(computed.opposite_field)
            },
            relation_name: None,
            attributes: BTreeMap::new(),
        });
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
    let mut on_delete = None;

    while idx < tokens.len() {
        let token = tokens[idx].to_ascii_lowercase();
        match token.as_str() {
            "default" => {
                idx += 1;
                default_always = tokens.get(idx).is_some_and(|t| t.eq_ignore_ascii_case("always"));
                if default_always {
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
            "reference" => {
                idx += 1;
            }
            "on" => {
                if tokens.get(idx + 1).is_some_and(|t| t.eq_ignore_ascii_case("delete")) {
                    idx += 2;
                    let action = tokens.get(idx).map(String::as_str).unwrap_or("IGNORE");
                    on_delete = core::OnDeleteAction::parse(action);
                    idx += 1;
                } else {
                    idx += 1;
                }
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
        // INFO `record<table>` id fields become Model via map_record_links; restore RecordId.
        field_type = match field_type {
            FieldType::Model(model) => FieldType::RecordId(model),
            other => other,
        };
        link_target = None;
    }

    // RECORD links map to @link; REFERENCE adds storage + on_delete
    let (link_storage, on_delete) = if link_target.is_some() {
        (
            Some(core::LinkStorage::Stored),
            Some(on_delete.unwrap_or(core::OnDeleteAction::Ignore)),
        )
    } else {
        (None, None)
    };

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
        link_name: None,
        on_delete,
        link_storage,
        link_opposite_field: None,
        relation_name: None,
        attributes: BTreeMap::new(),
    })
}

struct ComputedBacklink {
    target_model: String,
    opposite_field: String,
}

fn parse_computed_backlink(define: &str, preserve: &DatabaseSchema) -> Option<ComputedBacklink> {
    let lower = define.to_ascii_lowercase();
    let computed_at = lower.find("computed")?;
    let after = define[computed_at + "computed".len()..].trim();
    let after = after.trim_end_matches(';').trim();
    if !after.contains("<~") {
        return None;
    }
    let start = after.find("<~")?;
    let mut body = after[start + 2..].trim();
    body = body.trim_start_matches('(').trim_end_matches(')').trim();
    // table FIELD field  OR  table
    let parts: Vec<&str> = body.split_whitespace().collect();
    if parts.len() >= 3 && parts[1].eq_ignore_ascii_case("field") {
        return Some(ComputedBacklink {
            target_model: model_name_for_table(parts[0], preserve),
            opposite_field: parts[2].trim_matches(|c| c == ')' || c == ';').to_owned(),
        });
    }
    if parts.len() == 1 {
        return Some(ComputedBacklink {
            target_model: model_name_for_table(parts[0], preserve),
            opposite_field: String::new(),
        });
    }
    None
}

fn pair_pulled_links(schema: &mut DatabaseSchema, preserve: &DatabaseSchema) {
    // Assign shared link_name from computed side → stored opposite
    let mut pairs: Vec<(String, String, String, String)> = Vec::new();
    // (computed_model, computed_field, stored_model, stored_field)
    for model in &schema.models {
        for field in &model.fields {
            if !field.is_computed_link() {
                continue;
            }
            let (Some(target), Some(opposite)) = (&field.link_target, &field.link_opposite_field)
            else {
                continue;
            };
            pairs.push((
                model.name.clone(),
                field.name.clone(),
                target.clone(),
                opposite.clone(),
            ));
        }
    }

    for (computed_model, computed_field, stored_model, stored_field) in pairs {
        let pair_name = preserved_link_name(preserve, &computed_model, &computed_field)
            .or_else(|| preserved_link_name(preserve, &stored_model, &stored_field))
            .unwrap_or_else(|| format!("{stored_model}{computed_model}"));
        // Computed side always exists — pairs are collected from `schema.models`.
        let computed = schema
            .models
            .iter_mut()
            .find(|m| m.name == computed_model)
            .and_then(|m| m.fields.iter_mut().find(|f| f.name == computed_field))
            .expect("computed pair source");
        computed.link_name = Some(pair_name.clone());

        if let Some(field) = schema
            .models
            .iter_mut()
            .find(|m| m.name == stored_model)
            .and_then(|m| m.fields.iter_mut().find(|f| f.name == stored_field))
        {
            field.link_name = Some(pair_name);
            if field.link_storage.is_none() {
                field.link_storage = Some(core::LinkStorage::Stored);
            }
            if field.on_delete.is_none() {
                field.on_delete = Some(core::OnDeleteAction::Ignore);
            }
        }
    }
}

fn preserved_link_name(
    preserve: &DatabaseSchema,
    model_name: &str,
    field_name: &str,
) -> Option<String> {
    preserve
        .models
        .iter()
        .find(|model| model.name == model_name)?
        .fields
        .iter()
        .find(|field| field.name == field_name)?
        .link_name
        .clone()
}

fn parse_index_define(define: &str) -> Result<Index, DomainError> {
    let tokens = tokenize(define);
    // DEFINE INDEX <name> ON <table> FIELDS <field…> [UNIQUE|FULLTEXT|HNSW…]
    if tokens.len() < 7 {
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
            || token.eq_ignore_ascii_case("hnsw")
            || token.eq_ignore_ascii_case("vector")
        {
            break;
        }
        fields.push(token.clone());
        idx += 1;
    }

    let unique = tokens.iter().any(|t| t.eq_ignore_ascii_case("unique"));
    let fulltext = tokens.iter().any(|t| t.eq_ignore_ascii_case("fulltext"));
    let vector = tokens.iter().any(|t| t.eq_ignore_ascii_case("hnsw"));

    let mut fulltext_analyzer = None;
    if fulltext {
        if let Some(analyzer_pos) = tokens.iter().position(|t| t.eq_ignore_ascii_case("analyzer")) {
            if let Some(analyzer) = tokens.get(analyzer_pos + 1) {
                fulltext_analyzer = Some(analyzer.clone());
            }
        }
    }

    let mut vector_dimension = None;
    let mut vector_dist = None;
    if vector {
        if let Some(dim_pos) = tokens.iter().position(|t| t.eq_ignore_ascii_case("dimension")) {
            if let Some(raw) = tokens.get(dim_pos + 1) {
                vector_dimension = raw.parse::<u32>().ok();
            }
        }
        if let Some(dist_pos) = tokens.iter().position(|t| t.eq_ignore_ascii_case("dist")) {
            if let Some(raw) = tokens.get(dist_pos + 1) {
                vector_dist = VectorDist::parse(raw);
            }
        }
    }

    Ok(Index {
        name,
        fields,
        unique,
        fulltext,
        fulltext_analyzer,
        vector,
        vector_dimension,
        vector_dist,
    })
}

fn parse_type_tokens(
    tokens: &[String],
    idx: &mut usize,
) -> Result<(FieldType, bool, bool), DomainError> {
    let raw = tokens.get(*idx).ok_or_else(|| DomainError::DatabaseError("missing type".into()))?;
    *idx += 1;

    // SurrealDB 3.3 INFO often emits optionals as `none | T` (or `T | none`).
    if raw.eq_ignore_ascii_case("none")
        && tokens.get(*idx).is_some_and(|t| t == "|")
        && tokens.get(*idx + 1).is_some()
    {
        *idx += 1; // |
        let inner = &tokens[*idx];
        *idx += 1;
        let field_type = parse_type_name(inner)?;
        return Ok((field_type, true, false));
    }

    let (field_type, mut optional) = if raw.starts_with("option<") && raw.ends_with('>') {
        (parse_inner_option_type(raw), true)
    } else {
        (parse_type_name(raw)?, false)
    };

    if tokens.get(*idx).is_some_and(|t| t == "|")
        && tokens.get(*idx + 1).is_some_and(|t| t.eq_ignore_ascii_case("none"))
    {
        *idx += 2;
        optional = true;
    }

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

/// Caller must pass a token that already matches `option<...>`.
fn parse_inner_option_type(raw: &str) -> FieldType {
    let inner = raw
        .strip_prefix("option<")
        .and_then(|s| s.strip_suffix('>'))
        .unwrap_or(raw);
    map_scalar_type_name(inner)
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
            // `filter` above guarantees a non-empty part.
            let first = chars.next().expect("non-empty split part");
            first.to_ascii_uppercase().to_string() + chars.as_str()
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
    fn parses_plain_index_without_unique_flag() {
        let index = parse_index_define("DEFINE INDEX post_title_idx ON post FIELDS title")
            .expect("plain index");
        assert_eq!(index.name.as_deref(), Some("post_title_idx"));
        assert_eq!(index.fields, vec!["title".to_owned()]);
        assert!(!index.unique);
        assert!(!index.fulltext);
        assert!(!index.vector);
    }

    #[test]
    fn parses_none_pipe_optional_types() {
        let tokens = tokenize("DEFINE FIELD age ON user TYPE none | int PERMISSIONS FULL");
        let type_pos = tokens.iter().position(|t| t.eq_ignore_ascii_case("type")).unwrap();
        let mut idx = type_pos + 1;
        let (field_type, optional, _) = parse_type_tokens(&tokens, &mut idx).expect("type");
        assert_eq!(field_type, FieldType::Int);
        assert!(optional);
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

    #[test]
    fn synthesizes_id_when_info_omits_it() {
        let preserve = preserve_with_user();
        let mut tables = BTreeMap::new();
        tables.insert("user".into(), "DEFINE TABLE user SCHEMAFULL PERMISSIONS NONE;".into());
        let mut info = TableInfo::default();
        info.fields.insert(
            "email".into(),
            "DEFINE FIELD email ON user TYPE string;".into(),
        );
        let mut table_infos = BTreeMap::new();
        table_infos.insert("user".into(), info);

        let pulled = map_database_info(&tables, &table_infos, &preserve).expect("map");
        let user = &pulled.models[0];
        let id = user.fields.iter().find(|f| f.name == "id").expect("synthesized id");
        assert!(id.is_id);
        assert_eq!(id.field_type, FieldType::RecordId("User".into()));
        assert!(user.permissions.is_none());
    }

    #[test]
    fn maps_fulltext_and_hnsw_indexes() {
        let preserve = preserve_with_user();
        let mut tables = BTreeMap::new();
        tables.insert("doc".into(), "DEFINE TABLE doc SCHEMAFULL;".into());
        let mut info = TableInfo::default();
        info.fields.insert(
            "id".into(),
            "DEFINE FIELD id ON doc TYPE record<doc>;".into(),
        );
        info.fields.insert(
            "title".into(),
            "DEFINE FIELD title ON doc TYPE string;".into(),
        );
        info.fields.insert(
            "embedding".into(),
            "DEFINE FIELD embedding ON doc TYPE array<float>;".into(),
        );
        info.indexes.insert(
            "doc_title_idx".into(),
            "DEFINE INDEX doc_title_idx ON doc FIELDS title FULLTEXT ANALYZER english BM25;".into(),
        );
        info.indexes.insert(
            "doc_embedding_idx".into(),
            "DEFINE INDEX doc_embedding_idx ON doc FIELDS embedding HNSW DIMENSION 1536 DIST COSINE;"
                .into(),
        );
        let mut table_infos = BTreeMap::new();
        table_infos.insert("doc".into(), info);

        let pulled = map_database_info(&tables, &table_infos, &preserve).expect("map");
        let doc = &pulled.models[0];
        let fulltext = doc
            .indexes
            .iter()
            .find(|i| i.fulltext)
            .expect("fulltext index");
        assert_eq!(fulltext.fulltext_analyzer.as_deref(), Some("english"));
        let vector = doc.indexes.iter().find(|i| i.vector).expect("vector index");
        assert_eq!(vector.vector_dimension, Some(1536));
        assert_eq!(vector.vector_dist, Some(VectorDist::Cosine));
    }

    #[test]
    fn maps_reference_and_computed_pair() {
        let mut preserve = DatabaseSchema::empty();
        preserve.datasource.provider = "surrealdb".into();
        preserve.naming.tables = core::NamingCase::SnakeCase;
        preserve.models.push(Model {
            name: "User".into(),
            fields: vec![Field {
                name: "posts".into(),
                field_type: FieldType::Array(Box::new(FieldType::Model("Post".into()))),
                optional: false,
                unique: false,
                is_id: false,
                default_value: None,
                default_always: false,
                value_expression: None,
                readonly: false,
                flexible: false,
                link_target: Some("Post".into()),
                link_name: Some("PostAuthor".into()),
                on_delete: None,
                link_storage: Some(core::LinkStorage::Computed),
                link_opposite_field: Some("author".into()),
                relation_name: None,
                attributes: BTreeMap::new(),
            }],
            table_mode: TableMode::Schemafull,
            permissions: None,
            indexes: vec![],
            attributes: BTreeMap::new(),
        });
        preserve.models.push(Model {
            name: "Post".into(),
            fields: vec![Field {
                name: "author".into(),
                field_type: FieldType::Model("User".into()),
                optional: false,
                unique: false,
                is_id: false,
                default_value: None,
                default_always: false,
                value_expression: None,
                readonly: false,
                flexible: false,
                link_target: Some("User".into()),
                link_name: Some("PostAuthor".into()),
                on_delete: Some(core::OnDeleteAction::Cascade),
                link_storage: Some(core::LinkStorage::Stored),
                link_opposite_field: None,
                relation_name: None,
                attributes: BTreeMap::new(),
            }],
            table_mode: TableMode::Schemafull,
            permissions: None,
            indexes: vec![],
            attributes: BTreeMap::new(),
        });

        let mut tables = BTreeMap::new();
        tables.insert("user".into(), "DEFINE TABLE user SCHEMAFULL;".into());
        tables.insert("post".into(), "DEFINE TABLE post SCHEMAFULL;".into());

        let mut user_info = TableInfo::default();
        user_info.fields.insert(
            "id".into(),
            "DEFINE FIELD id ON user TYPE record<user>;".into(),
        );
        user_info.fields.insert(
            "posts".into(),
            "DEFINE FIELD posts ON user COMPUTED <~(post FIELD author);".into(),
        );

        let mut post_info = TableInfo::default();
        post_info.fields.insert(
            "id".into(),
            "DEFINE FIELD id ON post TYPE record<post>;".into(),
        );
        post_info.fields.insert(
            "author".into(),
            "DEFINE FIELD author ON post TYPE record<user> REFERENCE ON DELETE CASCADE;".into(),
        );

        let mut table_infos = BTreeMap::new();
        table_infos.insert("user".into(), user_info);
        table_infos.insert("post".into(), post_info);

        let pulled = map_database_info(&tables, &table_infos, &preserve).expect("map");
        let user = pulled.models.iter().find(|m| m.name == "User").unwrap();
        let post = pulled.models.iter().find(|m| m.name == "Post").unwrap();
        let posts = user.fields.iter().find(|f| f.name == "posts").unwrap();
        let author = post.fields.iter().find(|f| f.name == "author").unwrap();
        assert_eq!(posts.link_storage, Some(core::LinkStorage::Computed));
        assert_eq!(author.link_storage, Some(core::LinkStorage::Stored));
        assert_eq!(author.on_delete, Some(core::OnDeleteAction::Cascade));
        assert_eq!(posts.link_name.as_deref(), Some("PostAuthor"));
        assert_eq!(author.link_name.as_deref(), Some("PostAuthor"));
    }

    #[test]
    fn maps_schemaless_table_mode() {
        let preserve = preserve_with_user();
        let mut tables = BTreeMap::new();
        tables.insert("user".into(), "DEFINE TABLE user SCHEMALESS;".into());
        let mut table_infos = BTreeMap::new();
        table_infos.insert("user".into(), TableInfo::default());

        let pulled = map_database_info(&tables, &table_infos, &preserve).expect("map");
        assert_eq!(pulled.models[0].table_mode, TableMode::Schemaless);
    }

    #[test]
    fn rejects_relation_define_missing_in_out() {
        let preserve = DatabaseSchema::empty();
        let mut tables = BTreeMap::new();
        tables.insert(
            "likes".into(),
            "DEFINE TABLE likes TYPE RELATION SCHEMAFULL;".into(),
        );
        let err = map_database_info(&tables, &BTreeMap::new(), &preserve).expect_err("missing IN");
        assert!(matches!(err, DomainError::DatabaseError(msg) if msg.contains("missing IN")));
    }

    #[test]
    fn maps_field_modifiers_and_trailing_tokens() {
        let preserve = preserve_with_user();
        let mut tables = BTreeMap::new();
        tables.insert("user".into(), "DEFINE TABLE user SCHEMAFULL;".into());
        let mut info = TableInfo::default();
        info.fields.insert(
            "label".into(),
            "DEFINE FIELD label ON user TYPE string DEFAULT 'plain';".into(),
        );
        info.fields.insert(
            "status".into(),
            "DEFINE FIELD status ON user TYPE string DEFAULT ALWAYS 'active' VALUE string::lowercase($value) READONLY FLEXIBLE REFERENCE ON DELETE CASCADE ASSERT $value != NONE PERMISSIONS FULL;".into(),
        );
        info.fields.insert(
            "buddy".into(),
            "DEFINE FIELD buddy ON user TYPE record<user> REFERENCE ON DELETE CASCADE;".into(),
        );
        info.fields.insert(
            "note".into(),
            "DEFINE FIELD note ON user TYPE string ON other VALUE time::now();".into(),
        );
        let mut table_infos = BTreeMap::new();
        table_infos.insert("user".into(), info);

        let pulled = map_database_info(&tables, &table_infos, &preserve).expect("map");
        let user = &pulled.models[0];
        let label = user.fields.iter().find(|f| f.name == "label").unwrap();
        assert!(!label.default_always);
        assert_eq!(label.default_value.as_deref(), Some("'plain'"));
        let status = user.fields.iter().find(|f| f.name == "status").unwrap();
        assert!(status.default_always);
        assert_eq!(status.default_value.as_deref(), Some("'active'"));
        assert_eq!(status.value_expression.as_deref(), Some("string::lowercase($value)"));
        assert!(status.readonly);
        let buddy = user.fields.iter().find(|f| f.name == "buddy").unwrap();
        assert_eq!(buddy.on_delete, Some(core::OnDeleteAction::Cascade));
        assert_eq!(buddy.link_storage, Some(core::LinkStorage::Stored));
        let note = user.fields.iter().find(|f| f.name == "note").unwrap();
        assert_eq!(note.value_expression.as_deref(), Some("time::now()"));
    }

    #[test]
    fn maps_scalar_option_array_record_and_custom_types() {
        let preserve = preserve_with_user();
        let mut tables = BTreeMap::new();
        tables.insert("doc".into(), "DEFINE TABLE doc SCHEMAFULL;".into());
        let mut info = TableInfo::default();
        for (name, define) in [
            ("opt", "DEFINE FIELD opt ON doc TYPE option<string>;"),
            ("pipe", "DEFINE FIELD pipe ON doc TYPE string | none;"),
            ("flag", "DEFINE FIELD flag ON doc TYPE bool;"),
            ("when", "DEFINE FIELD when ON doc TYPE datetime;"),
            ("meta", "DEFINE FIELD meta ON doc TYPE object;"),
            ("tags", "DEFINE FIELD tags ON doc TYPE array<string>;"),
            ("owner", "DEFINE FIELD owner ON doc TYPE record<user>;"),
            ("shape", "DEFINE FIELD shape ON doc TYPE geometry;"),
            ("matrix", "DEFINE FIELD matrix ON doc TYPE array<array<int>>;"),
        ] {
            info.fields.insert(name.into(), define.into());
        }
        let mut table_infos = BTreeMap::new();
        table_infos.insert("doc".into(), info);

        let pulled = map_database_info(&tables, &table_infos, &preserve).expect("map");
        let doc = &pulled.models[0];
        let field = |name: &str| doc.fields.iter().find(|f| f.name == name).unwrap();

        assert!(field("opt").optional);
        assert_eq!(field("opt").field_type, FieldType::String);
        assert!(field("pipe").optional);
        assert_eq!(field("pipe").field_type, FieldType::String);
        assert_eq!(field("flag").field_type, FieldType::Bool);
        assert_eq!(field("when").field_type, FieldType::Datetime);
        assert_eq!(field("meta").field_type, FieldType::Object);
        assert_eq!(
            field("tags").field_type,
            FieldType::Array(Box::new(FieldType::String))
        );
        assert_eq!(field("owner").link_target.as_deref(), Some("User"));
        assert_eq!(field("shape").field_type, FieldType::Custom("geometry".into()));
        assert_eq!(
            field("matrix").field_type,
            FieldType::Array(Box::new(FieldType::Array(Box::new(FieldType::Int))))
        );

        let tokens = tokenize("TYPE option<string>");
        let mut idx = 1;
        let (ty, optional, _) = parse_type_tokens(&tokens, &mut idx).expect("option");
        assert_eq!(ty, FieldType::String);
        assert!(optional);
    }

    #[test]
    fn maps_computed_backlink_single_table_and_field_form() {
        let mut preserve = preserve_with_user();
        preserve.models.push(Model {
            name: "Post".into(),
            fields: vec![],
            table_mode: TableMode::Schemafull,
            permissions: None,
            indexes: vec![],
            attributes: BTreeMap::new(),
        });

        let mut tables = BTreeMap::new();
        tables.insert("user".into(), "DEFINE TABLE user SCHEMAFULL;".into());
        tables.insert("post".into(), "DEFINE TABLE post SCHEMAFULL;".into());

        let mut user_info = TableInfo::default();
        user_info.fields.insert(
            "posts".into(),
            "DEFINE FIELD posts ON user COMPUTED <~ post;".into(),
        );
        user_info.fields.insert(
            "authored".into(),
            "DEFINE FIELD authored ON user COMPUTED <~ (post FIELD author);".into(),
        );
        let mut post_info = TableInfo::default();
        post_info.fields.insert(
            "author".into(),
            "DEFINE FIELD author ON post TYPE record<user>;".into(),
        );

        let mut table_infos = BTreeMap::new();
        table_infos.insert("user".into(), user_info);
        table_infos.insert("post".into(), post_info);

        let pulled = map_database_info(&tables, &table_infos, &preserve).expect("map");
        let user = pulled.models.iter().find(|m| m.name == "User").unwrap();
        let posts = user.fields.iter().find(|f| f.name == "posts").unwrap();
        assert_eq!(posts.link_storage, Some(core::LinkStorage::Computed));
        assert_eq!(posts.link_target.as_deref(), Some("Post"));
        assert!(posts.link_opposite_field.is_none());

        let authored = user.fields.iter().find(|f| f.name == "authored").unwrap();
        assert_eq!(authored.link_opposite_field.as_deref(), Some("author"));
    }

    #[test]
    fn pair_pulled_links_preserves_name_and_fills_stored_defaults() {
        let mut preserve = DatabaseSchema::empty();
        preserve.datasource.provider = "surrealdb".into();
        preserve.naming.tables = core::NamingCase::SnakeCase;
        preserve.models.push(Model {
            name: "User".into(),
            fields: vec![Field {
                name: "posts".into(),
                field_type: FieldType::Array(Box::new(FieldType::Model("Post".into()))),
                optional: false,
                unique: false,
                is_id: false,
                default_value: None,
                default_always: false,
                value_expression: None,
                readonly: false,
                flexible: false,
                link_target: Some("Post".into()),
                link_name: Some("PostAuthor".into()),
                on_delete: None,
                link_storage: Some(core::LinkStorage::Computed),
                link_opposite_field: Some("author".into()),
                relation_name: None,
                attributes: BTreeMap::new(),
            }],
            table_mode: TableMode::Schemafull,
            permissions: None,
            indexes: vec![],
            attributes: BTreeMap::new(),
        });
        preserve.models.push(Model {
            name: "Post".into(),
            fields: vec![],
            table_mode: TableMode::Schemafull,
            permissions: None,
            indexes: vec![],
            attributes: BTreeMap::new(),
        });

        let mut tables = BTreeMap::new();
        tables.insert("user".into(), "DEFINE TABLE user SCHEMAFULL;".into());
        tables.insert("post".into(), "DEFINE TABLE post SCHEMAFULL;".into());

        let mut user_info = TableInfo::default();
        user_info.fields.insert(
            "posts".into(),
            "DEFINE FIELD posts ON user COMPUTED <~(post FIELD author);".into(),
        );
        let mut post_info = TableInfo::default();
        // Non-record opposite so pair_pulled_links fills link_storage/on_delete defaults.
        post_info.fields.insert(
            "author".into(),
            "DEFINE FIELD author ON post TYPE string;".into(),
        );

        let mut table_infos = BTreeMap::new();
        table_infos.insert("user".into(), user_info);
        table_infos.insert("post".into(), post_info);

        let pulled = map_database_info(&tables, &table_infos, &preserve).expect("map");
        let post = pulled.models.iter().find(|m| m.name == "Post").unwrap();
        let author = post.fields.iter().find(|f| f.name == "author").unwrap();
        assert_eq!(author.link_name.as_deref(), Some("PostAuthor"));
        assert_eq!(author.link_storage, Some(core::LinkStorage::Stored));
        assert_eq!(author.on_delete, Some(core::OnDeleteAction::Ignore));
    }

    #[test]
    fn index_define_errors_too_short_and_missing_fields() {
        let short = parse_index_define("DEFINE INDEX x").expect_err("short");
        assert!(matches!(short, DomainError::DatabaseError(_)));
        // Need ≥7 tokens so the length check passes before FIELDS lookup.
        let missing =
            parse_index_define("DEFINE INDEX x ON user TABLE placeholder UNIQUE").expect_err("fields");
        assert!(matches!(missing, DomainError::DatabaseError(msg) if msg.contains("FIELDS")));
    }

    #[test]
    fn field_define_errors_propagate_through_map_fields() {
        let preserve = preserve_with_user();
        let mut tables = BTreeMap::new();
        tables.insert("user".into(), "DEFINE TABLE user SCHEMAFULL;".into());

        let cases = [
            "NOT A FIELD DEFINE",
            "DEFINE FIELD email TABLE user TYPE string;",
            "DEFINE FIELD email ON other TYPE string;",
            "DEFINE FIELD email ON user;",
        ];
        for define in cases {
            let mut info = TableInfo::default();
            info.fields.insert("email".into(), define.into());
            let mut table_infos = BTreeMap::new();
            table_infos.insert("user".into(), info);
            let err = map_database_info(&tables, &table_infos, &preserve).expect_err(define);
            assert!(matches!(err, DomainError::DatabaseError(_)), "{define}");
        }
    }

    #[test]
    fn skips_internal_tables_and_array_item_field_defines() {
        let preserve = preserve_with_user();
        let mut tables = BTreeMap::new();
        tables.insert("_awesome_migrations".into(), "DEFINE TABLE _awesome_migrations SCHEMAFULL;".into());
        tables.insert("user".into(), "DEFINE TABLE user SCHEMAFULL;".into());
        let mut info = TableInfo::default();
        info.fields.insert(
            "tags.*".into(),
            "DEFINE FIELD tags.* ON user TYPE string;".into(),
        );
        info.fields.insert(
            "email".into(),
            "DEFINE FIELD email ON user TYPE string;".into(),
        );
        let mut table_infos = BTreeMap::new();
        table_infos.insert("user".into(), info);

        let pulled = map_database_info(&tables, &table_infos, &preserve).expect("map");
        assert_eq!(pulled.models.len(), 1);
        assert!(pulled.models[0].fields.iter().all(|f| !f.name.contains('*')));
        assert!(pulled.models[0].fields.iter().any(|f| f.name == "email"));
    }

    #[test]
    fn model_name_for_table_uses_edge_map_attribute() {
        let mut preserve = DatabaseSchema::empty();
        preserve.datasource.provider = "surrealdb".into();
        preserve.naming.tables = core::NamingCase::SnakeCase;
        let mut attrs = BTreeMap::new();
        attrs.insert("map".into(), "user_likes".into());
        preserve.edges.push(Edge {
            name: "Likes".into(),
            in_model: "User".into(),
            out_model: "Post".into(),
            fields: vec![],
            table_mode: TableMode::Schemafull,
            permissions: None,
            attributes: attrs,
        });
        preserve.models.push(Model {
            name: "User".into(),
            fields: vec![],
            table_mode: TableMode::Schemafull,
            permissions: None,
            indexes: vec![],
            attributes: BTreeMap::new(),
        });

        let mut tables = BTreeMap::new();
        tables.insert("user".into(), "DEFINE TABLE user SCHEMAFULL;".into());
        let mut info = TableInfo::default();
        info.fields.insert(
            "edge_ref".into(),
            "DEFINE FIELD edge_ref ON user TYPE record<user_likes>;".into(),
        );
        let mut table_infos = BTreeMap::new();
        table_infos.insert("user".into(), info);

        let pulled = map_database_info(&tables, &table_infos, &preserve).expect("map");
        let edge_ref = pulled.models[0]
            .fields
            .iter()
            .find(|f| f.name == "edge_ref")
            .unwrap();
        assert_eq!(edge_ref.link_target.as_deref(), Some("Likes"));
    }

    #[test]
    fn read_expression_empty_start_and_keyword_break() {
        let empty = read_expression(&[], 0);
        assert_eq!(empty.0, "");
        assert_eq!(empty.1, 0);

        let tokens = tokenize("time::now() READONLY VALUE other");
        let (value, next) = read_expression(&tokens, 0);
        assert_eq!(value, "time::now()");
        assert_eq!(tokens[next].to_ascii_lowercase(), "readonly");
    }

    #[test]
    fn permissions_none_vs_full_on_table_and_edge() {
        let preserve = preserve_with_user();
        let mut tables = BTreeMap::new();
        tables.insert(
            "user".into(),
            "DEFINE TABLE user SCHEMAFULL PERMISSIONS NONE;".into(),
        );
        tables.insert(
            "likes".into(),
            "DEFINE TABLE likes TYPE RELATION IN user OUT user SCHEMAFULL PERMISSIONS FULL;".into(),
        );
        let pulled = map_database_info(&tables, &BTreeMap::new(), &preserve).expect("map");
        assert!(pulled.models[0].permissions.is_none());
        assert_eq!(pulled.edges[0].permissions.as_deref(), Some("FULL"));
    }

    #[test]
    fn id_field_non_record_keeps_scalar_type() {
        let preserve = preserve_with_user();
        let mut tables = BTreeMap::new();
        tables.insert("user".into(), "DEFINE TABLE user SCHEMAFULL;".into());
        let mut info = TableInfo::default();
        info.fields.insert(
            "id".into(),
            "DEFINE FIELD id ON user TYPE string;".into(),
        );
        let mut table_infos = BTreeMap::new();
        table_infos.insert("user".into(), info);
        let pulled = map_database_info(&tables, &table_infos, &preserve).expect("map");
        let id = pulled.models[0].fields.iter().find(|f| f.name == "id").unwrap();
        assert!(id.is_id);
        assert_eq!(id.field_type, FieldType::String);
    }

    #[test]
    fn unique_index_expected_name_marks_field_unique() {
        let preserve = preserve_with_user();
        let mut tables = BTreeMap::new();
        tables.insert("user".into(), "DEFINE TABLE user SCHEMAFULL;".into());
        let mut info = TableInfo::default();
        info.fields.insert(
            "email".into(),
            "DEFINE FIELD email ON user TYPE string;".into(),
        );
        info.indexes.insert(
            "user_email_unique".into(),
            "DEFINE INDEX user_email_unique ON user FIELDS email UNIQUE;".into(),
        );
        let mut table_infos = BTreeMap::new();
        table_infos.insert("user".into(), info);

        let pulled = map_database_info(&tables, &table_infos, &preserve).expect("map");
        let email = pulled.models[0]
            .fields
            .iter()
            .find(|f| f.name == "email")
            .unwrap();
        assert!(email.unique);
        assert!(pulled.models[0].indexes.is_empty());
    }

    #[test]
    fn rejects_relation_missing_out_and_targets() {
        let preserve = DatabaseSchema::empty();
        let cases = [
            (
                "DEFINE TABLE likes TYPE RELATION IN user SCHEMAFULL;",
                "missing OUT",
            ),
            (
                "DEFINE TABLE likes TYPE RELATION OUT user IN",
                "missing IN target",
            ),
            (
                "DEFINE TABLE likes TYPE RELATION IN user OUT",
                "missing OUT target",
            ),
        ];
        for (define, needle) in cases {
            let mut tables = BTreeMap::new();
            tables.insert("likes".into(), define.into());
            let err = map_database_info(&tables, &BTreeMap::new(), &preserve).expect_err(needle);
            assert!(
                matches!(err, DomainError::DatabaseError(ref msg) if msg.contains(needle)),
                "define={define} err={err:?}"
            );
        }
    }

    #[test]
    fn computed_without_backlink_and_weird_body_are_not_backlinks() {
        let preserve = preserve_with_user();
        let mut tables = BTreeMap::new();
        tables.insert("user".into(), "DEFINE TABLE user SCHEMAFULL;".into());
        let mut info = TableInfo::default();
        info.fields.insert(
            "score".into(),
            "DEFINE FIELD score ON user COMPUTED 1 + 2;".into(),
        );
        info.fields.insert(
            "weird".into(),
            "DEFINE FIELD weird ON user COMPUTED <~ (post FIELD);".into(),
        );
        let mut table_infos = BTreeMap::new();
        table_infos.insert("user".into(), info);

        // COMPUTED without <~ falls through to TYPE parse and fails; weird body is not a backlink.
        let err = map_database_info(&tables, &table_infos, &preserve).expect_err("no type");
        assert!(matches!(err, DomainError::DatabaseError(msg) if msg.contains("TYPE")));

        assert!(parse_computed_backlink(
            "DEFINE FIELD score ON user COMPUTED 1 + 2;",
            &preserve
        )
        .is_none());
        assert!(parse_computed_backlink(
            "DEFINE FIELD weird ON user COMPUTED <~ (post FIELD);",
            &preserve
        )
        .is_none());
    }

    #[test]
    fn parses_flexible_after_type_and_map_scalar_via_option_array() {
        let preserve = preserve_with_user();
        let mut tables = BTreeMap::new();
        tables.insert("doc".into(), "DEFINE TABLE doc SCHEMAFULL;".into());
        let mut info = TableInfo::default();
        info.fields.insert(
            "meta".into(),
            "DEFINE FIELD meta ON doc TYPE object FLEXIBLE;".into(),
        );
        info.fields.insert(
            "flags".into(),
            "DEFINE FIELD flags ON doc TYPE array<bool>;".into(),
        );
        info.fields.insert(
            "when".into(),
            "DEFINE FIELD when ON doc TYPE option<datetime>;".into(),
        );
        info.fields.insert(
            "owner".into(),
            "DEFINE FIELD owner ON doc TYPE array<record<user>>;".into(),
        );
        info.fields.insert(
            "shape".into(),
            "DEFINE FIELD shape ON doc TYPE option<geometry>;".into(),
        );
        let mut table_infos = BTreeMap::new();
        table_infos.insert("doc".into(), info);

        let pulled = map_database_info(&tables, &table_infos, &preserve).expect("map");
        let doc = &pulled.models[0];
        let field = |name: &str| doc.fields.iter().find(|f| f.name == name).unwrap();
        assert!(field("meta").flexible);
        assert_eq!(
            field("flags").field_type,
            FieldType::Array(Box::new(FieldType::Bool))
        );
        assert!(field("when").optional);
        assert_eq!(field("when").field_type, FieldType::Datetime);
        assert_eq!(
            field("owner").field_type,
            FieldType::Array(Box::new(FieldType::RecordId("user".into())))
        );
        assert_eq!(field("shape").field_type, FieldType::Custom("geometry".into()));
    }

    #[test]
    fn model_and_edge_name_fallback_and_map_paths() {
        let mut preserve = DatabaseSchema::empty();
        preserve.naming.tables = core::NamingCase::SnakeCase;
        let mut attrs = BTreeMap::new();
        attrs.insert("map".into(), "custom_edge".into());
        preserve.edges.push(Edge {
            name: "Follows".into(),
            in_model: "User".into(),
            out_model: "User".into(),
            fields: vec![],
            table_mode: TableMode::Schemafull,
            permissions: None,
            attributes: attrs,
        });

        let mut tables = BTreeMap::new();
        tables.insert("mystery_table".into(), "DEFINE TABLE mystery_table SCHEMAFULL;".into());
        tables.insert(
            "custom_edge".into(),
            "DEFINE TABLE custom_edge TYPE RELATION IN user OUT user SCHEMAFULL;".into(),
        );
        let pulled = map_database_info(&tables, &BTreeMap::new(), &preserve).expect("map");
        assert!(pulled.models.iter().any(|m| m.name == "MysteryTable"));
        assert_eq!(pulled.edges[0].name, "Follows");
        assert_eq!(model_name_for_table("unknown_x", &preserve), "UnknownX");
        assert_eq!(edge_name_for_table("custom_edge", &preserve), "Follows");
        assert_eq!(edge_name_for_table("other_edge", &preserve), "OtherEdge");
    }

    #[test]
    fn index_analyzer_and_hnsw_empty_values_are_ignored() {
        let ft = parse_index_define(
            "DEFINE INDEX body_ft ON doc FIELDS body FULLTEXT ANALYZER;",
        )
        .expect("fulltext");
        assert!(ft.fulltext);
        assert!(ft.fulltext_analyzer.is_none());

        // FULLTEXT without ANALYZER keyword at all.
        let ft_bare =
            parse_index_define("DEFINE INDEX body_ft ON doc FIELDS body FULLTEXT").expect("ft bare");
        assert!(ft_bare.fulltext);
        assert!(ft_bare.fulltext_analyzer.is_none());

        let hn_dim = parse_index_define(
            "DEFINE INDEX emb ON doc FIELDS embedding HNSW DIMENSION",
        )
        .expect("hnsw dim");
        assert!(hn_dim.vector);
        assert!(hn_dim.vector_dimension.is_none());

        // HNSW without DIMENSION keyword.
        let hn_bare =
            parse_index_define("DEFINE INDEX emb ON doc FIELDS embedding HNSW").expect("hn bare");
        assert!(hn_bare.vector);
        assert!(hn_bare.vector_dimension.is_none());

        let hn_dist = parse_index_define(
            "DEFINE INDEX emb ON doc FIELDS embedding HNSW DIMENSION 3 DIST",
        )
        .expect("hnsw dist");
        assert_eq!(hn_dist.vector_dimension, Some(3));
        assert!(hn_dist.vector_dist.is_none());
    }

    #[test]
    fn unique_index_with_custom_name_is_kept_as_index() {
        let preserve = preserve_with_user();
        let mut tables = BTreeMap::new();
        tables.insert("user".into(), "DEFINE TABLE user SCHEMAFULL;".into());
        let mut info = TableInfo::default();
        info.fields.insert(
            "email".into(),
            "DEFINE FIELD email ON user TYPE string;".into(),
        );
        info.indexes.insert(
            "custom_email_idx".into(),
            "DEFINE INDEX custom_email_idx ON user FIELDS email UNIQUE;".into(),
        );
        let mut table_infos = BTreeMap::new();
        table_infos.insert("user".into(), info);
        let pulled = map_database_info(&tables, &table_infos, &preserve).expect("map");
        assert_eq!(pulled.models[0].indexes.len(), 1);
        assert!(!pulled.models[0]
            .fields
            .iter()
            .find(|f| f.name == "email")
            .unwrap()
            .unique);
    }

    #[test]
    fn unique_index_without_matching_field_is_kept() {
        let preserve = preserve_with_user();
        let mut tables = BTreeMap::new();
        tables.insert("user".into(), "DEFINE TABLE user SCHEMAFULL;".into());
        let mut info = TableInfo::default();
        info.indexes.insert(
            "user_email_unique".into(),
            "DEFINE INDEX user_email_unique ON user FIELDS email UNIQUE;".into(),
        );
        let mut table_infos = BTreeMap::new();
        table_infos.insert("user".into(), info);
        let pulled = map_database_info(&tables, &table_infos, &preserve).expect("map");
        assert_eq!(pulled.models[0].indexes.len(), 1);
        assert_eq!(pulled.models[0].indexes[0].fields, vec!["email".to_owned()]);
    }

    #[test]
    fn pair_pulled_links_skips_missing_computed_or_stored_sides() {
        let mut preserve = preserve_with_user();
        preserve.models.push(Model {
            name: "Post".into(),
            fields: vec![],
            table_mode: TableMode::Schemafull,
            permissions: None,
            indexes: vec![],
            attributes: BTreeMap::new(),
        });
        let mut tables = BTreeMap::new();
        tables.insert("user".into(), "DEFINE TABLE user SCHEMAFULL;".into());
        // Computed backlink to Post.author, but Post table is absent from pull → stored side missing.
        let mut user_info = TableInfo::default();
        user_info.fields.insert(
            "posts".into(),
            "DEFINE FIELD posts ON user COMPUTED <~(post FIELD author);".into(),
        );
        let mut table_infos = BTreeMap::new();
        table_infos.insert("user".into(), user_info);
        let pulled = map_database_info(&tables, &table_infos, &preserve).expect("map");
        let posts = pulled.models[0]
            .fields
            .iter()
            .find(|f| f.name == "posts")
            .unwrap();
        assert_eq!(posts.link_name.as_deref(), Some("PostUser"));
    }

    #[test]
    fn read_expression_start_past_end() {
        let tokens = tokenize("DEFAULT 'x'");
        let (value, next) = read_expression(&tokens, tokens.len());
        assert_eq!(value, "");
        assert_eq!(next, tokens.len());
    }
}
