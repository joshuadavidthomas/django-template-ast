use std::fmt;
use std::fmt::Debug;
use std::ops::{Deref, DerefMut};
use std::string::ToString;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Block,
    Variable,
    Comment,

    TagOpen,
    TagClose,
    TagVoid,

    Text,
    Eof,
}

impl TokenType {
    pub fn delimiters(&self) -> (&'static str, &'static str) {
        match self {
            Self::Block => ("{%", "%}"),
            Self::Variable => ("{{", "}}"),
            Self::Comment => ("{#", "#}"),
            Self::TagOpen => ("<", ">"),
            Self::TagClose => ("</", ">"),
            Self::TagVoid => ("<", "/>"),
            _ => ("", ""),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: usize,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.lexeme)
    }
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
        self.tokens.push(token);
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
