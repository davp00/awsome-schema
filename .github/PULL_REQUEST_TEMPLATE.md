## Summary

<!-- What changed, and why. -->

## Test plan

- [ ] `cargo test --workspace --locked --exclude e2e`
- [ ] `cargo build -p awesome-schema --locked` then `SURREAL_IMAGE=surrealdb/surrealdb:v3.3.0 cargo test -p e2e --locked --test surreal` (and same with `v4`) if live Surreal behavior changed
- [ ] `SURREAL_IMAGE=… pnpm --filter awesome-schema-typescript-e2e test` on both matrix images if the generated TypeScript client changed
