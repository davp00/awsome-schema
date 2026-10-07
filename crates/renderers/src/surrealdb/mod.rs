#![allow(
    clippy::missing_errors_doc,
    clippy::needless_pass_by_value,
    clippy::format_push_string,
    clippy::unnecessary_wraps,
    clippy::match_same_arms
)]

use core::ports::{MigrationRenderer, SchemaRenderer};
use core::{
    DatabaseSchema, DomainError, Edge, Field, Index, MigrationOperation, MigrationPlan, Model,
    NamingContext, TableMode,
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
        let naming = NamingContext::new(&schema.naming, &schema.models);
        let mut lines = Vec::new();

        for model in &schema.models {
            lines.extend(render_model_schema(model, &naming));
        }

        for edge in &schema.edges {
            lines.extend(render_edge_schema(edge, &naming));
        }

        Ok(lines.join("\n"))
    }
}

impl MigrationRenderer for SurrealDbRenderer {
    fn render_migration(&self, migration: &MigrationPlan) -> Result<String, DomainError> {
        let naming = NamingContext::new(&migration.naming, &[]);
        let mut lines = vec![format!("-- Migration: {}", migration.name)];

        for operation in &migration.operations {
            lines.extend(render_operation(operation, &naming)?);
        }

        Ok(lines.join("\n"))
    }
}

fn render_model_schema(model: &Model, naming: &NamingContext<'_>) -> Vec<String> {
    let table = naming.table_name_for_model(model);
    let mut lines = vec![render_define_table(&table, model.table_mode)];

    for field in &model.fields {
        if field.relation_name.is_some() {
            continue;
        }
        lines.push(render_define_field(&table, field, naming));
        if field.unique && !field.is_computed_link() {
            lines.push(render_unique_index(&table, field, naming));
        }
    }

    for index in &model.indexes {
        lines.push(render_define_index(&table, index, naming));
    }

    if let Some(permission) = &model.permissions {
        lines.push(format!("DEFINE TABLE {table} PERMISSIONS {permission};"));
    }

    lines
}

