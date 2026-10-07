use core::{DomainError, FieldType, LinkStorage, NamingCase, OnDeleteAction, TableMode};

use parser::{parse, print_schema};

#[test]
fn parses_generator_and_datasource_extras() {
    let schema = parse(
        r#"datasource db {
  provider = "surrealdb"
  url = env("DB_URL")
  username = "root"
}
generator client {
  provider = "typescript"
  output = "./gen"
  extra = "value"
}
model User { id @id }"#,
    )
    .expect("parse");

    assert_eq!(schema.datasource.extra.get("username").map(String::as_str), Some("root"));
    assert_eq!(schema.generators.len(), 1);
    assert_eq!(schema.generators[0].provider, "typescript");
    assert_eq!(schema.generators[0].extra.get("extra").map(String::as_str), Some("value"));
}

#[test]
fn parses_all_naming_cases() {
    let schema = parse(
        r#"datasource db { provider = "surrealdb" }
naming {
  tables = "PascalCase"
  fields = "kebab-case"
}"#,
    )
    .expect("parse");

    assert_eq!(schema.naming.tables, NamingCase::PascalCase);
    assert_eq!(schema.naming.fields, Some(NamingCase::KebabCase));
}

#[test]
fn parses_schemaless_table_and_custom_attributes() {
    let schema = parse(
        r#"model User {
  id @id
  @@table(schemaless)
  @@map("users")
}"#,
    )
    .expect("parse");

    assert_eq!(schema.models[0].table_mode, TableMode::Schemaless);
    assert_eq!(schema.models[0].attributes.get("map").map(String::as_str), Some("users"));
}

#[test]
fn parses_default_always_and_relation_fields() {
    let schema = parse(
        r#"model Post {
  id @id
  author User @link
  tags string[] @defaultAlways("active")
  posts Post[] @relation("post_children")
}"#,
    )
    .expect("parse");

    let post = &schema.models[0];
    let author = post.fields.iter().find(|f| f.name == "author").unwrap();
    assert_eq!(author.link_target.as_deref(), Some("User"));
    assert!(post.fields.iter().any(|f| f.name == "tags" && f.default_always));
    let relation = post.fields.iter().find(|f| f.name == "posts").unwrap();
    assert_eq!(relation.relation_name.as_deref(), Some("post_children"));
}

#[test]
fn parses_edge_schemaless_and_permissions() {
    let schema = parse(
        r#"edge Follows {
  in User
  out User
  score int
  @@table(schemaless)
  @@permissions("FULL")
}"#,
    )
    .expect("parse");

    let edge = &schema.edges[0];
    assert_eq!(edge.table_mode, TableMode::Schemaless);
    assert_eq!(edge.permissions.as_deref(), Some("FULL"));
    assert_eq!(edge.fields[0].name, "score");
}

#[test]
fn parses_custom_lowercase_field_type() {
    let schema = parse("model User { id @id status my_enum }").expect("parse");
    assert_eq!(schema.models[0].fields[1].field_type, FieldType::Custom("my_enum".to_owned()));
}

#[test]
fn rejects_unexpected_top_level_declaration() {
    let error = parse("unknown { }").expect_err("unexpected");
    assert!(matches!(error, DomainError::ParseError(_)));
}

#[test]
fn rejects_unknown_naming_table_case() {
    let error = parse(
        r#"datasource db { provider = "surrealdb" }
naming { tables = "UnknownCase" }"#,
    )
    .expect_err("bad tables case");
    assert!(matches!(error, DomainError::ParseError(_)));
}

#[test]
fn rejects_unknown_naming_fields_case() {
    let error = parse(
        r#"datasource db { provider = "surrealdb" }
naming { fields = "UnknownCase" }"#,
    )
    .expect_err("bad fields case");
    assert!(matches!(error, DomainError::ParseError(_)));
}

#[test]
fn rejects_unknown_naming_option() {
    let error = parse(
        r#"datasource db { provider = "surrealdb" }
naming { columns = "snake_case" }"#,
    )
    .expect_err("bad option");
    assert!(matches!(error, DomainError::ParseError(_)));
}

