use thiserror::Error;

#[derive(Error, Debug)]
pub enum LexerError {
    #[error("Empty token at line {line:?}")]
    EmptyToken { line: usize },
    #[error("Unexpected character '{character}' at line {line}")]
    UnexpectedCharacter { character: char, line: usize },
    #[error("At beginning of input")]
    AtBeginningOfInput,
    #[error("At end of input")]
    AtEndOfInput,
    #[error("Invalid character access")]
    InvalidCharacterAccess,

}
