//! Push → pull → validate round-trip for models, links, edges, and indexes.

use std::fs;

use predicates::prelude::*;

use super::common::{TestProject, require_surrealdb};

fn roundtrip_schema(endpoint: &str) -> String {
    format!(
        r#"datasource db {{
  provider = "surrealdb"
  url      = "{endpoint}"
  namespace = "test"
  database  = "main"
}}

naming {{
  tables = "snake_case"
}}

model User {{
  id @id
  email string @unique
  posts Post[] @link("PostAuthor")
  liked Post[] @relation("Likes")
  @@index([email])
  @@table(schemafull)
}}

model Post {{
  id @id
  title string
  author User @link("PostAuthor") @onDelete(Cascade)
  @@index([title])
  @@table(schemafull)
}}

edge Likes {{
  in  User
  out Post
  score int
  @@table(schemafull)
}}
"#
    )
}

#[tokio::test]
async fn cli_db_push_pull_roundtrip_preserves_structure() {
    let Some((_container, endpoint)) =
        require_surrealdb("cli_db_push_pull_roundtrip_preserves_structure").await
    else {
        return;
    };

    let project = TestProject::new();
    project.write_schema(&roundtrip_schema(&endpoint));

    project.awesome_schema_cmd().arg("db").arg("push").assert().success();

    project
        .awesome_schema_cmd()
        .arg("db")
        .arg("pull")
        .arg("--force")
        .assert()
        .success()
        .stdout(predicate::str::contains("Pulled"));

    let pulled = fs::read_to_string(project.path("awesome.schema")).expect("pulled schema");
    assert!(pulled.contains("model User"), "missing User:\n{pulled}");
    assert!(pulled.contains("model Post"), "missing Post:\n{pulled}");
    assert!(pulled.contains("edge Likes"), "missing Likes edge:\n{pulled}");
    assert!(
        pulled.contains("@link(\"PostAuthor\")"),
        "missing @link pair:\n{pulled}"
    );
    assert!(
        pulled.contains("@relation(\"Likes\")"),
        "missing @relation nav:\n{pulled}"
    );
    assert!(
        pulled.contains("@@index") || pulled.contains("@unique"),
        "missing index kinds:\n{pulled}"
    );

    // Residual loss: named object types / generators are DSL-only and kept via merge
    // from the preserve schema, not reconstructed from INFO.
    project.awesome_schema_cmd().arg("validate").assert().success();
}
