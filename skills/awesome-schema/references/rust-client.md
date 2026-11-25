# Rust generated client

## Prerequisites

The generated file calls the shared runtime crate `awesome-schema-surreal` (library `awesome_schema_surreal`) and the official Rust SDK. This alpha cut does not publish that runtime to crates.io. Depend on it from this repository (or git) until a crates.io release exists.

```toml
[dependencies]
awesome-schema-surreal = { version = "0.1.0-alpha.1", path = "path/to/awsome-schema/clients/rust" }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
surrealdb = { version = "4", default-features = false, features = ["protocol-ws"] }
tokio = { version = "1", features = ["rt-multi-thread", "macros"] }
```

Supported servers: SurrealDB **3.3+** and **4.x**, through `surrealdb = "4"`.

Schema must include a Surreal datasource and:

```prisma
generator client {
  provider = "rust"
  output   = "./src/generated.rs"   # a directory becomes client.rs
}
```

## Generate

```bash
awesome-schema generate --target rust
# print only:
awesome-schema generate --target rust --stdout
```

`--stdout` does not write `generator.output`. A missing `provider = "rust"` generator is an error unless you pass `--stdout`.

## What the client can express

Behavior matches the TypeScript client. Rust cannot express `UserGetPayload<S>`, so methods are generic over `T: DeserializeOwned`:

- No `select`: deserialize into the stored record (`User`). Computed links and relations are omitted from that struct.
- With `select`: deserialize into `serde_json::Value` or your own subset struct. `*Selected` is the fully expanded link shape, not a projection.
- `create`, `update`, and `delete` return `T`. `create_many`, `update_many`, and `delete_many` return `ManyResult` (`{ count }` or rows). `upsert` returns `SingleResult`.
- Names are snake_case: `find_unique`, `find_many`, `count`, `group_by`, `create_many`, `update_many`, `delete_many`, `query_raw`, `execute_raw`, `live_sql`.
- `live()` is session-only (`client.user().live()`). `transaction` does not expose live.
- Arguments serialize with empty options omitted, into the same JSON the runtime already accepts. Record-id filters use `type::record`.

Open a signed-in `awesome_schema_surreal::Session` and pass `Arc<Session<_>>` to `create_client`.

## Still deferred

Same gaps as the TypeScript client: multi-hop nested writes, nested bags on the edge API, top-level `aggregate()`, relation-field `count` select, `skipDuplicates`, single-script `BEGIN`/`COMMIT`, nested transactions, cursor pagination, multi-hop select, and validators.