#[test]
fn rejects_unknown_table_mode_on_model() {
    let error = parse(
        r"model User {
  id @id
  @@table(invalid)
}",
    )
    .expect_err("bad mode");
    assert!(matches!(error, DomainError::ParseError(_)));
}

#[test]
fn rejects_unknown_table_mode_on_edge() {
    let error = parse(
        r"edge Likes {
  in User
  out Post
  @@table(invalid)
}",
    )
    .expect_err("bad edge mode");
    assert!(matches!(error, DomainError::ParseError(_)));
}

#[test]
fn rejects_link_without_type() {
    let error = parse("model User { id @id buddy @link }").expect_err("link");
    assert!(matches!(error, DomainError::ParseError(_)));
}

#[test]
fn rejects_readonly_without_value_or_default() {
    let error = parse("model User { id @id name string @readonly }").expect_err("readonly");
    assert!(matches!(error, DomainError::ParseError(_)));
}

#[test]
fn rejects_id_outside_model() {
    let error = parse(
        r"edge Likes {
  in User
  out Post
  id @id
}",
    )
    .expect_err("id outside model");
    assert!(matches!(error, DomainError::ParseError(_)));
}

#[test]
fn rejects_field_without_type() {
    let error = parse("model User { id @id email @unique }").expect_err("missing type");
    assert!(matches!(error, DomainError::ParseError(_)));
}

#[test]
fn rejects_non_literal_datasource_value() {
    let error = parse("datasource db { provider = @ }").expect_err("bad value");
    assert!(matches!(error, DomainError::ParseError(_)));
}

#[test]
fn parses_explicit_link_target() {
    let schema = parse("model Post { id @id author User @link(User) }").expect("parse");
    let author = schema.models[0].fields.iter().find(|f| f.name == "author").unwrap();
    assert_eq!(author.link_target.as_deref(), Some("User"));
    assert_eq!(author.link_name.as_deref(), Some("User"));
    assert_eq!(author.link_storage, Some(LinkStorage::Stored));
}

#[test]
fn parses_named_link_pair_and_on_delete() {
    let schema = parse(
        r#"
model User {
  id @id
  posts Post[] @link("PostAuthor")
}
model Post {
  id @id
  author User @link("PostAuthor") @onDelete(Cascade)
}
"#,
    )
    .expect("parse");
    let user = schema.models.iter().find(|m| m.name == "User").unwrap();
    let post = schema.models.iter().find(|m| m.name == "Post").unwrap();
    let posts = user.fields.iter().find(|f| f.name == "posts").unwrap();
    let author = post.fields.iter().find(|f| f.name == "author").unwrap();
    assert_eq!(posts.link_storage, Some(LinkStorage::Computed));
    assert_eq!(author.link_storage, Some(LinkStorage::Stored));
    assert_eq!(author.on_delete, Some(OnDeleteAction::Cascade));
    assert_eq!(posts.link_opposite_field.as_deref(), Some("author"));
    assert_eq!(posts.link_target.as_deref(), Some("Post"));
    assert_eq!(author.link_target.as_deref(), Some("User"));
}

#[test]
fn rejects_link_and_relation_on_same_field() {
    let error = parse("model User { id @id posts Post[] @link @relation(\"Likes\") }")
        .expect_err("both");
    assert!(matches!(error, DomainError::ParseError(_)));
}

#[test]
fn parses_model_attribute_without_value() {
    let schema = parse(
        r"model User {
  id @id
  @@map
}",
    )
    .expect("parse");
    assert_eq!(schema.models[0].attributes.get("map").map(String::as_str), Some(""));
}

#[test]
fn rejects_empty_index_field_list() {
    let error = parse(
        r"model User {
  id @id
  @@index([])
}",
    )
    .expect_err("empty index");
    assert!(matches!(error, DomainError::ParseError(_)));
}

#[test]
fn rejects_bare_index_without_fields() {
    let error = parse(
        r#"model User { id @id }
model UserSession {
  id @id
  user User @link("UserSession") @onDelete(Ignore) @@index
}"#,
    )
    .expect_err("bare @@index");
    assert!(matches!(error, DomainError::ParseError(_)));
}

