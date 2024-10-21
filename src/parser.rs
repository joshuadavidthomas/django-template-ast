use crate::ast::{Ast, Node};
use crate::error::ParserError;
use crate::token::{Token, TokenStream, TokenType};

pub struct Parser {
    tokens: TokenStream,
    current: usize,
}

impl Parser {
    pub fn new(tokens: TokenStream) -> Self {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Ast, ParserError> {
        let mut ast = Ast::new();
        while !self.is_at_end() {
            let node = self.next_node()?;
            ast.add_node(node);
        }
        Ok(ast)
    }

    fn next_node(&mut self) -> Result<Node, ParserError> {
        let token = self.peek()?;
        let node = match token.token_type {
            TokenType::Text => self.text(),
            TokenType::Eof => Err(ParserError::AtEndOfStream),
            _ => Err(ParserError::UnexpectedToken(token.clone())),
        };
        self.advance()?;
        node
    }

    fn text(&self) -> Result<Node, ParserError> {
        let token = self.peek()?;
        Ok(Node::new_text(token.lexeme)?)
    }

    fn advance(&mut self) -> Result<Token, ParserError> {
        if self.is_at_end() {
            return Err(ParserError::AtEndOfStream);
        }
        self.current += 1;
        self.peek()
    }

    fn peek(&self) -> Result<Token, ParserError> {
        self.item_at(self.current)
    }

    fn peek_next(&self) -> Result<Token, ParserError> {
        self.item_at(self.current + 1)
    }

    fn peek_previous(&self) -> Result<Token, ParserError> {
        self.item_at(self.current - 1)
    }

    fn item_at(&self, index: usize) -> Result<Token, ParserError> {
        if let Some(token) = self.tokens.get(index) {
            Ok(token.clone())
        } else {
            let error = if self.tokens.is_empty() {
                ParserError::EmptyTokenStream
            } else if index < self.current {
                ParserError::AtBeginningOfStream
            } else if index >= self.tokens.len() {
                ParserError::AtEndOfStream
            } else {
                ParserError::InvalidTokenAccess
            };
            Err(error)
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_token_stream(tokens: Vec<(TokenType, &str, usize)>) -> TokenStream {
        let mut stream = TokenStream::new();
        let mut last_line = 0;

        for (token_type, lexeme, line) in tokens {
            let token = Token::new(token_type, lexeme, line);
            println!("Created token: {:?}", token); // Debug print
            stream.add_token(token);
            last_line = line;
        }

        let eof_token = Token::new(TokenType::Eof, "", last_line.max(1) + 1);
        println!("Created EOF token: {:?}", eof_token); // Debug print
        stream.add_token(eof_token);

        stream
    }

    #[test]
    fn test_next_node() {
        let tokens = vec![(TokenType::Text, "Hello", 1)];
        let stream = create_token_stream(tokens);
        println!("Created stream: {:?}", stream); // Debug print

        let mut parser = Parser::new(stream);

        let node = parser.next_node().unwrap();
        println!("Parsed node: {:?}", node); // Debug print

        assert_eq!(node, Node::Text("Hello".to_string()));
    }
}
