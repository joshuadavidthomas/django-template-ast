use std::fmt;
use std::fmt::Debug;

use crate::scanner::Scanner;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    LeftParen,         // (
    RightParen,        // )
    LeftBrace,         // {
    RightBrace,        // }
    LeftBracket,       // [
    RightBracket,      // ]
    LeftAngle,         // <
    RightAngle,        // >
    Comma,             // ,
    Dot,               // .
    Minus,             // -
    Plus,              // +
    Colon,             // :
    Semicolon,         // ;
    Slash,             // /
    Star,              // *
    Bang,              // !
    Equal,             // =
    Pipe,              // |
    Percent,           // %
    Hash,              // #
    SingleQuote,       // '
    DoubleQuote,       // "
    DoubleLeftBrace,   // {{
    DoubleRightBrace,  // }}
    LeftBracePercent,  // {%
    PercentRightBrace, // %}
    LeftBraceHash,     // {#
    HashRightBrace,    // #}
    BangEqual,         // !=
    DoubleEqual,       // ==
    LeftAngleEqual,    // <=
    RightAngleEqual,   // =>
    Text,
    Eof,
}

#[derive(Debug, Clone)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Option<String>,
    line: usize,
}

impl Token {
    fn new(token_type: TokenType, lexeme: String, literal: Option<String>, line: usize) -> Self {
        Token {
            token_type,
            lexeme,
            literal,
            line,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{}",
            self.lexeme,
            if let Some(literal) = &self.literal {
                format!(" ({})", literal)
            } else {
                String::new()
            }
        )
    }
}

pub trait Tokenizer: Scanner {
    type Token: Debug;
    type TokenType: Debug;
    type Error: std::error::Error;

    fn tokenize(&mut self) -> Result<Vec<Self::Token>, Self::Error>;
    fn scan_token(&mut self) -> Result<(), Self::Error>;
    fn add_token(&mut self, token_type: Self::TokenType, literal: Option<String>);
}

#[derive(Debug)]
pub enum LexerError {
    EmptyToken(usize),
}

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LexerError::EmptyToken(line) => write!(f, "Empty token at line {}", line),
        }
    }
}

impl std::error::Error for LexerError {}

pub struct Lexer {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        Lexer {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    fn scan_token(&mut self) -> Result<(), LexerError> {
        let c = self.advance();

        let (token_type, literal) = match c {
            '(' => (TokenType::LeftParen, None),
            ')' => (TokenType::RightParen, None),
            '[' => (TokenType::LeftBracket, None),
            ']' => (TokenType::RightBracket, None),
            ',' => (TokenType::Comma, None),
            '.' => (TokenType::Dot, None),
            '-' => (TokenType::Minus, None),
            '+' => (TokenType::Plus, None),
            ':' => (TokenType::Colon, None),
            ';' => (TokenType::Semicolon, None),
            '*' => (TokenType::Star, None),
            '|' => (TokenType::Pipe, None),
            '\'' => (TokenType::SingleQuote, None),
            '"' => (TokenType::DoubleQuote, None),
            '{' => self.handle_left_brace(),
            '}' => self.handle_right_brace(),
            '%' => self.handle_percent(),
            '#' => self.handle_hash(),
            '!' => self.handle_bang(),
            '=' => self.handle_equal(),
            '<' => self.handle_left_angle(),
            '>' => self.handle_right_angle(),
            '/' => self.handle_slash(),
            ' ' | '\r' | '\t' | '\n' => self.handle_whitespace(c),
            _ => self.handle_text()?,
        };

        if token_type != TokenType::Text || literal.is_some() {
            self.add_token(token_type, literal);
        }

        Ok(())
    }

    fn handle_left_brace(&mut self) -> (TokenType, Option<String>) {
        let token_type = if self.advance_if_matches('{') {
            TokenType::DoubleLeftBrace
        } else if self.advance_if_matches('%') {
            TokenType::LeftBracePercent
        } else if self.advance_if_matches('#') {
            TokenType::LeftBraceHash
        } else {
            TokenType::LeftBrace
        };
        (token_type, None)
    }

    fn handle_right_brace(&mut self) -> (TokenType, Option<String>) {
        let token_type = if self.advance_if_matches('}') {
            TokenType::DoubleRightBrace
        } else {
            TokenType::RightBrace
        };
        (token_type, None)
    }

