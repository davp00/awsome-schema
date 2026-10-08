# awesome-schema

Alpha client runtime and CLI launcher for [Awesome Schema](https://github.com/davp00/awsome-schema).

Generated TypeScript imports this package. The npm tarball does not contain a native binary. On install, the launcher downloads the one CLI build that matches this machine from the GitHub Release for the same version.

```bash
pnpm add awesome-schema@test
pnpm awesome-schema validate
```

`awesome-schema@test` is the newest `1.0.0-test.<commit>` build from `main`. `awesome-schema@alpha` stays the tagged alpha. `AWESOME_SCHEMA_BIN` points the launcher at a binary you already have and skips the download.
