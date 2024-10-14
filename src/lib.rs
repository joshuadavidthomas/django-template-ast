mod error;
mod lexer;
mod scanner;
mod token;

use lexer::Lexer;
use std::error::Error;
use token::Tokenizer;

pub fn compile(template: &str) -> Result<String, Box<dyn Error>> {
    let tokens = Lexer::new(template).tokenize()?;
    println!("{:?}", tokens);
    todo!("Implement compilation process")
}

// re-export important types or functions from modules here, e.g.
// pub use lexer::Lexer;
// that should be part of the public API
