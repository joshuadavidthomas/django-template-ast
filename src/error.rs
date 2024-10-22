use crate::token::{Token, TokenType};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AstError {}

#[derive(Error, Debug)]
pub enum LexerError {
    #[error("empty token at line {line:?}")]
    EmptyToken { line: usize },
    #[error("unexpected character '{character}' at line {line}")]
    UnexpectedCharacter { character: char, line: usize },
    #[error("source is empty")]
    EmptySource,
    #[error("at beginning of source")]
    AtBeginningOfSource,
    #[error("at end of source")]
    AtEndOfSource,
    #[error("invalid character access")]
    InvalidCharacterAccess,
    #[error("unexpected token type '{0:?}'")]
    UnexpectedTokenType(TokenType),
    #[error(transparent)]
    TokenError(#[from] TokenError),
}

#[derive(Error, Debug)]
pub enum NodeError {
    #[error("tag name cannot be empty")]
    NoTagName,
    #[error("django block cannot be empty")]
    EmptyDjangoBlock,
}

#[derive(Error, Debug)]
pub enum ParserError {
    #[error("token stream is empty")]
    EmptyTokenStream,
    #[error("at beginning of token stream")]
    AtBeginningOfStream,
    #[error("at end of token stream")]
    AtEndOfStream,
    #[error("invalid token access")]
    InvalidTokenAccess,
    #[error("unexpected token '{0:?}', expected type '{1:?}'")]
    ExpectedTokenType(Token, TokenType),
    #[error("unexpected token '{0:?}'")]
    UnexpectedToken(Token),
    #[error(transparent)]
    AstError(#[from] AstError),
    #[error(transparent)]
    NodeError(#[from] NodeError),
}

#[derive(Error, Debug)]
pub enum TokenError {
    #[error("unexpected character '{character}'")]
    UnexpectedCharacter { character: char },
    #[error("string did not match a token")]
    NoTokenMatch,
    #[error("unexpected end of input, expected string literal")]
    UnexpectedEndOfInput,
    #[error("cannot call size, token type has dynamic size")]
    DynamicTokenSize,
}
