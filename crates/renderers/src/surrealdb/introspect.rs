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

    let mut schema = DatabaseSchema {
        datasource: DatabaseSchema::empty().datasource,
        naming: NamingConvention::default(),
        generators: Vec::new(),
        object_types: Vec::new(),
        models,
        edges,
    };
    pair_pulled_links(&mut schema);
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
        if let FieldType::Model(model) = field_type {
            field_type = FieldType::RecordId(model);
            link_target = None;
        }
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

fn pair_pulled_links(schema: &mut DatabaseSchema) {
    // Assign shared link_name from computed side → stored opposite
    let mut pairs: Vec<(String, String, String, String)> = Vec::new();
    // (computed_model, computed_field, stored_model, stored_field)
    for model in &schema.models {
        for field in &model.fields {
            if !field.is_computed_link() {
                continue;
            }
            let Some(target) = &field.link_target else {
                continue;
            };
            let Some(opposite) = &field.link_opposite_field else {
                continue;
            };
            if opposite.is_empty() {
                continue;
            }
            pairs.push((
                model.name.clone(),
                field.name.clone(),
                target.clone(),
                opposite.clone(),
            ));
        }
    }

    for (computed_model, computed_field, stored_model, stored_field) in pairs {
        let pair_name = format!("{stored_model}{computed_model}");
        if let Some(model) = schema.models.iter_mut().find(|m| m.name == computed_model) {
            if let Some(field) = model.fields.iter_mut().find(|f| f.name == computed_field) {
                field.link_name = Some(pair_name.clone());
                // Prefer list type for computed backrefs
                if let Some(target) = field.link_target.clone() {
                    if !field.is_list_link() {
                        field.field_type = FieldType::Array(Box::new(FieldType::Model(target)));
                    }
                }
            }
        }
        if let Some(model) = schema.models.iter_mut().find(|m| m.name == stored_model) {
            if let Some(field) = model.fields.iter_mut().find(|f| f.name == stored_field) {
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
            fields: vec![],
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
        assert_eq!(posts.link_name.as_ref(), author.link_name.as_ref());
        assert!(posts.link_name.is_some());
    }
}
