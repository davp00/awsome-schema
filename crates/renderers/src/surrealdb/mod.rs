#![allow(
    clippy::missing_errors_doc,
    clippy::needless_pass_by_value,
    clippy::format_push_string,
    clippy::unnecessary_wraps,
    clippy::match_same_arms
)]

use core::ports::{MigrationRenderer, SchemaRenderer};
use core::{
    DatabaseSchema, DomainError, Edge, Field, MigrationOperation, MigrationPlan, Model, TableMode,
};

pub struct SurrealDbRenderer;

impl SurrealDbRenderer {
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl Default for SurrealDbRenderer {
    fn default() -> Self {
        Self::new()
    }
}

impl SchemaRenderer for SurrealDbRenderer {
    fn render_schema(&self, schema: &DatabaseSchema) -> Result<String, DomainError> {
        let mut lines = Vec::new();

        for model in &schema.models {
            lines.extend(render_model_schema(model));
        }

        for edge in &schema.edges {
            lines.extend(render_edge_schema(edge));
        }

        Ok(lines.join("\n"))
    }
}

impl MigrationRenderer for SurrealDbRenderer {
    fn render_migration(&self, migration: &MigrationPlan) -> Result<String, DomainError> {
        let mut lines = vec![format!("-- Migration: {}", migration.name)];

        for operation in &migration.operations {
            lines.extend(render_operation(operation)?);
        }

        Ok(lines.join("\n"))
    }
}

fn render_model_schema(model: &Model) -> Vec<String> {
    let table = model.table_name();
    let mut lines = vec![render_define_table(&table, model.table_mode)];

    for field in &model.fields {
        if field.relation_name.is_some() {
            continue;
        }
        lines.push(render_define_field(&table, field));
        if field.unique {
            lines.push(render_unique_index(&table, field));
        }
    }

    for index in &model.indexes {
        lines.push(render_define_index(&table, index));
    }

    if let Some(permission) = &model.permissions {
        lines.push(format!("DEFINE TABLE {table} PERMISSIONS {permission};"));
    }

    lines
}

fn render_edge_schema(edge: &Edge) -> Vec<String> {
    let table = edge.table_name();
    let mut lines = vec![render_define_table(&table, edge.table_mode)];

    for field in &edge.fields {
        lines.push(render_define_field(&table, field));
    }

    lines
}

fn render_define_table(name: &str, mode: TableMode) -> String {
    let mode_name = match mode {
        TableMode::Schemafull => "SCHEMAFULL",
        TableMode::Schemaless => "SCHEMALESS",
    };
    format!("DEFINE TABLE {name} {mode_name};")
}

fn render_define_field(table: &str, field: &Field) -> String {
    let mut line = format!(
        "DEFINE FIELD {} ON {table} TYPE {}",
        field.name,
        field.field_type.surreal_type_name(field.optional)
    );

    if let Some(default) = &field.default_value {
        if field.default_always {
            line.push_str(" DEFAULT ALWAYS");
        } else {
            line.push_str(" DEFAULT");
        }
        line.push_str(&format!(" {default}"));
    }

    if let Some(value) = &field.value_expression {
        line.push_str(&format!(" VALUE {value}"));
    }

    if field.readonly {
        line.push_str(" READONLY");
    }

    line.push(';');
    line
}

fn render_unique_index(table: &str, field: &Field) -> String {
    format!("DEFINE INDEX {}_{}_unique ON {table} FIELDS {} UNIQUE;", table, field.name, field.name)
}

fn render_define_index(table: &str, index: &Index) -> String {
    let name = index.resolved_name(table);
    let fields = index.fields.join(", ");

    let mut line = format!("DEFINE INDEX {name} ON {table} FIELDS {fields}");
    if index.unique {
        line.push_str(" UNIQUE");
    }
    if index.fulltext {
        line.push_str(" FULLTEXT");
    }
    if index.vector {
        line.push_str(" VECTOR");
    }
    line.push(';');
    line
}

fn render_operation(operation: &MigrationOperation) -> Result<Vec<String>, DomainError> {
    let lines = match operation {
        MigrationOperation::CreateTable { name, mode } => {
            vec![render_define_table(name, *mode)]
        }
        MigrationOperation::DropTable { name } => vec![format!("REMOVE TABLE {name};")],
        MigrationOperation::AlterTable { name, mode } => vec![render_define_table(name, *mode)],
        MigrationOperation::CreateField { table, field } => vec![render_define_field(table, field)],
        MigrationOperation::DropField { table, name } => {
            vec![format!("REMOVE FIELD {name} ON {table};")]
        }
        MigrationOperation::AlterField { table, field } => vec![render_define_field(table, field)],
        MigrationOperation::CreateIndex { table, index } => vec![render_define_index(table, index)],
        MigrationOperation::DropIndex { table, name } => {
            vec![format!("REMOVE INDEX {name} ON {table};")]
        }
        MigrationOperation::CreateEvent { table, name, body } => {
            vec![format!("DEFINE EVENT {name} ON {table} {body};")]
        }
        MigrationOperation::DropEvent { table, name } => {
            vec![format!("REMOVE EVENT {name} ON {table};")]
        }
        MigrationOperation::CreateFunction { name, body } => {
            vec![format!("DEFINE FUNCTION fn::{name}() {body};")]
        }
        MigrationOperation::DropFunction { name } => vec![format!("REMOVE FUNCTION fn::{name};")],
        MigrationOperation::CreatePermission { table, permission } => {
            vec![format!("DEFINE TABLE {table} PERMISSIONS {permission};")]
        }
        MigrationOperation::UpdatePermission { table, permission } => {
            vec![format!("DEFINE TABLE {table} PERMISSIONS {permission};")]
        }
    };

    Ok(lines)
}

use core::Index;

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use core::{Datasource, Field, FieldType};

