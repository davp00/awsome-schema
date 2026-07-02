use serde::{Deserialize, Serialize};

use super::naming::NamingConvention;
use super::{Field, Index, TableMode};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MigrationPlan {
    pub name: String,
    pub operations: Vec<MigrationOperation>,
    #[serde(default)]
    pub naming: NamingConvention,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum MigrationOperation {
    CreateTable { name: String, mode: TableMode },
    DropTable { name: String },
    AlterTable { name: String, mode: TableMode },
    CreateField { table: String, field: Field },
    DropField { table: String, name: String },
    AlterField { table: String, field: Field },
    CreateIndex { table: String, index: Index },
    DropIndex { table: String, name: String },
    CreateEvent { table: String, name: String, body: String },
    DropEvent { table: String, name: String },
    CreateFunction { name: String, body: String },
    DropFunction { name: String },
    CreatePermission { table: String, permission: String },
    UpdatePermission { table: String, permission: String },
}