    fn handle_percent(&mut self) -> (TokenType, Option<String>) {
        let token_type = if self.advance_if_matches('}') {
            TokenType::PercentRightBrace
        } else {
            TokenType::Percent
        };
        (token_type, None)
    }

    fn handle_hash(&mut self) -> (TokenType, Option<String>) {
        let token_type = if self.advance_if_matches('}') {
            TokenType::HashRightBrace
        } else {
            TokenType::Hash
        };
        (token_type, None)
    }

    fn handle_bang(&mut self) -> (TokenType, Option<String>) {
        let token_type = if self.advance_if_matches('=') {
            TokenType::BangEqual
        } else {
            TokenType::Bang
        };
        (token_type, None)
    }

    fn handle_equal(&mut self) -> (TokenType, Option<String>) {
        let token_type = if self.advance_if_matches('=') {
            TokenType::DoubleEqual
        } else {
            TokenType::Equal
        };
        (token_type, None)
    }

    fn handle_left_angle(&mut self) -> (TokenType, Option<String>) {
        let token_type = if self.advance_if_matches('=') {
            TokenType::LeftAngleEqual
        } else {
            TokenType::LeftAngle
        };
        (token_type, None)
    }

    fn handle_right_angle(&mut self) -> (TokenType, Option<String>) {
        let token_type = if self.advance_if_matches('=') {
            TokenType::RightAngleEqual
        } else {
            TokenType::RightAngle
        };
        (token_type, None)
    }

    fn handle_slash(&mut self) -> (TokenType, Option<String>) {
        if self.advance_if_matches('/') {
            let start = self.current - 2;
            while self.peek() != '\n' && !self.is_at_end() {
                self.advance();
            }
            let comment = self.source[start..self.current].to_string();
            (TokenType::Text, Some(comment))
        } else {
            (TokenType::Slash, None)
        }
    }

    fn handle_whitespace(&mut self, c: char) -> (TokenType, Option<String>) {
        if c == '\n' {
            self.line += 1;
        }
        (TokenType::Text, None)
    }

    fn handle_text(&mut self) -> Result<(TokenType, Option<String>), LexerError> {
        while !self.is_at_end() && !self.is_delimiter(self.peek()) {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        let text = self.source[self.start..self.current].to_string();
        if text.is_empty() {
            Err(LexerError::EmptyToken(self.line))
        } else {
            Ok((TokenType::Text, Some(text)))
        }
    }

    fn is_delimiter(&self, c: char) -> bool {
        matches!(
            c,
            '(' | ')'
                | '['
                | ']'
                | '{'
                | '}'
                | ','
                | '.'
                | '-'
                | '+'
                | ':'
                | ';'
                | '*'
                | '|'
                | '%'
                | '#'
                | '!'
                | '='
                | '<'
                | '>'
                | '/'
                | ' '
                | '\r'
                | '\t'
                | '\n'
                | '"'
                | '\''
        )
    }

    fn advance_if_matches(&mut self, expected: char) -> bool {
        if self.is_at_end() || self.peek() != expected {
            false
        } else {
            self.current += 1;
            true
        }
    }
}

impl Tokenizer for Lexer {
    type Token = Token;
    type TokenType = TokenType;
    type Error = LexerError;

    fn tokenize(&mut self) -> Result<Vec<Self::Token>, Self::Error> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token()?;
        }

        self.tokens
            .push(Token::new(TokenType::Eof, String::new(), None, self.line));
        Ok(self.tokens.clone())
    }

    fn scan_token(&mut self) -> Result<(), LexerError> {
        self.scan_token()
    }

    fn add_token(&mut self, token_type: Self::TokenType, literal: Option<String>) {
        let text = self.source[self.start..self.current].to_string();
        self.tokens
            .push(Token::new(token_type, text, literal, self.line));
    }
}

impl Scanner for Lexer {
    type Item = char;

    fn advance(&mut self) -> Self::Item {
        let current_char = self.peek();
        self.current += 1;
        current_char
    }

    fn peek(&self) -> Self::Item {
        self.source.chars().nth(self.current).unwrap_or('\0')
    }

