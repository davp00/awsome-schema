use std::collections::BTreeMap;

use core::{
    DatabaseSchema, Datasource, DomainError, Edge, Field, FieldType, Generator, Index, Model,
    NamingCase, NamingConvention, ObjectTypeDefinition, ObjectTypeField, TableMode, VectorDist,
    nested_fields_from_object_body, normalize_schema,
};

use crate::lexer::{Lexer, Token};

pub fn parse_schema(source: &str) -> Result<DatabaseSchema, DomainError> {
    let tokens =
        Lexer::new(source).tokenize().map_err(|message| DomainError::ParseError(message))?;

    let mut parser = Parser::new(tokens);
    parser.parse_document()
}

struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, position: 0 }
    }

    fn parse_document(&mut self) -> Result<DatabaseSchema, DomainError> {
        let mut datasource = Datasource {
            provider: String::new(),
            url: None,
            namespace: None,
            database: None,
            extra: BTreeMap::new(),
        };
        let mut naming = NamingConvention::default();
        let mut generators = Vec::new();
        let mut object_types = Vec::new();
        let mut models = Vec::new();
        let mut edges = Vec::new();

        while !self.is_at(Token::Eof) {
            let keyword = self.expect_identifier()?;
            match keyword.as_str() {
                "datasource" => {
                    let name = self.expect_identifier()?;
                    datasource = self.parse_datasource(name)?;
                }
                "naming" => {
                    naming = self.parse_naming()?;
                }
                "generator" => {
                    let name = self.expect_identifier()?;
                    generators.push(self.parse_generator(name)?);
                }
                "model" => {
                    let name = self.expect_identifier()?;
                    models.push(self.parse_model(name)?);
                }
                "type" => {
                    let name = self.expect_identifier()?;
                    object_types.push(self.parse_object_type(name)?);
                }
                "edge" => {
                    let name = self.expect_identifier()?;
                    edges.push(self.parse_edge(name)?);
                }
                other => {
                    return Err(DomainError::ParseError(format!(
                        "unexpected top-level declaration `{other}`"
                    )));
                }
            }
        }

        let mut schema =
            DatabaseSchema { datasource, naming, generators, object_types, models, edges };
        normalize_schema(&mut schema)?;
        Ok(schema)
    }

    fn parse_naming(&mut self) -> Result<NamingConvention, DomainError> {
        self.expect(Token::LeftBrace)?;
        let mut naming = NamingConvention::default();

        while !self.is_at(Token::RightBrace) {
            let key = self.expect_identifier()?;
            self.expect(Token::Equals)?;
            let value = self.parse_value()?;
            match key.as_str() {
                "tables" => {
                    naming.tables = NamingCase::parse(&value).ok_or_else(|| {
                        DomainError::ParseError(format!("unknown naming case `{value}` for tables"))
                    })?;
                }
                "fields" => {
                    naming.fields = Some(NamingCase::parse(&value).ok_or_else(|| {
                        DomainError::ParseError(format!("unknown naming case `{value}` for fields"))
                    })?);
                }
                other => {
                    return Err(DomainError::ParseError(format!(
                        "unknown naming option `{other}`"
                    )));
                }
            }
        }

        self.expect(Token::RightBrace)?;
        Ok(naming)
    }

    fn parse_datasource(&mut self, _name: String) -> Result<Datasource, DomainError> {
        self.expect(Token::LeftBrace)?;
        let mut datasource = Datasource {
            provider: String::new(),
            url: None,
            namespace: None,
            database: None,
            extra: BTreeMap::new(),
        };

        while !self.is_at(Token::RightBrace) {
            let key = self.expect_identifier()?;
            self.expect(Token::Equals)?;
            let value = self.parse_value()?;
            match key.as_str() {
                "provider" => datasource.provider = value,
                "url" => datasource.url = Some(value),
                "namespace" => datasource.namespace = Some(value),
                "database" => datasource.database = Some(value),
                other => {
                    datasource.extra.insert(other.to_owned(), value);
                }
            }
        }

        self.expect(Token::RightBrace)?;
        Ok(datasource)
    }

    fn parse_generator(&mut self, _name: String) -> Result<Generator, DomainError> {
        self.expect(Token::LeftBrace)?;
        let mut generator =
            Generator { provider: String::new(), output: String::new(), extra: BTreeMap::new() };

        while !self.is_at(Token::RightBrace) {
            let key = self.expect_identifier()?;
            self.expect(Token::Equals)?;
            let value = self.parse_value()?;
            match key.as_str() {
                "provider" => generator.provider = value,
                "output" => generator.output = value,
                other => {
                    generator.extra.insert(other.to_owned(), value);
                }
            }
        }

        self.expect(Token::RightBrace)?;
        Ok(generator)
    }

    fn parse_model(&mut self, name: String) -> Result<Model, DomainError> {
        self.expect(Token::LeftBrace)?;
        let mut fields = Vec::new();
        let mut table_mode = TableMode::Schemafull;
        let mut permissions = None;
        let mut indexes = Vec::new();
        let mut attributes = BTreeMap::new();

        while !self.is_at(Token::RightBrace) {
            if self.is_at(Token::At)
                && matches!(self.tokens.get(self.position + 1), Some(Token::At))
            {
                self.advance();
                self.advance();
                let attr = self.parse_model_attribute()?;
                match attr.0.as_str() {
                    "table" => {
                        table_mode = match attr.1.as_str() {
                            "schemafull" => TableMode::Schemafull,
                            "schemaless" => TableMode::Schemaless,
                            other => {
                                return Err(DomainError::ParseError(format!(
                                    "unknown table mode `{other}`"
                                )));
                            }
                        };
                    }
                    "permissions" => permissions = Some(attr.1),
                    "index" => {
                        let fields = parse_index_fields(&attr.1)?;
                        indexes.push(self.parse_index_attrs(fields)?);
                    }
                    other => {
                        attributes.insert(other.to_owned(), attr.1);
                    }
                }
            } else {
                let (field, nested) = self.parse_field(Some(&name))?;
                fields.push(field);
                fields.extend(nested);
            }
        }

        self.expect(Token::RightBrace)?;
        Ok(Model { name, fields, table_mode, permissions, indexes, attributes })
    }

    fn parse_edge(&mut self, name: String) -> Result<Edge, DomainError> {
        self.expect(Token::LeftBrace)?;
        let mut in_model = String::new();
        let mut out_model = String::new();
        let mut fields = Vec::new();
        let mut table_mode = TableMode::Schemafull;
        let mut permissions = None;
        let mut attributes = BTreeMap::new();

        while !self.is_at(Token::RightBrace) {
            if self.is_at(Token::At)
                && matches!(self.tokens.get(self.position + 1), Some(Token::At))
            {
                self.advance();
                self.advance();
                let attr = self.parse_model_attribute()?;
                match attr.0.as_str() {
                    "table" => {
                        table_mode = match attr.1.as_str() {
                            "schemafull" => TableMode::Schemafull,
                            "schemaless" => TableMode::Schemaless,
                            other => {
                                return Err(DomainError::ParseError(format!(
                                    "unknown table mode `{other}`"
                                )));
                            }
                        };
                    }
                    "permissions" => permissions = Some(attr.1),
                    other => {
                        attributes.insert(other.to_owned(), attr.1);
                    }
                }
            } else {
                let field_name = self.expect_identifier()?;
                if field_name == "in" {
                    in_model = self.expect_identifier()?;
                } else if field_name == "out" {
                    out_model = self.expect_identifier()?;
                } else {
                    self.position -= 1;
                    let (field, nested) = self.parse_field(None)?;
                    fields.push(field);
                    fields.extend(nested);
                }
            }
        }

        self.expect(Token::RightBrace)?;
        Ok(Edge { name, in_model, out_model, fields, table_mode, permissions, attributes })
    }

    fn parse_object_type(&mut self, name: String) -> Result<ObjectTypeDefinition, DomainError> {
        let mut flexible = false;

        while self.is_at(Token::At)
            && !matches!(self.tokens.get(self.position + 1), Some(Token::At))
        {
            self.advance();
            let attr = self.parse_field_attribute()?;
            match attr.0.as_str() {
                "flexible" => flexible = true,
                other => {
                    return Err(DomainError::ParseError(format!(
                        "object type `{name}` does not support attribute `@{other}`"
                    )));
                }
            }
        }

        let fields = self.parse_object_type_body()?;
        Ok(ObjectTypeDefinition { name, flexible, fields })
    }

    fn parse_object_type_body(&mut self) -> Result<Vec<ObjectTypeField>, DomainError> {
        self.expect(Token::LeftBrace)?;
        let mut fields = Vec::new();

        while !self.is_at(Token::RightBrace) {
            fields.push(self.parse_object_type_field()?);
        }

        self.expect(Token::RightBrace)?;
        Ok(fields)
    }

    fn parse_object_type_field(&mut self) -> Result<ObjectTypeField, DomainError> {
        let name = self.parse_field_path()?;
        if name.contains('.') {
            return Err(DomainError::ParseError(format!(
                "object type field `{name}` must be a simple name, not a nested path"
            )));
        }

        let field_type = self.parse_field_type()?;
        let optional = self.match_token(Token::Question);

        if self.is_at(Token::At) {
            return Err(DomainError::ParseError(format!(
                "object type field `{name}` does not support field attributes"
            )));
        }

        Ok(ObjectTypeField { name, field_type, optional })
    }

    fn parse_field(
        &mut self,
        model_name: Option<&str>,
    ) -> Result<(Field, Vec<Field>), DomainError> {
        let name = self.parse_field_path()?;
        let field_type = if self.is_at(Token::At)
            && !matches!(self.tokens.get(self.position + 1), Some(Token::At))
        {
            None
        } else {
            Some(self.parse_field_type()?)
        };
        let optional = self.match_token(Token::Question);
        let ParsedFieldAttributes {
            optional,
            unique,
            is_id,
            default_value,
            default_always,
            value_expression,
            readonly,
            flexible,
            link_target,
            link_name,
            on_delete,
            relation_name,
            attributes,
        } = self.parse_field_attributes(&name, field_type.as_ref(), optional)?;

        validate_field_rules(
            &name,
            is_id,
            flexible,
            field_type.as_ref(),
            readonly,
            default_value.as_ref(),
            value_expression.as_ref(),
        )?;

        let field_type = match (field_type, is_id, model_name) {
            (Some(field_type), _, _) => field_type,
            (None, true, Some(model)) => FieldType::RecordId(model.to_owned()),
            (None, true, None) => {
                return Err(DomainError::ParseError(format!(
                    "field `{name}` with @id must be declared inside a model"
                )));
            }
            (None, false, _) => {
                return Err(DomainError::ParseError(format!("field `{name}` requires a type")));
            }
        };

        let nested = self.parse_inline_object_fields(&name, &field_type)?;

        Ok((
            Field {
                name,
                field_type,
                optional,
                unique,
                is_id,
                default_value,
                default_always,
                value_expression,
                readonly,
                flexible,
                link_target,
                link_name,
                on_delete,
                link_storage: None,
                link_opposite_field: None,
                relation_name,
                attributes,
            },
            nested,
        ))
    }

    fn parse_inline_object_fields(
        &mut self,
        name: &str,
        field_type: &FieldType,
    ) -> Result<Vec<Field>, DomainError> {
        if !self.is_at(Token::LeftBrace) {
            return Ok(Vec::new());
        }

        if *field_type != FieldType::Object {
            return Err(DomainError::ParseError(format!(
                "field `{name}` inline object body requires type object"
            )));
        }

        let body = self.parse_object_type_body()?;
        Ok(nested_fields_from_object_body(name, &body))
    }

    fn parse_field_attributes(
        &mut self,
        name: &str,
        field_type: Option<&FieldType>,
        mut optional: bool,
    ) -> Result<ParsedFieldAttributes, DomainError> {
        let mut unique = false;
        let mut is_id = false;
        let mut default_value = None;
        let mut default_always = false;
        let mut value_expression = None;
        let mut readonly = false;
        let mut flexible = false;
        let mut link_target = None;
        let mut link_name = None;
        let mut on_delete = None;
        let mut has_link = false;
        let mut relation_name = None;
        let mut attributes = BTreeMap::new();

        while self.is_at(Token::At)
            && !matches!(self.tokens.get(self.position + 1), Some(Token::At))
        {
            self.advance();
            let attr = self.parse_field_attribute()?;
            match attr.0.as_str() {
                "id" => is_id = true,
                "unique" => unique = true,
                "default" => default_value = Some(attr.1),
                "defaultAlways" => {
                    default_value = Some(attr.1);
                    default_always = true;
                }
                "value" | "updated" => value_expression = Some(attr.1),
                "readonly" => readonly = true,
                "flexible" => flexible = true,
                "link" => {
                    has_link = true;
                    if !attr.1.is_empty() {
                        link_name = Some(attr.1);
                    }
                    if let Some(ft) = field_type {
                        link_target = Some(link_target_from_type(ft));
                    }
                }
                "onDelete" => {
                    let Some(action) = core::OnDeleteAction::parse(&attr.1) else {
                        return Err(DomainError::ParseError(format!(
                            "field `{name}` @onDelete expects Ignore, Unset, Cascade, or Reject"
                        )));
                    };
                    on_delete = Some(action);
                }
                "relation" => relation_name = Some(attr.1),
                other => {
                    attributes.insert(other.to_owned(), attr.1);
                }
            }
        }

        // When `@link` is parsed with a known type, `link_target` is set above.
        // Attribute-before-type fields (`name @link`) have no type yet → error.
        if has_link && link_target.is_none() {
            return Err(DomainError::ParseError(format!(
                "field `{name}` @link requires a type when no target is given"
            )));
        }

        if has_link && relation_name.is_some() {
            return Err(DomainError::ParseError(format!(
                "field `{name}` cannot use both @link and @relation"
            )));
        }

        if on_delete.is_some() && !has_link {
            return Err(DomainError::ParseError(format!(
                "field `{name}` @onDelete requires @link"
            )));
        }

        if matches!(field_type, Some(FieldType::Model(_))) && !optional {
            optional = false;
        }

        Ok(ParsedFieldAttributes {
            optional,
            unique,
            is_id,
            default_value,
            default_always,
            value_expression,
            readonly,
            flexible,
            link_target,
            link_name,
            on_delete,
            relation_name,
            attributes,
        })
    }

    fn parse_field_path(&mut self) -> Result<String, DomainError> {
        let mut path = self.expect_identifier()?;
        while self.match_token(Token::Dot) {
            path.push('.');
            path.push_str(&self.expect_identifier()?);
        }
        Ok(path)
    }

    fn parse_field_type(&mut self) -> Result<FieldType, DomainError> {
        let base = self.expect_identifier()?;

        if base == "RecordId" {
            return Err(DomainError::ParseError(
                "RecordId<Model> is not supported; declare model ids as `id @id`".to_owned(),
            ));
        }

        let field_type = match base.as_str() {
            "string" => FieldType::String,
            "int" => FieldType::Int,
            "float" => FieldType::Float,
            "bool" => FieldType::Bool,
            "datetime" => FieldType::Datetime,
            "object" => FieldType::Object,
            other if other.chars().next().is_some_and(|ch| ch.is_ascii_uppercase()) => {
                if self.match_token(Token::LeftBracket) {
                    self.expect(Token::RightBracket)?;
                    FieldType::Array(Box::new(FieldType::Model(other.to_owned())))
                } else {
                    FieldType::Model(other.to_owned())
                }
            }
            other => FieldType::Custom(other.to_owned()),
        };

        if self.match_token(Token::LeftBracket) {
            self.expect(Token::RightBracket)?;
            return Ok(FieldType::Array(Box::new(field_type)));
        }

        Ok(field_type)
    }

    fn parse_field_attribute(&mut self) -> Result<(String, String), DomainError> {
        let name = self.expect_identifier()?;
        if self.match_token(Token::LeftParen) {
            let value = self.parse_expression()?;
            self.expect(Token::RightParen)?;
            Ok((name, value))
        } else {
            Ok((name, String::new()))
        }
    }

    fn parse_index_attrs(&mut self, fields: Vec<String>) -> Result<Index, DomainError> {
        let mut unique = false;
        let mut fulltext = false;
        let mut fulltext_analyzer = None;
        let mut vector = false;
        let mut vector_dimension = None;
        let mut vector_dist = None;

        while self.is_at(Token::At)
            && !matches!(self.tokens.get(self.position + 1), Some(Token::At))
        {
            self.advance();
            let (name, value) = self.parse_field_attribute()?;
            match name.as_str() {
                "unique" => unique = true,
                "fulltext" => {
                    if value.is_empty() {
                        return Err(DomainError::ParseError(
                            "@@index @fulltext requires an analyzer name".to_owned(),
                        ));
                    }
                    fulltext = true;
                    fulltext_analyzer = Some(value);
                }
                "vector" => {
                    if value.is_empty() {
                        return Err(DomainError::ParseError(
                            "@@index @vector requires a positive dimension".to_owned(),
                        ));
                    }
                    let dimension: u32 = value.parse().map_err(|_| {
                        DomainError::ParseError(format!(
                            "@@index @vector expects a positive integer dimension, got `{value}`"
                        ))
                    })?;
                    if dimension == 0 {
                        return Err(DomainError::ParseError(
                            "@@index @vector requires a positive dimension".to_owned(),
                        ));
                    }
                    vector = true;
                    vector_dimension = Some(dimension);
                }
                "dist" => {
                    if value.is_empty() {
                        return Err(DomainError::ParseError(
                            "@@index @dist requires Euclidean, Cosine, or Manhattan".to_owned(),
                        ));
                    }
                    let Some(dist) = VectorDist::parse(&value) else {
                        return Err(DomainError::ParseError(format!(
                            "@@index @dist expects Euclidean, Cosine, or Manhattan, got `{value}`"
                        )));
                    };
                    vector_dist = Some(dist);
                }
                other => {
                    return Err(DomainError::ParseError(format!(
                        "unknown @@index attribute `@{other}`"
                    )));
                }
            }
        }

        if fulltext && vector {
            return Err(DomainError::ParseError(
                "@@index cannot combine @fulltext and @vector".to_owned(),
            ));
        }
        if unique && (fulltext || vector) {
            return Err(DomainError::ParseError(
                "@@index @unique cannot combine with @fulltext or @vector".to_owned(),
            ));
        }
        if vector_dist.is_some() && !vector {
            return Err(DomainError::ParseError(
                "@@index @dist requires @vector".to_owned(),
            ));
        }
        if vector && fields.len() != 1 {
            return Err(DomainError::ParseError(
                "@@index @vector requires exactly one field".to_owned(),
            ));
        }

        Ok(Index {
            name: None,
            fields,
            unique,
            fulltext,
            fulltext_analyzer,
            vector,
            vector_dimension,
            vector_dist,
        })
    }

    fn parse_model_attribute(&mut self) -> Result<(String, String), DomainError> {
        let name = self.expect_identifier()?;
        if self.match_token(Token::LeftParen) {
            if self.is_at(Token::LeftBracket) {
                self.advance();
                let mut fields = Vec::new();
                while !self.is_at(Token::RightBracket) {
                    fields.push(self.expect_identifier()?);
                    if self.match_token(Token::Comma) {
                        continue;
                    }
                }
                self.expect(Token::RightBracket)?;
                self.expect(Token::RightParen)?;
                Ok((name, fields.join(",")))
            } else {
                let value = self.parse_expression()?;
                self.expect(Token::RightParen)?;
                Ok((name, value))
            }
        } else {
            Ok((name, String::new()))
        }
    }

    fn parse_expression(&mut self) -> Result<String, DomainError> {
        let head = match self.advance() {
            Token::StringLiteral(value) => return Ok(value),
            Token::Identifier(value) => value,
            other => {
                return Err(DomainError::ParseError(format!(
                    "expected expression, found `{other:?}`"
                )));
            }
        };

        if self.match_token(Token::LeftParen) {
            let mut args = Vec::new();
            if !self.is_at(Token::RightParen) {
                loop {
                    args.push(self.parse_expression()?);
                    if self.match_token(Token::Comma) {
                        continue;
                    }
                    break;
                }
            }
            self.expect(Token::RightParen)?;
            Ok(format!("{head}({})", args.join(", ")))
        } else {
            Ok(head)
        }
    }

    fn parse_value(&mut self) -> Result<String, DomainError> {
        match self.advance() {
            Token::StringLiteral(value) => Ok(value),
            Token::Identifier(value) => {
                if value == "env" && self.match_token(Token::LeftParen) {
                    let env_key = self.parse_value()?;
                    self.expect(Token::RightParen)?;
                    Ok(format!("env(\"{env_key}\")"))
                } else {
                    Ok(value)
                }
            }
            other => Err(DomainError::ParseError(format!("expected value, found `{other:?}`"))),
        }
    }

    fn expect_identifier(&mut self) -> Result<String, DomainError> {
        match self.advance() {
            Token::Identifier(value) => Ok(value),
            other => {
                Err(DomainError::ParseError(format!("expected identifier, found `{other:?}`")))
            }
        }
    }

    fn expect(&mut self, expected: Token) -> Result<(), DomainError> {
        if self.is_at(expected.clone()) {
            self.advance();
            Ok(())
        } else {
            Err(DomainError::ParseError(format!(
                "expected `{expected:?}`, found `{:?}`",
                self.peek()
            )))
        }
    }

    fn match_token(&mut self, expected: Token) -> bool {
        if self.is_at(expected.clone()) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn is_at(&self, expected: Token) -> bool {
        self.peek() == expected
    }

    fn peek(&self) -> Token {
        self.tokens.get(self.position).cloned().unwrap_or(Token::Eof)
    }

    fn advance(&mut self) -> Token {
        let token = self.peek();
        if !matches!(token, Token::Eof) {
            self.position += 1;
        }
        token
    }
}

