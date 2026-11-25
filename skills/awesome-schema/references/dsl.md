# Awesome Schema DSL (SurrealDB)

Keep schemas small and valid: every project needs a `datasource`; at least one `model` or `edge`; each model needs `id @id`.

## Datasource and generator

```prisma
datasource db {
  provider = "surrealdb"
  url      = env("SURREALDB_URL")   // or "ws://127.0.0.1:8000"
  namespace = "app"
  database  = "main"
}

naming {
  tables = "snake_case"
}

generator client {
  provider = "typescript"
  output   = "./generated"
}
```

Only `surrealdb` is supported for SQL render, `db push`/`pull`, migrate apply, and client emit. `generator.provider` is `typescript` or `rust`.

## Models, links, edges

```prisma
model User {
  id    @id
  email string @unique
  posts Post[] @link("PostAuthor")     // computed opposite
  liked Post[] @relation("Likes")      // nav over edge Likes

  @@table(schemafull)
}

model Post {
  id     @id
  title  string
  author User @link("PostAuthor") @onDelete(Cascade)

  @@index([title])
}

edge Likes {
  in  User
  out Post
}
```

| Construct | Meaning |
|---|---|
| `@link` / `@link("Name")` | Surreal `REFERENCE`; named pairs share Prisma-style opposite; list side becomes `COMPUTED <~` |
| `@onDelete(Cascade\|Unset\|Reject\|Ignore)` | On stored `@link` (default Ignore) |
| `@relation("EdgeName")` | Navigation only — must name an existing `edge`; not stored as a field in Surreal |
| `edge` | `DEFINE TABLE … TYPE RELATION IN … OUT …` |
| `@@index([fields])` | Optional trailing `@unique`, `@fulltext("analyzer")`, `@vector(N)`, `@dist(…)` |

## Field assignments

| DSL | Surreal |
|---|---|
| `@default(expr)` | `DEFAULT` |
| `@defaultAlways(expr)` | `DEFAULT ALWAYS` |
| `@value(expr)` / `@updated(expr)` | `VALUE` |
| `@readonly` | `READONLY` |
| `object @flexible` | `FLEXIBLE` (type must include object) |

Reusable shapes: `type Name @flexible { … }` then `field Name` on a model.

## Push vs pull vs migrate

- **`db push`** — render schema SurrealQL and execute on the live DB (dev-friendly).
- **`db pull --force`** — introspect into DSL. Preserves datasource/naming/generator/types from disk. Synthesizes `@relation` nav from RELATION edges when possible. Needs `--force` to overwrite non-empty schema files. `--split-by-table` writes one file per model/edge under `schema/`.
- **`migrate dev`** — diff vs last snapshot; write up/down + snapshot.
- **`migrate apply` / `rollback` / `status`** — ledger table `_awesome_migrations`.

Fulltext: analyzer named in `@fulltext("…")` must already exist in the DB or `db push` fails.

## Split schema directories

```bash
awesome-schema --schema schema validate
awesome-schema --schema schema db pull --split-by-table --force
```
