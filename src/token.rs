use crate::scanner::Scanner;
use std::fmt::Debug;

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
pub struct Token<'a> {
    pub token_type: TokenType,
    pub lexeme: &'a str,
    pub line: usize,
}

impl<'a> Token<'a> {
    pub fn new(token_type: TokenType, lexeme: &'a str, line: usize) -> Self {
        Token {
            token_type,
            lexeme,
            line,
        }
    }
}

pub trait Tokenizer<'a>: Scanner {
    type Token: Debug;
    type TokenType: Debug;

    fn tokenize(&mut self) -> Result<Vec<Self::Token>, Self::Error>;
    fn next_token(&mut self) -> Result<(Self::TokenType, &'a str), Self::Error>;
    fn add_token(&mut self, token_type: Self::TokenType, text: &'a str);
}