    use super::*;

    #[test]
    fn renders_default_value_field() {
        let field = Field {
            name: "createdAt".to_owned(),
            field_type: FieldType::Datetime,
            optional: false,
            unique: false,
            is_id: false,
            default_value: Some("time::now()".to_owned()),
            default_always: false,
            value_expression: None,
            readonly: false,
            link_target: None,
            relation_name: None,
            attributes: BTreeMap::new(),
        };

        let rendered = render_define_field("user", &field);
        assert_eq!(rendered, "DEFINE FIELD createdAt ON user TYPE datetime DEFAULT time::now();");
    }

    #[test]
    fn renders_value_and_readonly_field() {
        let field = Field {
            name: "createdAt".to_owned(),
            field_type: FieldType::Datetime,
            optional: false,
            unique: false,
            is_id: false,
            default_value: None,
            default_always: false,
            value_expression: Some("time::now()".to_owned()),
            readonly: true,
            link_target: None,
            relation_name: None,
            attributes: BTreeMap::new(),
        };

        let rendered = render_define_field("user", &field);
        assert_eq!(
            rendered,
            "DEFINE FIELD createdAt ON user TYPE datetime VALUE time::now() READONLY;"
        );
    }

    #[test]
    fn renders_updated_value_field() {
        let field = Field {
            name: "updatedAt".to_owned(),
            field_type: FieldType::Datetime,
            optional: false,
            unique: false,
            is_id: false,
            default_value: None,
            default_always: false,
            value_expression: Some("time::now()".to_owned()),
            readonly: false,
            link_target: None,
            relation_name: None,
            attributes: BTreeMap::new(),
        };

        let rendered = render_define_field("user", &field);
        assert_eq!(rendered, "DEFINE FIELD updatedAt ON user TYPE datetime VALUE time::now();");
    }

    #[test]
    fn renders_optional_field_as_option_type() {
        let field = Field {
            name: "age".to_owned(),
            field_type: FieldType::Int,
            optional: true,
            unique: false,
            is_id: false,
            default_value: None,
            default_always: false,
            value_expression: None,
            readonly: false,
            link_target: None,
            relation_name: None,
            attributes: BTreeMap::new(),
        };

        let rendered = render_define_field("user", &field);
        assert_eq!(rendered, "DEFINE FIELD age ON user TYPE option<int>;");
    }

    #[test]
    fn renders_optional_record_link() {
        let field = Field {
            name: "author".to_owned(),
            field_type: FieldType::Model("User".to_owned()),
            optional: true,
            unique: false,
            is_id: false,
            default_value: None,
            default_always: false,
            value_expression: None,
            readonly: false,
            link_target: Some("User".to_owned()),
            relation_name: None,
            attributes: BTreeMap::new(),
        };

        let rendered = render_define_field("post", &field);
        assert_eq!(rendered, "DEFINE FIELD author ON post TYPE option<record<User>>;");
    }

    #[test]
    fn renders_down_migration_for_field_removal() {
        let plan = MigrationPlan {
            name: "add_name_down".to_owned(),
            operations: vec![MigrationOperation::DropField {
                table: "user".to_owned(),
                name: "name".to_owned(),
            }],
        };

        let rendered =
            SurrealDbRenderer::new().render_migration(&plan).expect("down migration should render");

        assert!(rendered.contains("REMOVE FIELD name ON user;"));
    }

    #[test]
    fn renders_user_table_migration() {
        let plan = MigrationPlan {
            name: "create_user".to_owned(),
            operations: vec![
                MigrationOperation::CreateTable {
                    name: "user".to_owned(),
                    mode: TableMode::Schemafull,
                },
                MigrationOperation::CreateField {
                    table: "user".to_owned(),
                    field: Field {
                        name: "email".to_owned(),
                        field_type: FieldType::String,
                        optional: false,
                        unique: true,
                        is_id: false,
                        default_value: None,
                        default_always: false,
                        value_expression: None,
                        readonly: false,
                        link_target: None,
                        relation_name: None,
                        attributes: BTreeMap::new(),
                    },
                },
            ],
        };

        let rendered =
            SurrealDbRenderer::new().render_migration(&plan).expect("migration should render");

        assert!(rendered.contains("DEFINE TABLE user SCHEMAFULL;"));
        assert!(rendered.contains("DEFINE FIELD email ON user TYPE string;"));
    }

    #[test]
    fn renders_full_schema() {
        let schema = DatabaseSchema {
            datasource: Datasource {
                provider: "surrealdb".to_owned(),
                url: None,
                namespace: None,
                database: None,
                extra: BTreeMap::new(),
            },
            generators: Vec::new(),
            models: vec![Model {
                name: "User".to_owned(),
                fields: vec![Field {
                    name: "email".to_owned(),
                    field_type: FieldType::String,
                    optional: false,
                    unique: true,
                    is_id: false,
                    default_value: None,
                    default_always: false,
                    value_expression: None,
                    readonly: false,
                    link_target: None,
                    relation_name: None,
                    attributes: BTreeMap::new(),
                }],
                table_mode: TableMode::Schemafull,
                permissions: None,
                indexes: Vec::new(),
                attributes: BTreeMap::new(),
            }],
            edges: Vec::new(),
        };

        let rendered =
            SurrealDbRenderer::new().render_schema(&schema).expect("schema should render");

        assert!(rendered.contains("DEFINE TABLE user SCHEMAFULL;"));
        assert!(rendered.contains("DEFINE INDEX user_email_unique ON user FIELDS email UNIQUE;"));
    }
}
