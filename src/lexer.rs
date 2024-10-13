use std::fmt;
use std::fmt::Debug;

use crate::scanner::{Scanner, ScannerState};

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
    Whitespace,        // special token to account for whitespace
    Text,
    Eof,
}

#[derive(Debug, Clone)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    line: usize,
}

impl Token {
    fn new(token_type: TokenType, lexeme: String, line: usize) -> Self {
        Token {
            token_type,
            lexeme,
            line,
        }
    }
}

pub trait Tokenizer: Scanner {
    type Token: Debug;
    type TokenType: Debug;
    type Error: std::error::Error;

    fn tokenize(&mut self) -> Result<Vec<Self::Token>, Self::Error>;
    fn scan_token(&mut self) -> Result<(), Self::Error>;
    fn add_token(&mut self, token_type: Self::TokenType);
}

#[derive(Debug)]
pub enum LexerError {
    EmptyToken(usize),
    UnexpectedCharacter(char, usize),
}

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LexerError::EmptyToken(line) => write!(f, "Empty token at line {}", line),
            LexerError::UnexpectedCharacter(c, line) => {
                write!(f, "Unexpected character '{}' at line {}", c, line)
            }
        }
    }
}

impl std::error::Error for LexerError {}

pub struct Lexer {
    source: String,
    tokens: Vec<Token>,
    state: ScannerState,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        Lexer {
            source,
            tokens: Vec::new(),
            state: ScannerState::new(),
        }
    }

    fn scan_token(&mut self) -> Result<(), LexerError> {
        let c = self.advance();

        let token_type = match c {
            '(' | ')' | '[' | ']' | ',' | '.' | '-' | '+' | ':' | ';' | '*' | '|' | '\'' | '"' => {
                self.handle_single_char(c)
            }
            '{' => self.handle_left_brace(),
            '}' => self.handle_right_brace(),
            '%' => self.handle_percent(),
            '#' => self.handle_hash(),
            '!' => self.handle_bang(),
            '=' => self.handle_equal(),
            '<' => self.handle_left_angle(),
            '>' => self.handle_right_angle(),
            '/' => self.handle_slash(),
            ' ' | '\r' | '\t' | '\n' => self.handle_whitespace(),
            _ => self.handle_text(),
        };

        self.add_token(token_type?);

        Ok(())
    }

    fn handle_single_char(&mut self, c: char) -> Result<TokenType, LexerError> {
        let token_type = match c {
            '(' => TokenType::LeftParen,
            ')' => TokenType::RightParen,
            '[' => TokenType::LeftBracket,
            ']' => TokenType::RightBracket,
            ',' => TokenType::Comma,
            '.' => TokenType::Dot,
            '-' => TokenType::Minus,
            '+' => TokenType::Plus,
            ':' => TokenType::Colon,
            ';' => TokenType::Semicolon,
            '*' => TokenType::Star,
            '|' => TokenType::Pipe,
            '\'' => TokenType::SingleQuote,
            '"' => TokenType::DoubleQuote,
            _ => return Err(LexerError::UnexpectedCharacter(c, self.state.line)),
        };
        Ok(token_type)
    }

    fn handle_left_brace(&mut self) -> Result<TokenType, LexerError> {
        let token_type = if self.advance_if_matches('{') {
            TokenType::DoubleLeftBrace
        } else if self.advance_if_matches('%') {
            TokenType::LeftBracePercent
        } else if self.advance_if_matches('#') {
            TokenType::LeftBraceHash
        } else {
            TokenType::LeftBrace
        };
        Ok(token_type)
    }

    fn handle_right_brace(&mut self) -> Result<TokenType, LexerError> {
        let token_type = if self.advance_if_matches('}') {
            TokenType::DoubleRightBrace
        } else {
            TokenType::RightBrace
        };
        Ok(token_type)
    }

    fn handle_percent(&mut self) -> Result<TokenType, LexerError> {
        let token_type = if self.advance_if_matches('}') {
            TokenType::PercentRightBrace
        } else {
            TokenType::Percent
        };
        Ok(token_type)
    }

    fn handle_hash(&mut self) -> Result<TokenType, LexerError> {
        let token_type = if self.advance_if_matches('}') {
            TokenType::HashRightBrace
        } else {
            TokenType::Hash
        };
        Ok(token_type)
    }

    fn handle_bang(&mut self) -> Result<TokenType, LexerError> {
        let token_type = if self.advance_if_matches('=') {
            TokenType::BangEqual
        } else {
            TokenType::Bang
        };
        Ok(token_type)
    }

    fn handle_equal(&mut self) -> Result<TokenType, LexerError> {
        let token_type = if self.advance_if_matches('=') {
            TokenType::DoubleEqual
        } else {
            TokenType::Equal
        };
        Ok(token_type)
    }

    fn handle_left_angle(&mut self) -> Result<TokenType, LexerError> {
        let token_type = if self.advance_if_matches('=') {
            TokenType::LeftAngleEqual
        } else {
            TokenType::LeftAngle
        };
        Ok(token_type)
    }

    fn handle_right_angle(&mut self) -> Result<TokenType, LexerError> {
        let token_type = if self.advance_if_matches('=') {
            TokenType::RightAngleEqual
        } else {
            TokenType::RightAngle
        };
        Ok(token_type)
    }

    fn handle_slash(&mut self) -> Result<TokenType, LexerError> {
        let token_type = if self.advance_if_matches('/') {
            while self.peek() != '\n' && !self.is_at_end() {
                self.advance();
            }
            TokenType::Text
        } else {
            TokenType::Slash
        };
        Ok(token_type)
    }

    fn handle_whitespace(&mut self) -> Result<TokenType, LexerError> {
        while !self.is_at_end() && self.peek().is_whitespace() {
            if self.peek() == '\n' {
                self.state.line += 1;
            }
            self.advance();
        }
        Ok(TokenType::Whitespace)
    }

    fn handle_text(&mut self) -> Result<TokenType, LexerError> {
        self.advance_while(|c| !Self::is_token_boundary(c));

        if self.state.start == self.state.current {
            Err(LexerError::EmptyToken(self.state.line))
        } else {
            Ok(TokenType::Text)
        }
    }

    fn advance_if_matches(&mut self, expected: char) -> bool {
        if self.is_at_end() || self.peek() != expected {
            false
        } else {
            self.state.current += 1;
            true
        }
    }

    fn advance_while<F>(&mut self, condition: F)
    where
        F: Fn(char) -> bool,
    {
        while !self.is_at_end() && condition(self.peek()) {
            if self.peek() == '\n' {
                self.state.line += 1;
            }
            self.advance();
        }
    }

    fn is_token_boundary(c: char) -> bool {
        const TOKEN_BOUNDARIES: &[char] = &[
            '(', ')', '[', ']', '{', '}', ',', '.', '-', '+', ':', ';', '*', '|', '%', '#', '!',
            '=', '<', '>', '/', ' ', '\r', '\t', '\n', '"', '\'',
        ];

        TOKEN_BOUNDARIES.contains(&c)
    }
}

