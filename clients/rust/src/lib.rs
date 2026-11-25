//! Shared SurrealDB runtime for generated Awesome Schema Rust clients.
//!
//! Generated code serializes typed arguments to JSON and calls these functions.
//! SurrealQL lives here, not in the emitter.

#![allow(
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::must_use_candidate,
    clippy::too_many_lines,
    clippy::too_many_arguments,
    clippy::similar_names,
    clippy::items_after_statements,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_possible_wrap,
    clippy::needless_pass_by_value,
    clippy::missing_const_for_fn,
    clippy::return_self_not_must_use,
    clippy::struct_excessive_bools,
    clippy::fn_params_excessive_bools,
    clippy::option_if_let_else,
    clippy::manual_let_else,
    clippy::redundant_closure_for_method_calls,
    clippy::map_unwrap_or,
    clippy::needless_raw_string_hashes,
    clippy::redundant_pub_crate,
    clippy::doc_markdown,
    clippy::double_must_use
)]

mod error;
mod id;
mod jsonutil;
mod meta;
mod nested;
mod ops;
mod session;
mod sql;

pub use error::ClientError;
pub use id::{ParsedRecordId, as_record_id, normalize_thing, parse_record_id, record_id};
pub use jsonutil::JsonMap;
pub use meta::{
    FieldSelectMeta, FieldWriteMeta, RelationDir, ScalarFilter, SelectMeta, WriteMeta,
    bind_schema_meta,
};
pub use nested::{NestedOp, is_nested_write_bag, render_nested_ops};
pub use ops::{
    DbHandle, ManyResult, MutationReturn, SingleResult, SurrealOps, count_records, create_many_via,
    create_record, create_with_nested, decode, delete_many_records, delete_one_record,
    delete_record, execute_query, execute_raw, find_many_records, find_unique_record,
    group_by_records, query_rows, relate_edge, select_record, select_record_related,
    update_many_records, update_one_record, update_record, update_with_nested, upsert_record,
    with_write_transaction,
};
pub use session::{LiveAction, LiveEvent, LiveHandle, Session};
pub use sql::{WhereCtx, append_limit_start, build_order_by, build_projection, build_where};

pub use nested::{collect_computed, collect_edge, split_and_resolve_write_data};