#[test]
fn parses_expression_with_multiple_arguments() {
    let schema = parse("model User { id @id x string @default(fn(a, b)) }").expect("parse");
    assert_eq!(schema.models[0].fields[1].default_value.as_deref(), Some("fn(a, b)"));
}

#[test]
fn rejects_unexpected_token_in_attribute() {
    let error = parse("model User { id @id x string @default(]) }").expect_err("bad token");
    assert!(matches!(error, DomainError::ParseError(_)));
}

#[test]
fn rejects_non_string_datasource_value() {
    let error = parse(r"datasource db { provider = ] }").expect_err("bad value");
    assert!(matches!(error, DomainError::ParseError(_)));
}

#[test]
fn rejects_missing_equals_in_datasource() {
    let error = parse(r#"datasource db { provider "surrealdb" }"#).expect_err("missing equals");
    assert!(matches!(error, DomainError::ParseError(_)));
}

#[test]
fn rejects_invalid_datasource_value_token() {
    let error = parse(r"datasource db { provider = { }").expect_err("bad value");
    assert!(matches!(error, DomainError::ParseError(_)));
}

#[test]
fn rejects_unexpected_expression_token() {
    let error = parse(r"model User { id @id x string @default([) }").expect_err("bad expr");
    assert!(matches!(error, DomainError::ParseError(_)));
}

#[test]
fn rejects_missing_identifier_in_model() {
    let error = parse("model { id @id }").expect_err("missing name");
    assert!(matches!(error, DomainError::ParseError(_)));
}

#[test]
fn rejects_missing_closing_brace() {
    let error = parse("model User { id @id").expect_err("unclosed");
    assert!(matches!(error, DomainError::ParseError(_)));
}

#[test]
fn parses_edge_custom_attribute() {
    let schema = parse(
        r#"edge Likes {
  in User
  out Post
  @@map("user_likes")
}"#,
    )
    .expect("parse");
    assert_eq!(schema.edges[0].attributes.get("map").map(String::as_str), Some("user_likes"));
}

#[test]
fn parses_field_custom_attribute() {
    let schema = parse(r#"model User { id @id email string @map("mail") }"#).expect("parse");
    assert_eq!(schema.models[0].fields[1].attributes.get("map").map(String::as_str), Some("mail"));
}

#[test]
fn parses_index_with_multiple_fields() {
    let schema = parse(
        r"model User {
  id @id
  first string
  last string
  @@index([first, last])
}",
    )
    .expect("parse");
    assert_eq!(schema.models[0].indexes[0].fields, vec!["first", "last"]);
}

#[test]
fn parses_index_unique_fulltext_and_vector() {
    let schema = parse(
        r#"model Doc {
  id @id
  email string
  title string
  embedding float
  @@index([email]) @unique
  @@index([title]) @fulltext("english")
  @@index([embedding]) @vector(1536) @dist(Cosine)
}"#,
    )
    .expect("parse");
    let indexes = &schema.models[0].indexes;
    assert!(indexes[0].unique);
    assert!(!indexes[0].fulltext);
    assert!(!indexes[0].vector);

    assert!(indexes[1].fulltext);
    assert_eq!(indexes[1].fulltext_analyzer.as_deref(), Some("english"));
    assert!(!indexes[1].unique);

    assert!(indexes[2].vector);
    assert_eq!(indexes[2].vector_dimension, Some(1536));
    assert_eq!(indexes[2].vector_dist, Some(core::VectorDist::Cosine));
    assert!(!indexes[2].unique);
}

#[test]
fn prints_index_kinds_round_trip() {
    let source = r#"model Doc {
  id @id
  email string
  title string
  embedding float
  @@index([email]) @unique
  @@index([title]) @fulltext("english")
  @@index([embedding]) @vector(1536) @dist(Cosine)
}
"#;
    let schema = parse(source).expect("parse");
    let printed = print_schema(&schema);
    let again = parse(&printed).expect("reparse");
    assert_eq!(schema.models[0].indexes, again.models[0].indexes);
}

