use std::collections::BTreeMap;

use core::{
    DatabaseSchema, Datasource, DomainError, Edge, Field, FieldType, Generator, Index, Model,
    TableMode,
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
        let mut generators = Vec::new();
        let mut models = Vec::new();
        let mut edges = Vec::new();

        while !self.is_at(Token::Eof) {
            let keyword = self.expect_identifier()?;
            match keyword.as_str() {
                "datasource" => {
                    let name = self.expect_identifier()?;
                    datasource = self.parse_datasource(name)?;
                }
                "generator" => {
                    let name = self.expect_identifier()?;
                    generators.push(self.parse_generator(name)?);
                }
                "model" => {
                    let name = self.expect_identifier()?;
                    models.push(self.parse_model(name)?);
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

        Ok(DatabaseSchema { datasource, generators, models, edges })
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
                    "index" => indexes.push(Index {
                        name: None,
                        fields: parse_index_fields(&attr.1)?,
                        unique: false,
                        fulltext: false,
                        vector: false,
                    }),
                    other => {
                        attributes.insert(other.to_owned(), attr.1);
                    }
                }
            } else {
                fields.push(self.parse_field()?);
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
                    fields.push(self.parse_field()?);
                }
            }
        }

        self.expect(Token::RightBrace)?;
        Ok(Edge { name, in_model, out_model, fields, table_mode, permissions, attributes })
    }

    fn parse_field(&mut self) -> Result<Field, DomainError> {
        let name = self.expect_identifier()?;
        let field_type = self.parse_field_type()?;
        let mut optional = self.match_token(Token::Question);
        let mut unique = false;
        let mut is_id = false;
        let mut default_value = None;
        let mut default_always = false;
        let mut value_expression = None;
        let mut readonly = false;
        let mut link_target = None;
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
                "link" => {
                    link_target = Some(if attr.1.is_empty() {
                        link_target_from_type(&field_type)
                    } else {
                        attr.1
                    });
                }
                "relation" => relation_name = Some(attr.1),
                other => {
                    attributes.insert(other.to_owned(), attr.1);
                }
            }
        }

        if matches!(field_type, FieldType::Model(_)) && !optional {
            optional = false;
        }

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

        Ok(Field {
            name,
            field_type,
            optional,
            unique,
            is_id,
            default_value,
            default_always,
            value_expression,
            readonly,
            link_target,
            relation_name,
            attributes,
        })
    }

    fn parse_field_type(&mut self) -> Result<FieldType, DomainError> {
        let base = self.expect_identifier()?;

        if base == "RecordId" && self.match_token(Token::Less) {
            let inner = self.expect_identifier()?;
            self.expect(Token::Greater)?;
            return Ok(FieldType::RecordId(inner));
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

fn parse_index_fields(raw: &str) -> Result<Vec<String>, DomainError> {
    if raw.is_empty() {
        return Ok(Vec::new());
    }
    Ok(raw.split(',').map(str::trim).map(ToOwned::to_owned).collect())
}

fn link_target_from_type(field_type: &FieldType) -> String {
    match field_type {
        FieldType::Model(name) => name.clone(),
        FieldType::RecordId(name) => name.clone(),
        other => other.base_surreal_type_name(),
    }
}
