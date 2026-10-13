//! Database-specific schema and migration renderers.

pub mod surrealdb;

pub use surrealdb::SurrealDbRenderer;
pub use surrealdb::{TableInfo, map_database_info};