#[test]
fn rejects_index_unique_with_fulltext() {
    let error = parse(
        r#"model Doc {
  id @id
  title string
  @@index([title]) @unique @fulltext("english")
}"#,
    )
    .expect_err("combo");
    assert!(matches!(error, DomainError::ParseError(_)));
}

#[test]
fn rejects_index_vector_without_dimension() {
    let error = parse(
        r"model Doc {
  id @id
  embedding float
  @@index([embedding]) @vector
}",
    )
    .expect_err("missing dim");
    assert!(matches!(error, DomainError::ParseError(_)));
}

#[test]
fn rejects_index_dist_without_vector() {
    let error = parse(
        r"model Doc {
  id @id
  embedding float
  @@index([embedding]) @dist(Cosine)
}",
    )
    .expect_err("dist alone");
    assert!(matches!(error, DomainError::ParseError(_)));
}

#[test]
fn rejects_index_vector_with_multiple_fields() {
    let error = parse(
        r"model Doc {
  id @id
  a float
  b float
  @@index([a, b]) @vector(3)
}",
    )
    .expect_err("multi field vector");
    assert!(matches!(error, DomainError::ParseError(_)));
}

#[test]
fn parses_bare_identifier_value_in_datasource() {
    let schema = parse("datasource db { provider = surrealdb }").expect("parse");
    assert_eq!(schema.datasource.provider, "surrealdb");
}

#[test]
fn parses_numeric_default_expression() {
    let schema = parse(r"model User { id @id x int @default(1) }").expect("parse");
    assert_eq!(schema.models[0].fields[1].default_value.as_deref(), Some("1"));
}

#[test]
fn parses_flexible_object_and_nested_subdocument_fields() {
    let schema = parse(
        r"model User {
  id @id
  metadata object @flexible
  metadata.user_id int?
  metadata.source string
}",
    )
    .expect("parse");

    let user = &schema.models[0];
    let metadata = user.fields.iter().find(|f| f.name == "metadata").unwrap();
    assert!(metadata.flexible);
    assert_eq!(metadata.field_type, FieldType::Object);

    let user_id = user.fields.iter().find(|f| f.name == "metadata.user_id").unwrap();
    assert!(user_id.optional);
    assert_eq!(user_id.field_type, FieldType::Int);
}

#[test]
fn rejects_flexible_on_non_object_field() {
    let error = parse("model User { id @id name string @flexible }").expect_err("flexible");
    assert!(matches!(error, DomainError::ParseError(_)));
}

#[test]
fn rejects_nested_field_with_id() {
    let error = parse("model User { id @id metadata.user_id @id }").expect_err("nested id");
    assert!(matches!(error, DomainError::ParseError(_)));
}

#[test]
fn parses_inline_flexible_object_block() {
    let schema = parse(
        r"model User {
  id @id
  metadata object @flexible {
    user_id int?
    source string
  }
}",
    )
    .expect("parse");

    let user = &schema.models[0];
    let metadata = user.fields.iter().find(|field| field.name == "metadata").unwrap();
    assert!(metadata.flexible);
    assert_eq!(metadata.field_type, FieldType::Object);
    assert!(user.fields.iter().any(|field| field.name == "metadata.user_id"));
    assert!(user.fields.iter().any(|field| field.name == "metadata.source"));
}

#[test]
fn parses_reusable_object_type_reference() {
    let schema = parse(
        r"type UserMetadata @flexible {
  user_id int?
  source string
}
model User {
  id @id
  metadata UserMetadata
}",
    )
    .expect("parse");

    assert_eq!(schema.object_types.len(), 1);
    assert_eq!(schema.object_types[0].name, "UserMetadata");
    let metadata = schema.models[0].fields.iter().find(|field| field.name == "metadata").unwrap();
    assert!(metadata.flexible);
    assert!(schema.models[0].fields.iter().any(|field| field.name == "metadata.user_id"));
}

