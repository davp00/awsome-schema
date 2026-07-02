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
    use core::{FieldType, TableMode};

    use super::*;

    const EXAMPLE: &str = include_str!("../../../examples/awesome.schema");

    #[test]
    fn parses_minimal_datasource() {
        let schema = parse(r#"datasource db { provider = "surrealdb" }"#).expect("should parse");
        assert_eq!(schema.datasource.provider, "surrealdb");
    }

    #[test]
    fn parses_model_with_id() {
        parse(r#"model User { id RecordId<User> @id }"#).expect("should parse");
    }

    #[test]
    fn parses_model_with_attributes() {
        parse(
            r#"model User {
  id RecordId<User> @id
  @@table(schemafull)
}"#,
        )
        .expect("should parse");
    }

    #[test]
    fn parses_example_schema() {
        let schema = parse(EXAMPLE).expect("example schema should parse");
        assert_eq!(schema.datasource.provider, "surrealdb");
        assert_eq!(schema.models.len(), 2);
        assert_eq!(schema.edges.len(), 1);

        let user = &schema.models[0];
        assert_eq!(user.name, "User");
        assert_eq!(user.table_mode, TableMode::Schemafull);
        assert!(user.fields.iter().any(|field| field.name == "email" && field.unique));

        let email = user.fields.iter().find(|field| field.name == "email").unwrap();
        assert_eq!(email.field_type, FieldType::String);

        let posts = user.fields.iter().find(|field| field.name == "posts").unwrap();
        assert!(matches!(posts.field_type, FieldType::Array(_)));
        assert_eq!(posts.relation_name.as_deref(), Some("user_posts"));
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
