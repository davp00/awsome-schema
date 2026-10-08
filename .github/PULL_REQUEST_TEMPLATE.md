## Summary

<!-- What changed, and why. -->

## Test plan

- [ ] `cargo test --workspace --locked --exclude e2e`
- [ ] `cargo test -p e2e --locked --test surreal` if live Surreal behavior changed
- [ ] `cd e2e/typescript && npm test` if the generated TypeScript client changed
