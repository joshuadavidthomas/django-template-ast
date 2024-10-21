use crate::error::TokenError;
use std::fmt::Debug;
use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenType {
    LeftAngle,             // <
    RightAngle,            // >
    Comma,                 // ,
    Dot,                   // .
    Dash,                  // -
    Plus,                  // +
    Colon,                 // :
    Slash,                 // /
    Bang,                  // !
    Equal,                 // =
    Pipe,                  // |
    Percent,               // %
    SingleQuote,           // '
    DoubleQuote,           // "
    DoubleLeftBrace,       // {{
    DoubleRightBrace,      // }}
    LeftBracePercent,      // {%
    PercentRightBrace,     // %}
    LeftBraceHash,         // {#
    HashRightBrace,        // #}
    BangEqual,             // !=
    DoubleEqual,           // ==
    LeftAngleEqual,        // <=
    RightAngleEqual,       // =>
    LeftAngleBangDashDash, // <!--
    DashDashRightAngle,    // -->
    SlashRightAngle,       // />
    DoubleSlash,           // //
    SlashStar,             // /*
    StarSlash,             // */
    Whitespace,            // special token to account for whitespace
    Text,
    Eof,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: usize,
}

impl<'a> Token {
    pub fn new(token_type: TokenType, lexeme: &'a str, line: usize) -> Self {
        Token {
            token_type,
            lexeme: String::from(lexeme),
            line,
        }
    }

    pub fn eof(line: usize) -> Self {
        Token {
            token_type: TokenType::Eof,
            lexeme: "".to_string(),
            line,
        }
    }

    pub fn size(&self) -> usize {
        self.lexeme.chars().count()
    }

    pub fn lines(&self) -> usize {
        match self.token_type {
            TokenType::Whitespace => self
                .lexeme
                .chars()
                .filter(|&c| c == '\n' || c == '\r')
                .count(),
            _ => 0,
        }
    }

    pub fn is_throwaway(&self) -> bool {
        self.is_token_type(TokenType::Whitespace)
    }

    pub fn is_token_type(&self, token_type: TokenType) -> bool {
        self.token_type == token_type
    }
}

#[derive(Clone, Debug)]
pub struct TokenStream {
    tokens: Vec<Token>,
}

impl TokenStream {
    pub fn new() -> Self {
        TokenStream { tokens: Vec::new() }
    }

    pub fn add_token(&mut self, token: Token) {
        if !token.is_throwaway() {
            self.tokens.push(token);
        }
    }

    pub fn finalize(&mut self, last_line: usize) -> TokenStream {
        let eof_token = Token::eof(last_line);
        self.add_token(eof_token);
        self.clone()
    }
}

impl Default for TokenStream {
    fn default() -> Self {
        TokenStream::new()
    }
}

impl AsRef<[Token]> for TokenStream {
    fn as_ref(&self) -> &[Token] {
        &self.tokens
    }
}

impl Deref for TokenStream {
    type Target = Vec<Token>;

    fn deref(&self) -> &Self::Target {
        &self.tokens
    }
}

impl DerefMut for TokenStream {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.tokens
    }
}

impl IntoIterator for TokenStream {
    type Item = Token;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.tokens.into_iter()
    }
}

impl<'a> IntoIterator for &'a TokenStream {
    type Item = &'a Token;
    type IntoIter = std::slice::Iter<'a, Token>;

    fn into_iter(self) -> Self::IntoIter {
        self.tokens.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
