use thiserror::Error;

#[derive(Error, Debug)]
pub enum LexerError {
    #[error("Empty token at line {line:?}")]
    EmptyToken { line: usize },
    #[error("Unexpected character '{character}' at line {line}")]
    UnexpectedCharacter { character: char, line: usize },
    #[error("Source is empty")]
    EmptySource,
    #[error("At beginning of source")]
    AtBeginningOfSource,
    #[error("At end of source")]
    AtEndOfSource,
    #[error("Invalid character access")]
    InvalidCharacterAccess,
}
