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
        let node = Node::new_text(token.lexeme)?;
        Ok(node)
    }

    fn advance(&mut self) -> Result<(), ParserError> {
        if self.is_at_end() {
            return Err(ParserError::AtEndOfStream);
        }
        self.current += 1;
        Ok(())
    }

    fn peek(&self) -> Result<Token, ParserError> {
        self.peek_at(0)
    }

    fn peek_next(&self) -> Result<Token, ParserError> {
        self.peek_at(1)
    }

    fn peek_previous(&self) -> Result<Token, ParserError> {
        self.peek_at(-1)
    }

    fn peek_at(&self, offset: isize) -> Result<Token, ParserError> {
        let index = self.current as isize + offset;
        self.item_at(index as usize)
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
        self.current + 1 >= self.tokens.len()
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
            stream.add_token(token);
            last_line = line;
        }
        stream.add_token(Token::new(TokenType::Eof, "", last_line.max(1) + 1));
        stream
    }

    #[test]
    fn test_next_node() {
        let stream = create_token_stream(vec![(TokenType::Text, "Hello", 1)]);
        let mut parser = Parser::new(stream);

        let node = parser.next_node().unwrap();

        assert_eq!(node, Node::Text("Hello".to_string()));
    }

    #[test]
    fn test_advance() {
        let stream = create_token_stream(vec![
            (TokenType::Text, "Hello", 1),
            (TokenType::Text, "World", 2),
        ]);
        let mut parser = Parser::new(stream);

        assert_eq!(parser.current, 0);

        parser.advance().unwrap();
        assert_eq!(parser.current, 1);

        parser.advance().unwrap();
        assert_eq!(parser.current, 2);

        let result = parser.advance();
        assert!(result.is_err());
        assert_eq!(parser.current, 2);
    }

    #[test]
    fn test_peek() {
        let stream = create_token_stream(vec![
            (TokenType::Text, "Hello", 1),
            (TokenType::Text, "World", 2),
        ]);
        let mut parser = Parser::new(stream);

        let test_cases = vec![(0, "Hello"), (1, "World"), (2, "")];

        for (current, expected) in test_cases {
            parser.current = current;
            assert_eq!(
                parser.peek().unwrap().lexeme,
                expected,
                "peek() failed for current: {}",
                current
            );
        }
    }

    #[test]
    fn test_peek_next() {
        let stream = create_token_stream(vec![
            (TokenType::Text, "Hello", 1),
            (TokenType::Text, "World", 2),
        ]);
        let mut parser = Parser::new(stream);

        let test_cases = vec![(0, "World"), (1, "")];

        for (current, expected) in test_cases {
            parser.current = current;
            assert_eq!(
                parser.peek_next().unwrap().lexeme,
                expected,
                "peek_next() failed for current: {}",
                current
            );
        }
    }

    #[test]
    fn test_peek_previous() {
        let stream = create_token_stream(vec![
            (TokenType::Text, "Hello", 1),
            (TokenType::Text, "World", 2),
        ]);
        let mut parser = Parser::new(stream);

        let test_cases = vec![(2, "World"), (1, "Hello")];

        for (current, expected) in test_cases {
            parser.current = current;
            assert_eq!(
                parser.peek_previous().unwrap().lexeme,
                expected,
                "peek_previous() failed for current: {}",
                current
            );
        }
    }

    #[test]
    fn test_peek_at() {
        let stream = create_token_stream(vec![
            (TokenType::Text, "Hello", 1),
            (TokenType::Text, "World", 2),
        ]);
        let mut parser = Parser::new(stream);

        let test_cases = vec![(1, -1, "Hello"), (1, 0, "World"), (1, 1, "")];

        for (current, offset, expected) in test_cases {
            parser.current = current;
            assert_eq!(
                parser.peek_at(offset).unwrap().lexeme,
                expected,
                "peek_at() failed for current: {}, offset: {}",
                current,
                offset
            );
        }
    }

    #[test]
    fn test_item_at() {
        let stream = create_token_stream(vec![
            (TokenType::Text, "Hello", 1),
            (TokenType::Text, "World", 2),
        ]);
        let parser = Parser::new(stream);

        let test_cases = vec![(0, "Hello"), (1, "World"), (2, "")];

        for (index, expected) in test_cases {
            assert_eq!(
                parser.item_at(index).unwrap().lexeme,
                expected,
                "peek_at() failed for index: {}",
                index,
            );
        }
    }

    #[test]
    fn test_is_at_end() {
        let stream = create_token_stream(vec![
            (TokenType::Text, "Hello", 1),
            (TokenType::Text, "World", 2),
        ]);
        let mut parser = Parser::new(stream);

        let test_cases = vec![
            (0, false),
            (1, false),
            (2, true),
            (3, true),
            (usize::MAX - 1, true),
        ];

        for (current, expected) in test_cases {
            parser.current = current;
            assert_eq!(
                parser.is_at_end(),
                expected,
                "item_at() failed for current: {}",
                current,
            );
        }
    }
}
