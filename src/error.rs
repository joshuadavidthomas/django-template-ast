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
}

impl ErrorMessage for LexerError {
    fn message(&self) -> &str {
        match self {
            LexerError::EmptyToken { message, .. }
            | LexerError::UnexpectedCharacter { message, .. }
            | LexerError::LexicalError { message, .. } => message,
        }
    }
}

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message())
    }
}

impl std::error::Error for LexerError {}
