#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    Identifier(String),
    StringLiteral(String),
    LeftBrace,
    RightBrace,
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    Less,
    Greater,
    At,
    Comma,
    Equals,
    Question,
    Eof,
}

pub struct Lexer<'a> {
    input: &'a str,
    position: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self { input, position: 0 }
    }

    pub fn tokenize(mut self) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();
        loop {
            let token = self.next_token()?;
            let is_eof = matches!(token, Token::Eof);
            tokens.push(token);
            if is_eof {
                break;
            }
        }
        Ok(tokens)
    }

    fn next_token(&mut self) -> Result<Token, String> {
        self.skip_whitespace_and_comments();

        if self.is_at_end() {
            return Ok(Token::Eof);
        }

        let ch = self.peek_char();
        let token = match ch {
            '{' => {
                self.advance();
                Token::LeftBrace
            }
            '}' => {
                self.advance();
                Token::RightBrace
            }
            '(' => {
                self.advance();
                Token::LeftParen
            }
            ')' => {
                self.advance();
                Token::RightParen
            }
            '[' => {
                self.advance();
                Token::LeftBracket
            }
            ']' => {
                self.advance();
                Token::RightBracket
            }
            '<' => {
                self.advance();
                Token::Less
            }
            '>' => {
                self.advance();
                Token::Greater
            }
            '@' => {
                self.advance();
                Token::At
            }
            ',' => {
                self.advance();
                Token::Comma
            }
            '=' => {
                self.advance();
                Token::Equals
            }
            '?' => {
                self.advance();
                Token::Question
            }
            '"' => Token::StringLiteral(self.read_string()?),
            _ if ch.is_ascii_alphabetic() || ch == '_' => Token::Identifier(self.read_identifier()),
            _ => return Err(format!("unexpected character `{ch}` at byte {}", self.position)),
        };

        Ok(token)
    }

    fn skip_whitespace_and_comments(&mut self) {
        loop {
            while !self.is_at_end() {
                let ch = self.peek_char();
                if ch.is_whitespace() {
                    self.advance();
                } else {
                    break;
                }
            }

            if self.remaining_starts_with("//") {
                while !self.is_at_end() && self.peek_char() != '\n' {
                    self.advance();
                }
                continue;
            }

            break;
        }
    }

    fn read_identifier(&mut self) -> String {
        let start = self.position;
        while !self.is_at_end() {
            let ch = self.peek_char();
            if ch.is_ascii_alphanumeric() || ch == '_' || ch == ':' {
                self.advance();
            } else {
                break;
            }
        }
        self.input[start..self.position].to_owned()
    }

    fn read_string(&mut self) -> Result<String, String> {
        self.advance();
        let start = self.position;
        while !self.is_at_end() && self.peek_char() != '"' {
            self.advance();
        }
        if self.is_at_end() {
            return Err("unterminated string literal".to_owned());
        }
        let value = self.input[start..self.position].to_owned();
        self.advance();
        Ok(value)
    }

    fn peek_char(&self) -> char {
        self.input[self.position..].chars().next().unwrap_or('\0')
    }

    fn advance(&mut self) {
        if !self.is_at_end() {
            let ch = self.peek_char();
            self.position += ch.len_utf8();
        }
    }

    fn is_at_end(&self) -> bool {
        self.position >= self.input.len()
    }

    fn remaining_starts_with(&self, prefix: &str) -> bool {
        self.input[self.position..].starts_with(prefix)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenizes_model_header() {
        let tokens = Lexer::new("model User { id @id }").tokenize().expect("tokenize");
        assert!(matches!(tokens.first(), Some(Token::Identifier(name)) if name == "model"));
    }

    #[test]
    fn tokenizes_comments_and_punctuation() {
        let tokens =
            Lexer::new("// comment\nprovider = \"surrealdb\", ?").tokenize().expect("tokenize");
        assert!(tokens.iter().any(|token| matches!(token, Token::Comma)));
        assert!(tokens.iter().any(|token| matches!(token, Token::Question)));
    }

    #[test]
    fn rejects_unexpected_character() {
        let error = Lexer::new("model User { id @id $ }").tokenize().expect_err("bad char");
        assert!(error.contains("unexpected character"));
    }

    #[test]
    fn rejects_unterminated_string() {
        let error = Lexer::new(r#"provider = "open"#).tokenize().expect_err("unterminated");
        assert!(error.contains("unterminated string literal"));
    }
}
