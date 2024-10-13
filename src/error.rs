use std::fmt;

#[derive(Debug)]
pub enum LexerError {
    EmptyToken(usize),
    UnexpectedCharacter(char, usize),
}

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LexerError::EmptyToken(line) => write!(f, "Empty token at line {}", line),
            LexerError::UnexpectedCharacter(c, line) => {
                write!(f, "Unexpected character '{}' at line {}", c, line)
            }
        }
    }
}

impl std::error::Error for LexerError {}
