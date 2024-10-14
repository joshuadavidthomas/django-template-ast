use std::fmt;

#[derive(Debug)]
pub enum LexerError {
    EmptyToken(usize),
    UnexpectedCharacter(char, usize),
    LexicalError { message: String, position: usize },
}

impl LexerError {
    pub fn empty_token<T>(line: usize) -> Result<T, Self> {
        Err(LexerError::EmptyToken(line))
    }

    pub fn unexpected_character<T>(character: char, line: usize) -> Result<T, Self> {
        Err(LexerError::UnexpectedCharacter(character, line))
    }

    pub fn lexical_error<T>(message: &str, position: usize) -> Result<T, Self> {
        Err(LexerError::LexicalError {
            message: message.to_string(),
            position,
        })
    }
}

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LexerError::EmptyToken(line) => write!(f, "Empty token at line {}", line),
            LexerError::UnexpectedCharacter(c, line) => {
                write!(f, "Unexpected character '{}' at line {}", c, line)
            }
            LexerError::LexicalError { message, position } => {
                write!(f, "Lexical error at position {}: {}", position, message)
            }
        }
    }
}

impl std::error::Error for LexerError {}
