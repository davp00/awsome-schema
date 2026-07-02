#![allow(
    clippy::missing_errors_doc,
    clippy::needless_pass_by_value,
    clippy::missing_const_for_fn,
    clippy::redundant_closure,
    clippy::unnecessary_wraps,
    clippy::match_same_arms,
    clippy::needless_raw_string_hashes,
    clippy::needless_continue,
    clippy::redundant_clone
)]

mod lexer;
mod parse;

use core::{DatabaseSchema, DomainError};

pub use parse::parse_schema;

/// Parse an `awesome.schema` source string into a database-agnostic schema model.
pub fn parse(source: &str) -> Result<DatabaseSchema, DomainError> {
    parse_schema(source)
}

#[cfg(test)]
mod tests {
    use core::{DomainError, FieldType, TableMode};

    use super::*;

    const EXAMPLE: &str = include_str!("../../../examples/awesome.schema");

    #[test]
    fn parses_minimal_datasource() {
        let schema = parse(r#"datasource db { provider = "surrealdb" }"#).expect("should parse");
        assert_eq!(schema.datasource.provider, "surrealdb");
    }

    #[test]
    fn parses_model_with_id() {
        let schema = parse(r#"model User { id @id }"#).expect("should parse");
        let id = &schema.models[0].fields[0];
        assert!(id.is_id);
        assert_eq!(id.field_type, FieldType::RecordId("User".to_owned()));
    }

    #[test]
    fn rejects_record_id_type_syntax() {
        let error = parse(r#"model User { id RecordId<User> @id }"#).expect_err("should reject");
        assert!(matches!(error, DomainError::ParseError(_)));
    }

    #[test]
    fn parses_model_with_attributes() {
        parse(
            r#"model User {
  id @id
  @@table(schemafull)
}"#,
        )
        .expect("should parse");
    }

    #[test]
    fn parses_field_default_value() {
        let schema = parse(r#"model User { id @id createdAt datetime @default(time::now()) }"#)
            .expect("should parse");

        let field = schema.models[0]
            .fields
            .iter()
            .find(|field| field.name == "createdAt")
            .expect("createdAt field");

        assert_eq!(field.default_value.as_deref(), Some("time::now()"));
        assert!(!field.default_always);
        assert!(field.value_expression.is_none());
        assert!(!field.readonly);
    }

    #[test]
    fn parses_field_value_and_updated_attributes() {
        let schema = parse(
            r#"model User {
  id @id
  createdAt datetime @value(time::now()) @readonly
  updatedAt datetime @updated(time::now())
}"#,
        )
        .expect("should parse");

        let user = &schema.models[0];
        let created = user.fields.iter().find(|field| field.name == "createdAt").unwrap();
        assert_eq!(created.value_expression.as_deref(), Some("time::now()"));
        assert!(created.readonly);
        assert!(created.default_value.is_none());

        let updated = user.fields.iter().find(|field| field.name == "updatedAt").unwrap();
        assert_eq!(updated.value_expression.as_deref(), Some("time::now()"));
        assert!(!updated.readonly);
    }

    #[test]
    fn rejects_default_and_value_on_same_field() {
        let error = parse(
            r#"model User { id @id when datetime @default(time::now()) @value(time::now()) }"#,
        )
        .expect_err("should reject conflicting assignments");

        assert!(matches!(error, DomainError::ParseError(_)));
    }

    #[test]
    fn parses_naming_block() {
        let schema = parse(
            r#"datasource db { provider = "surrealdb" }
naming { tables = "snake_case" fields = "camelCase" }"#,
        )
        .expect("should parse");

        assert_eq!(schema.naming.tables, core::NamingCase::SnakeCase);
        assert_eq!(schema.naming.fields, Some(core::NamingCase::CamelCase));
    }

    #[test]
    fn preserves_field_naming_when_fields_not_set() {
        let schema = parse(
            r#"datasource db { provider = "surrealdb" }
naming { tables = "snake_case" }"#,
        )
        .expect("should parse");

        assert_eq!(schema.naming.tables, core::NamingCase::SnakeCase);
        assert_eq!(schema.naming.fields, None);
    }

    #[test]
    fn parses_example_schema_naming() {
        let schema = parse(EXAMPLE).expect("example schema should parse");
        assert_eq!(schema.naming.fields, None);
    }

    #[test]
    fn parses_example_schema() {
        let schema = parse(EXAMPLE).expect("example schema should parse");
        assert_eq!(schema.datasource.provider, "surrealdb");
        assert_eq!(schema.models.len(), 3);
        assert_eq!(schema.edges.len(), 1);

        let user = &schema.models[0];
        assert_eq!(user.name, "User");
        assert_eq!(user.table_mode, TableMode::Schemafull);
        assert!(user.fields.iter().any(|field| field.name == "email" && field.unique));

        let id = user.fields.iter().find(|field| field.name == "id").unwrap();
        assert!(id.is_id);
        assert_eq!(id.field_type, FieldType::RecordId("User".to_owned()));

        let email = user.fields.iter().find(|field| field.name == "email").unwrap();
        assert_eq!(email.field_type, FieldType::String);

        let posts = user.fields.iter().find(|field| field.name == "posts").unwrap();
        assert!(matches!(posts.field_type, FieldType::Array(_)));
        assert_eq!(posts.relation_name.as_deref(), Some("user_posts"));

        let created = user.fields.iter().find(|field| field.name == "createdAt").unwrap();
        assert_eq!(created.value_expression.as_deref(), Some("time::now()"));
        assert!(created.readonly);

        let updated = user.fields.iter().find(|field| field.name == "updatedAt").unwrap();
        assert_eq!(updated.value_expression.as_deref(), Some("time::now()"));
    }

    #[test]
    fn parses_edge_block() {
        let schema = parse(EXAMPLE).expect("example schema should parse");
        let likes = &schema.edges[0];
        assert_eq!(likes.name, "Likes");
        assert_eq!(likes.in_model, "User");
        assert_eq!(likes.out_model, "Post");
    }
}