fn render_edge_schema(edge: &Edge, naming: &NamingContext<'_>) -> Vec<String> {
    let table = naming.map_table_name(&edge.name, &edge.attributes);
    let in_table = naming.table_name_for_type(&edge.in_model);
    let out_table = naming.table_name_for_type(&edge.out_model);
    let mut lines = vec![render_define_relation_table(&table, &in_table, &out_table, edge.table_mode)];

    for field in &edge.fields {
        lines.push(render_define_field(&table, field, naming));
    }

    if let Some(permission) = &edge.permissions {
        lines.push(format!("DEFINE TABLE {table} PERMISSIONS {permission};"));
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

fn render_define_relation_table(
    name: &str,
    in_table: &str,
    out_table: &str,
    mode: TableMode,
) -> String {
    let mode_name = match mode {
        TableMode::Schemafull => "SCHEMAFULL",
        TableMode::Schemaless => "SCHEMALESS",
    };
    format!("DEFINE TABLE {name} TYPE RELATION IN {in_table} OUT {out_table} {mode_name};")
}

fn render_define_table_op(
    name: &str,
    mode: TableMode,
    relation: Option<&core::RelationEndpoints>,
) -> String {
    match relation {
        Some(endpoints) => {
            render_define_relation_table(name, &endpoints.in_table, &endpoints.out_table, mode)
        }
        None => render_define_table(name, mode),
    }
}

fn render_define_field(table: &str, field: &Field, naming: &NamingContext<'_>) -> String {
    let field_name = naming.field_name(field);

    if field.is_computed_link() {
        return render_computed_link(table, field, naming);
    }

    let mut line = format!(
        "DEFINE FIELD {field_name} ON {table} TYPE {}",
        naming.surreal_type_name(&field.field_type, field.optional)
    );

    if field.flexible {
        line.push_str(" FLEXIBLE");
    }

    if field.is_stored_link() || field.is_link() {
        line.push_str(" REFERENCE");
        let action = field.on_delete.unwrap_or(core::OnDeleteAction::Ignore);
        line.push_str(&format!(" ON DELETE {}", action.as_surreal()));
    }

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

fn render_computed_link(_table: &str, field: &Field, naming: &NamingContext<'_>) -> String {
    let field_name = naming.field_name(field);
    let source_model = field
        .link_target
        .as_deref()
        .or_else(|| field.field_type.link_model_name())
        .unwrap_or("unknown");
    let source_table = naming.table_name_for_type(source_model);
    let back_field_dsl = field.link_opposite_field.as_deref().unwrap_or(field_name.as_str());
    let back_field = naming.field_name_str(back_field_dsl, &BTreeMap::new());

    format!("DEFINE FIELD {field_name} ON {_table} COMPUTED <~({source_table} FIELD {back_field});")
}

fn render_unique_index(table: &str, field: &Field, naming: &NamingContext<'_>) -> String {
    let field_name = naming.field_name(field);
    format!("DEFINE INDEX {table}_{field_name}_unique ON {table} FIELDS {field_name} UNIQUE;")
}

fn render_define_index(table: &str, index: &Index, naming: &NamingContext<'_>) -> String {
    let name = index.resolved_name(table, naming.convention());
    let fields = index
        .fields
        .iter()
        .map(|field| naming.field_name_str(field, &BTreeMap::new()))
        .collect::<Vec<_>>()
        .join(", ");

    let mut line = format!("DEFINE INDEX {name} ON {table} FIELDS {fields}");
    if index.unique {
        line.push_str(" UNIQUE");
    }
    if index.fulltext {
        let analyzer = index.fulltext_analyzer.as_deref().unwrap_or("english");
        line.push_str(&format!(" FULLTEXT ANALYZER {analyzer} BM25"));
    }
    if index.vector {
        let dimension = index.vector_dimension.unwrap_or(0);
        let dist = index.vector_dist.unwrap_or_default().as_surreal();
        line.push_str(&format!(" HNSW DIMENSION {dimension} DIST {dist}"));
    }
    line.push(';');
    line
}

fn render_operation(
    operation: &MigrationOperation,
    naming: &NamingContext<'_>,
) -> Result<Vec<String>, DomainError> {
    let lines = match operation {
        MigrationOperation::CreateTable { name, mode, relation } => {
            vec![render_define_table_op(name, *mode, relation.as_ref())]
        }
        MigrationOperation::DropTable { name } => vec![format!("REMOVE TABLE {name};")],
        MigrationOperation::AlterTable { name, mode, relation } => {
            vec![render_define_table_op(name, *mode, relation.as_ref())]
        }
        MigrationOperation::CreateField { table, field } => {
            vec![render_define_field(table, field, naming)]
        }
        MigrationOperation::DropField { table, name } => {
            let field_name = naming.field_name_str(name, &BTreeMap::new());
            vec![format!("REMOVE FIELD {field_name} ON {table};")]
        }
        MigrationOperation::AlterField { table, field } => {
            vec![render_define_field(table, field, naming)]
        }
        MigrationOperation::CreateIndex { table, index } => {
            vec![render_define_index(table, index, naming)]
        }
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
        MigrationOperation::DropPermission { table } => {
            vec![format!("DEFINE TABLE {table} PERMISSIONS NONE;")]
        }
    };

    Ok(lines)
}

use std::collections::BTreeMap;

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use core::{Datasource, Field, FieldType, Model, NamingCase, NamingContext, NamingConvention};

    use super::*;

    fn snake_case_fields() -> NamingConvention {
        NamingConvention { tables: NamingCase::SnakeCase, fields: Some(NamingCase::SnakeCase) }
    }

    #[test]
    fn renders_default_value_field() {
        let convention = snake_case_fields();
        let naming = NamingContext::new(&convention, &[]);
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
            flexible: false,
            link_target: None,
            link_name: None,
            on_delete: None,
            link_storage: None,
            link_opposite_field: None,
            relation_name: None,
            attributes: BTreeMap::new(),
        };

        let rendered = render_define_field("user", &field, &naming);
        assert_eq!(rendered, "DEFINE FIELD created_at ON user TYPE datetime DEFAULT time::now();");
    }

    #[test]
    fn renders_value_and_readonly_field() {
        let convention = snake_case_fields();
        let naming = NamingContext::new(&convention, &[]);
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
            flexible: false,
            link_target: None,
            link_name: None,
            on_delete: None,
            link_storage: None,
            link_opposite_field: None,
            relation_name: None,
            attributes: BTreeMap::new(),
        };

        let rendered = render_define_field("user", &field, &naming);
        assert_eq!(
            rendered,
            "DEFINE FIELD created_at ON user TYPE datetime VALUE time::now() READONLY;"
        );
    }

    #[test]
    fn renders_updated_value_field() {
        let convention = snake_case_fields();
        let naming = NamingContext::new(&convention, &[]);
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
            flexible: false,
            link_target: None,
            link_name: None,
            on_delete: None,
            link_storage: None,
            link_opposite_field: None,
            relation_name: None,
            attributes: BTreeMap::new(),
        };

        let rendered = render_define_field("user", &field, &naming);
        assert_eq!(rendered, "DEFINE FIELD updated_at ON user TYPE datetime VALUE time::now();");
    }

    #[test]
    fn preserves_field_name_when_fields_naming_not_set() {
        let convention = NamingConvention::default();
        let naming = NamingContext::new(&convention, &[]);
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
            flexible: false,
            link_target: None,
            link_name: None,
            on_delete: None,
            link_storage: None,
            link_opposite_field: None,
            relation_name: None,
            attributes: BTreeMap::new(),
        };

        let rendered = render_define_field("user", &field, &naming);
        assert_eq!(
            rendered,
            "DEFINE FIELD createdAt ON user TYPE datetime VALUE time::now() READONLY;"
        );
    }

    #[test]
    fn renders_optional_field_as_option_type() {
        let convention = NamingConvention::default();
        let naming = NamingContext::new(&convention, &[]);
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
            flexible: false,
            link_target: None,
            link_name: None,
            on_delete: None,
            link_storage: None,
            link_opposite_field: None,
            relation_name: None,
            attributes: BTreeMap::new(),
        };

        let rendered = render_define_field("user", &field, &naming);
        assert_eq!(rendered, "DEFINE FIELD age ON user TYPE option<int>;");
    }

    #[test]
    fn renders_flexible_object_and_nested_subdocument_fields() {
        let convention = snake_case_fields();
        let naming = NamingContext::new(&convention, &[]);

        let metadata = Field {
            name: "metadata".to_owned(),
            field_type: FieldType::Object,
            optional: false,
            unique: false,
            is_id: false,
            default_value: None,
            default_always: false,
            value_expression: None,
            readonly: false,
            flexible: true,
            link_target: None,
            link_name: None,
            on_delete: None,
            link_storage: None,
            link_opposite_field: None,
            relation_name: None,
            attributes: BTreeMap::new(),
        };
        assert_eq!(
            render_define_field("user", &metadata, &naming),
            "DEFINE FIELD metadata ON user TYPE object FLEXIBLE;"
        );

        let nested = Field {
            name: "metadata.userId".to_owned(),
            field_type: FieldType::Int,
            optional: true,
            unique: false,
            is_id: false,
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
        };
        assert_eq!(
            render_define_field("user", &nested, &naming),
            "DEFINE FIELD metadata.user_id ON user TYPE option<int>;"
        );
    }

    #[test]
    fn renders_optional_record_link() {
        let models = vec![Model {
            name: "User".to_owned(),
            fields: Vec::new(),
            table_mode: TableMode::Schemafull,
            permissions: None,
            indexes: Vec::new(),
            attributes: BTreeMap::new(),
        }];
        let convention = NamingConvention::default();
        let naming = NamingContext::new(&convention, &models);
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
            flexible: false,
            link_target: Some("User".to_owned()),
            link_name: None,
            on_delete: None,
            link_storage: None,
            link_opposite_field: None,
            relation_name: None,
            attributes: BTreeMap::new(),
        };

        let rendered = render_define_field("post", &field, &naming);
        assert_eq!(
            rendered,
            "DEFINE FIELD author ON post TYPE option<record<user>> REFERENCE ON DELETE IGNORE;"
        );
    }

    #[test]
    fn renders_down_migration_for_field_removal() {
        let plan = MigrationPlan {
            name: "add_name_down".to_owned(),
            operations: vec![MigrationOperation::DropField {
                table: "user".to_owned(),
                name: "name".to_owned(),
            }],
            naming: NamingConvention::default(),
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
                    relation: None,
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
                        flexible: false,
                        link_target: None,
                        link_name: None,
                        on_delete: None,
                        link_storage: None,
                        link_opposite_field: None,
                        relation_name: None,
                        attributes: BTreeMap::new(),
                    },
                },
            ],
            naming: NamingConvention::default(),
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
            naming: NamingConvention::default(),
            generators: Vec::new(),
            object_types: Vec::new(),
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
                    flexible: false,
                    link_target: None,
                    link_name: None,
                    on_delete: None,
                    link_storage: None,
                    link_opposite_field: None,
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

    #[test]
    fn renders_remaining_migration_operations() {
        use core::{Index, MigrationOperation, MigrationPlan};

        let plan = MigrationPlan {
            name: "full".to_owned(),
            operations: vec![
                MigrationOperation::DropTable { name: "legacy".to_owned() },
                MigrationOperation::AlterTable {
                    name: "user".to_owned(),
                    mode: TableMode::Schemaless,
                    relation: None,
                },
                MigrationOperation::DropField {
                    table: "user".to_owned(),
                    name: "legacy".to_owned(),
                },
                MigrationOperation::CreateIndex {
                    table: "user".to_owned(),
                    index: Index {
                        name: Some("custom_idx".to_owned()),
                        fields: vec!["email".to_owned()],
                        unique: true,
                        fulltext: false,
                        fulltext_analyzer: None,
                        vector: false,
                        vector_dimension: None,
                        vector_dist: None,
                    },
                },
                MigrationOperation::DropIndex {
                    table: "user".to_owned(),
                    name: "custom_idx".to_owned(),
                },
                MigrationOperation::CreateEvent {
                    table: "user".to_owned(),
                    name: "created".to_owned(),
                    body: "WHEN $event = 'CREATE'".to_owned(),
                },
                MigrationOperation::DropEvent {
                    table: "user".to_owned(),
                    name: "created".to_owned(),
                },
                MigrationOperation::CreateFunction {
                    name: "hello".to_owned(),
                    body: "{ RETURN 'hi'; }".to_owned(),
                },
                MigrationOperation::DropFunction { name: "hello".to_owned() },
                MigrationOperation::CreatePermission {
                    table: "user".to_owned(),
                    permission: "FULL".to_owned(),
                },
                MigrationOperation::UpdatePermission {
                    table: "user".to_owned(),
                    permission: "NONE".to_owned(),
                },
                MigrationOperation::DropPermission {
                    table: "user".to_owned(),
                },
            ],
            naming: NamingConvention::default(),
        };

        let rendered =
            SurrealDbRenderer::new().render_migration(&plan).expect("migration should render");

        assert!(rendered.contains("REMOVE TABLE legacy;"));
        assert!(rendered.contains("DEFINE TABLE user SCHEMALESS;"));
        assert!(rendered.contains("REMOVE FIELD legacy ON user;"));
        assert!(rendered.contains("DEFINE INDEX custom_idx ON user FIELDS email UNIQUE;"));
        assert!(rendered.contains("REMOVE INDEX custom_idx ON user;"));
        assert!(rendered.contains("DEFINE EVENT created ON user WHEN $event = 'CREATE';"));
        assert!(rendered.contains("DEFINE FUNCTION fn::hello() { RETURN 'hi'; };"));
        assert!(rendered.contains("DEFINE TABLE user PERMISSIONS NONE;"));
    }

    #[test]
    fn renders_fulltext_and_hnsw_indexes() {
        use core::{Index, MigrationOperation, MigrationPlan, VectorDist};

        let plan = MigrationPlan {
            name: "indexes".to_owned(),
            operations: vec![
                MigrationOperation::CreateIndex {
                    table: "doc".to_owned(),
                    index: Index {
                        name: Some("doc_title_idx".to_owned()),
                        fields: vec!["title".to_owned()],
                        unique: false,
                        fulltext: true,
                        fulltext_analyzer: Some("english".to_owned()),
                        vector: false,
                        vector_dimension: None,
                        vector_dist: None,
                    },
                },
                MigrationOperation::CreateIndex {
                    table: "doc".to_owned(),
                    index: Index {
                        name: Some("doc_embedding_idx".to_owned()),
                        fields: vec!["embedding".to_owned()],
                        unique: false,
                        fulltext: false,
                        fulltext_analyzer: None,
                        vector: true,
                        vector_dimension: Some(1536),
                        vector_dist: Some(VectorDist::Cosine),
                    },
                },
                MigrationOperation::CreateIndex {
                    table: "doc".to_owned(),
                    index: Index {
                        name: Some("doc_vec_default_idx".to_owned()),
                        fields: vec!["vec".to_owned()],
                        unique: false,
                        fulltext: false,
                        fulltext_analyzer: None,
                        vector: true,
                        vector_dimension: Some(3),
                        vector_dist: None,
                    },
                },
            ],
            naming: NamingConvention::default(),
        };

        let rendered =
            SurrealDbRenderer::new().render_migration(&plan).expect("migration should render");

        assert!(rendered.contains(
            "DEFINE INDEX doc_title_idx ON doc FIELDS title FULLTEXT ANALYZER english BM25;"
        ));
        assert!(rendered.contains(
            "DEFINE INDEX doc_embedding_idx ON doc FIELDS embedding HNSW DIMENSION 1536 DIST COSINE;"
        ));
        assert!(rendered.contains(
            "DEFINE INDEX doc_vec_default_idx ON doc FIELDS vec HNSW DIMENSION 3 DIST EUCLIDEAN;"
        ));
    }

    #[test]
    fn renders_default_always_clause() {
        let convention = snake_case_fields();
        let naming = NamingContext::new(&convention, &[]);
        let field = Field {
            name: "status".to_owned(),
            field_type: FieldType::String,
            optional: false,
            unique: false,
            is_id: false,
            default_value: Some("'active'".to_owned()),
            default_always: true,
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
        };

        let rendered = render_define_field("user", &field, &naming);
        assert_eq!(rendered, "DEFINE FIELD status ON user TYPE string DEFAULT ALWAYS 'active';");
    }

    #[test]
    fn renderer_default_and_edge_schema_paths() {
        let _ = SurrealDbRenderer;

        let schema = DatabaseSchema {
            datasource: Datasource {
                provider: "surrealdb".to_owned(),
                url: None,
                namespace: None,
                database: None,
                extra: BTreeMap::new(),
            },
            naming: NamingConvention::default(),
            generators: Vec::new(),
            object_types: Vec::new(),
            models: vec![Model {
                name: "User".to_owned(),
                fields: vec![
                    Field {
                        name: "id".to_owned(),
                        field_type: FieldType::RecordId("User".to_owned()),
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
                    },
                    Field {
                        name: "posts".to_owned(),
                        field_type: FieldType::Array(Box::new(FieldType::Model("Post".to_owned()))),
                        optional: false,
                        unique: false,
                        is_id: false,
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
                        relation_name: Some("Likes".to_owned()),
                        attributes: BTreeMap::new(),
                    },
                ],
                table_mode: TableMode::Schemafull,
                permissions: Some("FULL".to_owned()),
                indexes: vec![core::Index {
                    name: None,
                    fields: vec!["email".to_owned()],
                    unique: false,
                    fulltext: false,
                    fulltext_analyzer: None,
                    vector: false,
                    vector_dimension: None,
                    vector_dist: None,
                }],
                attributes: BTreeMap::new(),
            }],
            edges: vec![Edge {
                name: "Likes".to_owned(),
                in_model: "User".to_owned(),
                out_model: "Post".to_owned(),
                fields: vec![Field {
                    name: "score".to_owned(),
                    field_type: FieldType::Int,
                    optional: false,
                    unique: false,
                    is_id: false,
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
                }],
                table_mode: TableMode::Schemafull,
                permissions: None,
                attributes: BTreeMap::new(),
            }],
        };

        let rendered = SurrealDbRenderer::new().render_schema(&schema).expect("render");
        assert!(rendered.contains(
            "DEFINE TABLE likes TYPE RELATION IN user OUT post SCHEMAFULL;"
        ));
        assert!(rendered.contains("DEFINE FIELD score ON likes TYPE int;"));
        assert!(rendered.contains("DEFINE TABLE user PERMISSIONS FULL;"));
        assert!(!rendered.contains("posts"));

        let alter_plan = MigrationPlan {
            name: "alter".to_owned(),
            operations: vec![MigrationOperation::AlterField {
                table: "user".to_owned(),
                field: schema.models[0].fields[0].clone(),
            }],
            naming: NamingConvention::default(),
        };
        let migration = SurrealDbRenderer::new().render_migration(&alter_plan).expect("alter");
        assert!(migration.contains("DEFINE FIELD id ON user TYPE record<user>;"));
    }

    #[test]
    fn renders_relation_create_table_migration() {
        let plan = MigrationPlan {
            name: "create_likes".to_owned(),
            operations: vec![MigrationOperation::CreateTable {
                name: "likes".to_owned(),
                mode: TableMode::Schemafull,
                relation: Some(core::RelationEndpoints {
                    in_table: "user".to_owned(),
                    out_table: "post".to_owned(),
                }),
            }],
            naming: NamingConvention::default(),
        };

        let rendered =
            SurrealDbRenderer::new().render_migration(&plan).expect("migration should render");
        assert!(rendered.contains(
            "DEFINE TABLE likes TYPE RELATION IN user OUT post SCHEMAFULL;"
        ));
    }

    #[test]
    fn default_renderer_and_schemaless_edge_permissions() {
        let renderer = SurrealDbRenderer::default();
        let mut schema = DatabaseSchema::empty();
        schema.datasource.provider = "surrealdb".into();
        schema.models.push(Model {
            name: "User".to_owned(),
            fields: vec![Field {
                name: "id".to_owned(),
                field_type: FieldType::RecordId("User".to_owned()),
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
            }],
            table_mode: TableMode::Schemaless,
            permissions: None,
            indexes: Vec::new(),
            attributes: BTreeMap::new(),
        });
        schema.edges.push(Edge {
            name: "Likes".to_owned(),
            in_model: "User".to_owned(),
            out_model: "User".to_owned(),
            fields: Vec::new(),
            table_mode: TableMode::Schemaless,
            permissions: Some("FULL".to_owned()),
            attributes: BTreeMap::new(),
        });

        let rendered = renderer.render_schema(&schema).expect("render");
        assert!(rendered.contains("DEFINE TABLE user SCHEMALESS;"));
        assert!(rendered.contains(
            "DEFINE TABLE likes TYPE RELATION IN user OUT user SCHEMALESS;"
        ));
        assert!(rendered.contains("DEFINE TABLE likes PERMISSIONS FULL;"));
    }

    #[test]
    fn renders_reference_and_computed_backlink() {
        let models = vec![
            Model {
                name: "User".to_owned(),
                fields: Vec::new(),
                table_mode: TableMode::Schemafull,
                permissions: None,
                indexes: Vec::new(),
                attributes: BTreeMap::new(),
            },
            Model {
                name: "Post".to_owned(),
                fields: Vec::new(),
                table_mode: TableMode::Schemafull,
                permissions: None,
                indexes: Vec::new(),
                attributes: BTreeMap::new(),
            },
        ];
        let convention = NamingConvention::default();
        let naming = NamingContext::new(&convention, &models);

        let author = Field {
            name: "author".to_owned(),
            field_type: FieldType::Model("User".to_owned()),
            optional: false,
            unique: false,
            is_id: false,
            default_value: None,
            default_always: false,
            value_expression: None,
            readonly: false,
            flexible: false,
            link_target: Some("User".to_owned()),
            link_name: Some("PostAuthor".to_owned()),
            on_delete: Some(core::OnDeleteAction::Cascade),
            link_storage: Some(core::LinkStorage::Stored),
            link_opposite_field: None,
            relation_name: None,
            attributes: BTreeMap::new(),
        };
        assert_eq!(
            render_define_field("post", &author, &naming),
            "DEFINE FIELD author ON post TYPE record<user> REFERENCE ON DELETE CASCADE;"
        );

        let posts = Field {
            name: "posts".to_owned(),
            field_type: FieldType::Array(Box::new(FieldType::Model("Post".to_owned()))),
            optional: false,
            unique: false,
            is_id: false,
            default_value: None,
            default_always: false,
            value_expression: None,
            readonly: false,
            flexible: false,
            link_target: Some("Post".to_owned()),
            link_name: Some("PostAuthor".to_owned()),
            on_delete: None,
            link_storage: Some(core::LinkStorage::Computed),
            link_opposite_field: Some("author".to_owned()),
            relation_name: None,
            attributes: BTreeMap::new(),
        };
        assert_eq!(
            render_define_field("user", &posts, &naming),
            "DEFINE FIELD posts ON user COMPUTED <~(post FIELD author);"
        );

        // No link_target and non-model type → fallback table name "unknown".
        let orphan = Field {
            name: "orphans".to_owned(),
            field_type: FieldType::String,
            optional: false,
            unique: false,
            is_id: false,
            default_value: None,
            default_always: false,
            value_expression: None,
            readonly: false,
            flexible: false,
            link_target: None,
            link_name: None,
            on_delete: None,
            link_storage: Some(core::LinkStorage::Computed),
            link_opposite_field: None,
            relation_name: None,
            attributes: BTreeMap::new(),
        };
        assert!(render_define_field("user", &orphan, &naming).contains("unknown"));
    }
}

mod introspect;

pub use introspect::{map_database_info, TableInfo};
