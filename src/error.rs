use std::fmt;

pub trait ErrorMessage {
    fn message(&self) -> &str;
}

#[derive(Debug)]
pub enum LexerError {
    EmptyToken {
        line: usize,
        message: String,
    },
    UnexpectedCharacter {
        character: char,
        line: usize,
        message: String,
    },
    LexicalError {
        position: usize,
        message: String,
    },
}

impl LexerError {
    pub fn empty_token<T>(line: usize) -> Result<T, Self> {
        Err(LexerError::EmptyToken {
            line,
            message: format!("Empty token at line {}", line),
        })
    }

    pub fn unexpected_character<T>(character: char, line: usize) -> Result<T, Self> {
        Err(LexerError::UnexpectedCharacter {
            character,
            line,
            message: format!("Unexpected character '{}' at line {}", character, line),
        })
    }

    pub fn lexical_error<T>(message: &str, position: usize) -> Result<T, Self> {
        Err(LexerError::LexicalError {
            position,
            message: format!("Lexical error at position {}: {}", position, message),
        })
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
