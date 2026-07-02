#![allow(
    clippy::missing_errors_doc,
    clippy::needless_pass_by_value,
    clippy::missing_const_for_fn,
    clippy::map_unwrap_or,
    clippy::redundant_closure_for_method_calls,
    clippy::must_use_candidate,
    clippy::redundant_clone,
    clippy::collapsible_if
)]

use core::usecases::SchemaDiffPort;
use core::{DatabaseSchema, MigrationOperation, MigrationPlan, Model};

pub struct SchemaDiffer;

impl SchemaDiffer {
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl Default for SchemaDiffer {
    fn default() -> Self {
        Self::new()
    }
}

impl SchemaDiffPort for SchemaDiffer {
    fn diff(
        &self,
        from: Option<&DatabaseSchema>,
        to: &DatabaseSchema,
        name: &str,
    ) -> MigrationPlan {
        let mut operations = Vec::new();

        let from_models: &[Model] = from.map(|schema| schema.models.as_slice()).unwrap_or(&[]);
        let from_model_names =
            from_models.iter().map(|model| model.table_name()).collect::<Vec<_>>();

        for model in &to.models {
            let table = model.table_name();
            if !from_model_names.contains(&table) {
                operations.push(MigrationOperation::CreateTable {
                    name: table.clone(),
                    mode: model.table_mode,
                });
                operations.extend(field_operations_for_model(model));
                operations.extend(index_operations_for_model(model));
                if let Some(permission) = &model.permissions {
                    operations.push(MigrationOperation::CreatePermission {
                        table: table.clone(),
                        permission: permission.clone(),
                    });
                }
            } else if let Some(previous) = from_models.iter().find(|m| m.table_name() == table) {
                operations.extend(diff_model(previous, model));
            }
        }

        for previous in from_models {
            let table = previous.table_name();
            if !to.models.iter().any(|model| model.table_name() == table) {
                operations.push(MigrationOperation::DropTable { name: table });
            }
        }

        MigrationPlan { name: name.to_owned(), operations }
    }
}

pub fn diff_schemas(
    from: Option<&DatabaseSchema>,
    to: &DatabaseSchema,
    name: &str,
) -> MigrationPlan {
    SchemaDiffer::new().diff(from, to, name)
}

fn diff_model(from: &Model, to: &Model) -> Vec<MigrationOperation> {
    let table = to.table_name();
    let mut operations = Vec::new();

    if from.table_mode != to.table_mode {
        operations
            .push(MigrationOperation::AlterTable { name: table.clone(), mode: to.table_mode });
    }

    for field in &to.fields {
        if let Some(previous) = from.fields.iter().find(|f| f.name == field.name) {
            if previous != field {
                operations.push(MigrationOperation::AlterField {
                    table: table.clone(),
                    field: field.clone(),
                });
            }
        } else {
            operations.push(MigrationOperation::CreateField {
                table: table.clone(),
                field: field.clone(),
            });
        }
    }

    for field in &from.fields {
        if !to.fields.iter().any(|f| f.name == field.name) {
            operations.push(MigrationOperation::DropField {
                table: table.clone(),
                name: field.name.clone(),
            });
        }
    }

    for index in &to.indexes {
        if !from.indexes.contains(index) {
            operations.push(MigrationOperation::CreateIndex {
                table: table.clone(),
                index: index.clone(),
            });
        }
    }

    if from.permissions != to.permissions {
        if let Some(permission) = &to.permissions {
            operations.push(MigrationOperation::UpdatePermission {
                table: table.clone(),
                permission: permission.clone(),
            });
        }
    }

    operations
}

fn field_operations_for_model(model: &Model) -> Vec<MigrationOperation> {
    let table = model.table_name();
    model
        .fields
        .iter()
        .map(|field| MigrationOperation::CreateField { table: table.clone(), field: field.clone() })
        .collect()
}

fn index_operations_for_model(model: &Model) -> Vec<MigrationOperation> {
    let table = model.table_name();
    model
        .indexes
        .iter()
        .map(|index| MigrationOperation::CreateIndex { table: table.clone(), index: index.clone() })
        .collect()
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use core::{Datasource, Field, FieldType, TableMode};

    use super::*;

    fn user_model() -> Model {
        Model {
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
                    link_target: None,
                    relation_name: None,
                    attributes: BTreeMap::new(),
                },
                Field {
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
            ],
            table_mode: TableMode::Schemafull,
            permissions: Some("FULL".to_owned()),
            indexes: Vec::new(),
            attributes: BTreeMap::new(),
        }
    }

    fn target_schema() -> DatabaseSchema {
        DatabaseSchema {
            datasource: Datasource {
                provider: "surrealdb".to_owned(),
                url: None,
                namespace: None,
                database: None,
                extra: BTreeMap::new(),
            },
            generators: Vec::new(),
            models: vec![user_model()],
            edges: Vec::new(),
        }
    }

    #[test]
    fn creates_initial_migration_from_empty_snapshot() {
        let plan = diff_schemas(None, &target_schema(), "create_user");
        assert!(plan.operations.iter().any(|op| matches!(
            op,
            MigrationOperation::CreateTable { name, mode: TableMode::Schemafull }
            if name == "user"
        )));
        assert!(plan.operations.iter().any(|op| matches!(
            op,
            MigrationOperation::CreateField { table, field }
            if table == "user" && field.name == "email"
        )));
        assert!(plan.operations.iter().any(|op| matches!(
            op,
            MigrationOperation::CreatePermission { table, .. }
            if table == "user"
        )));
    }

    #[test]
    fn down_migration_reverses_up_changes() {
        let previous = target_schema();
        let mut current = target_schema();
        current.models[0].fields.push(Field {
            name: "name".to_owned(),
            field_type: FieldType::String,
            optional: false,
            unique: false,
            is_id: false,
            default_value: None,
            default_always: false,
            value_expression: None,
            readonly: false,
            link_target: None,
            relation_name: None,
            attributes: BTreeMap::new(),
        });

        let up = diff_schemas(Some(&previous), &current, "add_name");
        let down = diff_schemas(Some(&current), &previous, "add_name_down");

        assert!(up.operations.iter().any(|op| matches!(
            op,
            MigrationOperation::CreateField { table, field }
            if table == "user" && field.name == "name"
        )));
        assert!(down.operations.iter().any(|op| matches!(
            op,
            MigrationOperation::DropField { table, name }
            if table == "user" && name == "name"
        )));
    }

    #[test]
    fn detects_added_field() {
        let mut updated = target_schema();
        updated.models[0].fields.push(Field {
            name: "name".to_owned(),
            field_type: FieldType::String,
            optional: false,
            unique: false,
            is_id: false,
            default_value: None,
            default_always: false,
            value_expression: None,
            readonly: false,
            link_target: None,
            relation_name: None,
            attributes: BTreeMap::new(),
        });

        let plan = diff_schemas(Some(&target_schema()), &updated, "add_name");
        assert!(plan.operations.iter().any(|op| matches!(
            op,
            MigrationOperation::CreateField { table, field }
            if table == "user" && field.name == "name"
        )));
    }
}