#[test]
fn rejects_inline_object_body_on_non_object_field() {
    let error = parse(
        r"model User {
  id @id
  name string {
    nested string
  }
}",
    )
    .expect_err("inline on string");
    assert!(matches!(error, DomainError::ParseError(_)));
}

#[test]
fn rejects_unknown_type_reference() {
    let error = parse(
        r"model User {
  id @id
  metadata UnknownType
}",
    )
    .expect_err("unknown type");
    assert!(matches!(error, DomainError::ValidationError(_)));
}

#[test]
fn rejects_bad_object_type_attribute() {
    let error = parse(
        r"type Meta @unknown {
  value string
}",
    )
    .expect_err("bad attr");
    assert!(matches!(error, DomainError::ParseError(msg) if msg.contains("does not support attribute")));
}

#[test]
fn rejects_object_type_nested_field_path() {
    let error = parse(
        r"type Meta {
  nested.path string
}",
    )
    .expect_err("nested path");
    assert!(matches!(error, DomainError::ParseError(msg) if msg.contains("nested path")));
}

#[test]
fn rejects_object_type_field_with_attributes() {
    let error = parse(
        r"type Meta {
  value string @unique
}",
    )
    .expect_err("field attr");
    assert!(matches!(error, DomainError::ParseError(msg) if msg.contains("does not support field attributes")));
}

#[test]
fn rejects_bad_on_delete_value() {
    let error = parse(r#"model Post { id @id author User @link @onDelete(Explode) }"#)
        .expect_err("bad onDelete");
    assert!(matches!(error, DomainError::ParseError(msg) if msg.contains("@onDelete")));
}

#[test]
fn rejects_on_delete_without_link() {
    let error = parse(r#"model Post { id @id author User @onDelete(Cascade) }"#)
        .expect_err("onDelete without link");
    assert!(matches!(error, DomainError::ParseError(msg) if msg.contains("@onDelete requires @link")));
}

#[test]
fn rejects_fulltext_without_analyzer() {
    let error = parse(
        r"model Doc {
  id @id
  title string
  @@index([title]) @fulltext
}",
    )
    .expect_err("fulltext");
    assert!(matches!(error, DomainError::ParseError(msg) if msg.contains("@fulltext requires")));
}

#[test]
fn rejects_vector_bad_and_zero_dimension() {
    let bad = parse(
        r"model Doc {
  id @id
  embedding float
  @@index([embedding]) @vector(abc)
}",
    )
    .expect_err("bad dim");
    assert!(matches!(bad, DomainError::ParseError(_)));

    let zero = parse(
        r"model Doc {
  id @id
  embedding float
  @@index([embedding]) @vector(0)
}",
    )
    .expect_err("zero dim");
    assert!(matches!(zero, DomainError::ParseError(msg) if msg.contains("positive dimension")));
}

#[test]
fn rejects_bad_dist_value() {
    let error = parse(
        r"model Doc {
  id @id
  embedding float
  @@index([embedding]) @vector(3) @dist(Chebyshev)
}",
    )
    .expect_err("bad dist");
    assert!(matches!(error, DomainError::ParseError(msg) if msg.contains("@dist")));
}

#[test]
fn rejects_dist_without_value() {
    let error = parse(
        r"model Doc {
  id @id
  embedding float
  @@index([embedding]) @vector(3) @dist
}",
    )
    .expect_err("bare dist");
    assert!(matches!(
        error,
        DomainError::ParseError(msg) if msg.contains("@dist requires")
    ));
}

#[test]
fn rejects_unknown_index_attribute() {
    let error = parse(
        r"model Doc {
  id @id
  title string
  @@index([title]) @sparse
}",
    )
    .expect_err("unknown attr");
    assert!(matches!(error, DomainError::ParseError(msg) if msg.contains("unknown @@index")));
}

#[test]
fn rejects_combine_fulltext_and_vector() {
    let error = parse(
        r#"model Doc {
  id @id
  title string
  @@index([title]) @fulltext("english") @vector(3)
}"#,
    )
    .expect_err("combo");
    assert!(matches!(error, DomainError::ParseError(msg) if msg.contains("cannot combine @fulltext and @vector")));
}
