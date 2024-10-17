use thiserror::Error;

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
    #[error(transparent)] // Display the inner TokenError directly
    TokenError(#[from] TokenError), // This automatically implements From<TokenError>
}

#[derive(Error, Debug)]
pub enum TokenError {
    #[error("unexpected character '{character}'")]
    UnexpectedCharacter { character: char },
    #[error("string did not match a token")]
    NoTokenMatch,
    #[error("unexpected end of input, expected string literal")]
    UnexpectedEndOfInput,
}

#[derive(Error, Debug)]
pub enum NodeError {
    #[error("Tag name cannot be empty")]
    NoTagName,
    #[error("Block name cannot be empty")]
    NoBlockName,
}

#[derive(Error, Debug)]
pub enum ParserError {
    #[error("Token stream is empty")]
    EmptyTokenStream,
    #[error("At beginning of token stream")]
    AtBeginningOfStream,
    #[error("At end of token stream")]
    AtEndOfStream,
    #[error("Invalid token access")]
    InvalidTokenAccess,
    #[error("AST error: {0}")]
    ASTError(#[from] ASTError),
}

#[derive(Error, Debug)]
pub enum ASTError {}