impl Tokenizer for Lexer {
    type Token = Token;
    type TokenType = TokenType;
    type Error = LexerError;

    fn tokenize(&mut self) -> Result<Vec<Self::Token>, Self::Error> {
        while !self.is_at_end() {
            self.state.start = self.state.current;
            self.scan_token()?;
        }

        self.tokens
            .push(Token::new(TokenType::Eof, String::new(), self.state.line));
        Ok(self.tokens.clone())
    }

    fn scan_token(&mut self) -> Result<(), LexerError> {
        self.scan_token()
    }

    fn add_token(&mut self, token_type: Self::TokenType) {
        let text = self.source[self.state.start..self.state.current].to_string();
        if token_type != TokenType::Whitespace {
            self.tokens
                .push(Token::new(token_type, text, self.state.line));
        }
    }
}

impl Scanner for Lexer {
    type Item = char;

    fn advance(&mut self) -> Self::Item {
        let current_char = self.peek();
        self.state.current += 1;
        current_char
    }

    fn peek(&self) -> Self::Item {
        self.source.chars().nth(self.state.current).unwrap_or('\0')
    }

    fn peek_next(&self) -> Self::Item {
        self.source
            .chars()
            .nth(self.state.current + 1)
            .unwrap_or('\0')
    }

    fn is_at_end(&self) -> bool {
        self.state.current >= self.source.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tokenize(input: &str) -> Vec<Token> {
        let mut lexer = Lexer::new(input.to_string());
        match lexer.tokenize() {
            Ok(tokens) => {
                // Debug print all tokens
                for (i, token) in tokens.iter().enumerate() {
                    println!("{:?}", token)
                }

                tokens
            }
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