    fn peek_next(&self) -> Self::Item {
        self.source.chars().nth(self.current + 1).unwrap_or('\0')
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tokenize(input: &str) -> Vec<Token> {
        let mut lexer = Lexer::new(input.to_string());
        match lexer.tokenize() {
            Ok(tokens) => tokens,
            Err(e) => {
                eprintln!("Tokenization error: {:?}", e);
                eprintln!("Input that caused the error: {}", input);
                panic!("Tokenization failed. See error output above.");
            }
        }
    }

    #[test]
    fn test_opening_tag() {
        let tokens = tokenize("<html>");
        assert_eq!(tokens[0].token_type, TokenType::LeftAngle);
        assert_eq!(tokens[1].token_type, TokenType::Text);
        assert_eq!(tokens[2].token_type, TokenType::RightAngle);
    }

    #[test]
    fn test_closing_tag() {
        let tokens = tokenize("</body>");
        assert_eq!(tokens[0].token_type, TokenType::LeftAngle);
        assert_eq!(tokens[1].token_type, TokenType::Slash);
        assert_eq!(tokens[2].token_type, TokenType::Text);
        assert_eq!(tokens[3].token_type, TokenType::RightAngle);
    }

    #[test]
    fn test_html_attribute() {
        let tokens = tokenize(r#"<a href="link">"#);
        assert_eq!(tokens[0].token_type, TokenType::LeftAngle);
        assert_eq!(tokens[1].token_type, TokenType::Text);
        assert_eq!(tokens[2].token_type, TokenType::Text);
        assert_eq!(tokens[3].token_type, TokenType::Equal);
        assert_eq!(tokens[4].token_type, TokenType::DoubleQuote);
        assert_eq!(tokens[5].token_type, TokenType::Text);
        assert_eq!(tokens[6].token_type, TokenType::DoubleQuote);
        assert_eq!(tokens[7].token_type, TokenType::RightAngle);
    }

    #[test]
    fn test_django_variable() {
        let tokens = tokenize("{{ variable }}");
        assert_eq!(tokens[0].token_type, TokenType::DoubleLeftBrace);
        assert_eq!(tokens[1].token_type, TokenType::Text);
        assert_eq!(tokens[2].token_type, TokenType::DoubleRightBrace);
    }

    #[test]
    fn test_django_templatetag() {
        let tokens = tokenize("{% if condition %}");
        assert_eq!(tokens[0].token_type, TokenType::LeftBracePercent);
        assert_eq!(tokens[1].token_type, TokenType::Text);
        assert_eq!(tokens[2].token_type, TokenType::Text);
        assert_eq!(tokens[3].token_type, TokenType::PercentRightBrace);
    }

    #[test]
    fn test_django_comment() {
        let tokens = tokenize("{# This is a comment #}");
        assert_eq!(tokens[0].token_type, TokenType::LeftBraceHash);
        assert_eq!(tokens[1].token_type, TokenType::Text);
        assert_eq!(tokens[2].token_type, TokenType::Text);
        assert_eq!(tokens[3].token_type, TokenType::Text);
        assert_eq!(tokens[4].token_type, TokenType::Text);
        assert_eq!(tokens[5].token_type, TokenType::HashRightBrace);
    }

    #[test]
    fn test_django_filter() {
        let tokens = tokenize("{{ value|default:'default' }}");
        assert_eq!(tokens[0].token_type, TokenType::DoubleLeftBrace);
        assert_eq!(tokens[1].token_type, TokenType::Text);
        assert_eq!(tokens[2].token_type, TokenType::Pipe);
        assert_eq!(tokens[3].token_type, TokenType::Text);
        assert_eq!(tokens[4].token_type, TokenType::Colon);
        assert_eq!(tokens[5].token_type, TokenType::SingleQuote);
        assert_eq!(tokens[6].token_type, TokenType::Text);
        assert_eq!(tokens[7].token_type, TokenType::SingleQuote);
        assert_eq!(tokens[8].token_type, TokenType::DoubleRightBrace);
    }

    #[test]
    fn test_quoted_django_templatetag() {
        let tokens = tokenize(r#"'{% url "api:index" %}'"#);
        assert_eq!(tokens[0].token_type, TokenType::SingleQuote);
        assert_eq!(tokens[1].token_type, TokenType::LeftBracePercent);
        assert_eq!(tokens[2].token_type, TokenType::Text);
        assert_eq!(tokens[3].token_type, TokenType::DoubleQuote);
        assert_eq!(tokens[4].token_type, TokenType::Text);
        assert_eq!(tokens[5].token_type, TokenType::Colon);
        assert_eq!(tokens[6].token_type, TokenType::Text);
        assert_eq!(tokens[7].token_type, TokenType::DoubleQuote);
        assert_eq!(tokens[8].token_type, TokenType::PercentRightBrace);
        assert_eq!(tokens[9].token_type, TokenType::SingleQuote);
    }
}
