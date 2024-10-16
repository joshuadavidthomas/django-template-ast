mod ast;
mod error;
mod lexer;
mod parser;
mod scanner;
mod token;

use lexer::Lexer;
use parser::Parser;
use std::error::Error;

pub fn compile(template: &str) -> Result<String, Box<dyn Error>> {
    let mut lexer = Lexer::new(template);
    let tokens = lexer.tokenize()?;
    let ast = Parser::new(tokens.clone()).parse()?;
    println!("{:?}", tokens);
    println!("{:?}", ast);
    todo!("Implement compilation process")
}

// re-export important types or functions from modules here, e.g.
// pub use lexer::Lexer;
// that should be part of the public API
