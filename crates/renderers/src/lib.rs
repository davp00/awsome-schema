//! Database-specific schema and migration renderers.
//!
//! Each provider lives in its own module (e.g. [`surrealdb`]). When the project
//! grows, additional providers can be added here or split into dedicated crates
//! under `crates/providers/`.

pub mod surrealdb;

pub use surrealdb::SurrealDbRenderer;