#[allow(clippy::struct_excessive_bools)]
struct ParsedFieldAttributes {
    optional: bool,
    unique: bool,
    is_id: bool,
    default_value: Option<String>,
    default_always: bool,
    value_expression: Option<String>,
    readonly: bool,
    flexible: bool,
    link_target: Option<String>,
    link_name: Option<String>,
    on_delete: Option<core::OnDeleteAction>,
    relation_name: Option<String>,
    attributes: BTreeMap<String, String>,
}

fn parse_index_fields(raw: &str) -> Result<Vec<String>, DomainError> {
    let fields: Vec<String> = raw
        .split(',')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(ToOwned::to_owned)
        .collect();
    if fields.is_empty() {
        return Err(DomainError::ParseError(
            "@@index requires a non-empty field list, e.g. @@index([email])".to_owned(),
        ));
    }
    Ok(fields)
}

fn validate_field_rules(
    name: &str,
    is_id: bool,
    flexible: bool,
    field_type: Option<&FieldType>,
    readonly: bool,
    default_value: Option<&String>,
    value_expression: Option<&String>,
) -> Result<(), DomainError> {
    if default_value.is_some() && value_expression.is_some() {
        return Err(DomainError::ParseError(format!(
            "field `{name}` cannot use both @default and @value/@updated"
        )));
    }

    if readonly && value_expression.is_none() && default_value.is_none() {
        return Err(DomainError::ParseError(format!(
            "field `{name}` @readonly requires @value or @default"
        )));
    }

    if name.contains('.') && is_id {
        return Err(DomainError::ParseError(format!("nested field `{name}` cannot use @id")));
    }

    if flexible && !matches!(field_type, Some(FieldType::Object)) {
        return Err(DomainError::ParseError(format!(
            "field `{name}` @flexible requires type object"
        )));
    }

    Ok(())
}

fn link_target_from_type(field_type: &FieldType) -> String {
    match field_type {
        FieldType::Model(name) | FieldType::RecordId(name) => name.clone(),
        FieldType::Array(inner) => link_target_from_type(inner),
        other => other.base_surreal_type_name(),
    }
}

#[cfg(test)]
mod link_target_tests {
    use super::*;
    use core::FieldType;

    #[test]
    fn resolves_record_id_and_builtin_targets() {
        assert_eq!(link_target_from_type(&FieldType::RecordId("User".into())), "User");
        assert_eq!(link_target_from_type(&FieldType::String), "string");
    }

    #[test]
    fn resolves_array_model_targets() {
        assert_eq!(
            link_target_from_type(&FieldType::Array(Box::new(FieldType::Model("Post".into())))),
            "Post"
        );
    }
}
