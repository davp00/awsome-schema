//! SurrealDB CLI end-to-end tests.
//!
//! `cargo test -p e2e --test surreal` runs this binary. A later provider is another
//! `tests/<provider>.rs` binary so its driver is not required to compile this one.

#[path = "surreal/common/mod.rs"]
mod common;
#[path = "surreal/db_sync.rs"]
mod db_sync;
#[path = "surreal/graph_edges.rs"]
mod graph_edges;
#[path = "surreal/migrate.rs"]
mod migrate;
#[path = "surreal/offline_cli.rs"]
mod offline_cli;
#[path = "surreal/record_refs.rs"]
mod record_refs;
