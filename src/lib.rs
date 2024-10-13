mod error;
mod lexer;
mod scanner;
mod token;

use std::error::Error;

pub fn compile(_template: &str) -> Result<String, Box<dyn Error>> {
    todo!("Implement compilation process")
}

// re-export important types or functions from modules here, e.g.
// pub use lexer::Lexer;
// that should be part of the public API
