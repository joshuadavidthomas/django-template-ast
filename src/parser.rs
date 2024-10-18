use crate::ast::AST;
use crate::error::ParserError;
use crate::scanner::{ParserState, Scanner};
use crate::token::{Token, TokenType};

pub struct Parser<'a> {
    tokens: Vec<Token<'a>>,
    state: ParserState,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: Vec<Token<'a>>) -> Self {
        Parser {
            tokens,
            state: ParserState::new(),
        }
    }

    pub fn parse(&mut self) -> Result<AST, ParserError> {
        let ast = AST::new();
        while !self.is_at_end() {
            let token = self.advance()?;
            let node = ast.match_node(token)?;
            println!("{:?}", node);
            // ast.add_node(node);
        }
        Ok(ast)
    }
}

impl<'a> Scanner for Parser<'a> {
    type Item = Token<'a>;
    type Error = ParserError;

    fn advance(&mut self) -> Result<Self::Item, Self::Error> {
        let current_token = self.peek()?;
        if !self.is_at_end() {
            self.state.current += 1;
        }
        Ok(current_token)
    }

    fn peek(&self) -> Result<Self::Item, Self::Error> {
        self.item_at(self.state.current)
    }

    fn peek_next(&self) -> Result<Self::Item, Self::Error> {
        self.item_at(self.state.current + 1)
    }

    fn peek_previous(&self) -> Result<Self::Item, Self::Error> {
        self.item_at(self.state.current - 1)
    }

    fn item_at(&self, index: usize) -> Result<Self::Item, Self::Error> {
        if let Some(token) = self.tokens.get(index) {
            Ok(token.clone())
        } else {
            let error = if self.tokens.is_empty() {
                ParserError::EmptyTokenStream
            } else if index < self.state.current {
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
        self.peek()
            .map(|token| token.token_type == TokenType::Eof)
            .unwrap_or(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser_new() {
        let tokens = vec![
            Token::new(TokenType::Text, "Text", 1),
            Token::new(TokenType::Text, "More Text", 2),
            Token::new(TokenType::Eof, "", 3),
        ];
        let parser = Parser::new(tokens);

        assert_eq!(parser.tokens.len(), 3);
        assert_eq!(parser.state.current, 0);
    }

    #[test]
    fn test_parser_parse() {
        let tokens = vec![
            Token::new(TokenType::Text, "Text", 1),
            Token::new(TokenType::Text, "More Text", 2),
            Token::new(TokenType::Eof, "", 3),
        ];
        let mut parser = Parser::new(tokens);

        let ast = parser.parse().unwrap();

        assert_eq!(parser.state.current, 2);
        assert_eq!(ast.nodes, vec![]);
    }

    #[test]
    fn test_scanner_advance() {
        let tokens = vec![
            Token::new(TokenType::Text, "Text", 1),
            Token::new(TokenType::Text, "More Text", 2),
            Token::new(TokenType::Eof, "", 3),
        ];
        let mut parser = Parser::new(tokens.clone());
        assert_eq!(parser.state.current, 0);

        let token = parser.advance().unwrap();
        assert_eq!(parser.state.current, 1);
        assert_eq!(token, tokens[0]);

        let token = parser.advance().unwrap();
        assert_eq!(parser.state.current, 2);
        assert_eq!(token, tokens[1]);

        let eof_token = parser.advance().unwrap();
        assert_eq!(parser.state.current, 2);
        assert_eq!(eof_token, tokens[2]);
    }

    #[test]
    fn test_scanner_advance_is_at_end() {
        let tokens = vec![
            Token::new(TokenType::Text, "Text", 1),
            Token::new(TokenType::Text, "More Text", 2),
            Token::new(TokenType::Eof, "", 3),
        ];
        let mut parser = Parser::new(tokens.clone());

        assert_eq!(parser.state.current, 0);

        parser.state.current += 2;

        let token = parser.tokens.get(parser.state.current).unwrap().clone();
        assert_eq!(token, tokens[2]);

        // TODO: should this error?
        let next_token = parser.advance().unwrap();
        assert_eq!(next_token, token);
    }

    #[test]
    fn test_scanner_peek() {
        let tokens = vec![
            Token::new(TokenType::Text, "Text", 1),
            Token::new(TokenType::Text, "More Text", 2),
            Token::new(TokenType::Eof, "", 3),
        ];
        let mut parser = Parser::new(tokens.clone());

        let token = parser.peek().unwrap();
        assert_eq!(token, tokens[0]);

        parser.state.current += 2;

        let token = parser.peek().unwrap();
        assert_eq!(token, tokens[2]);
    }

    #[test]
    fn test_scanner_peek_next() {
        let tokens = vec![
            Token::new(TokenType::Text, "Text", 1),
            Token::new(TokenType::Text, "More Text", 2),
            Token::new(TokenType::Eof, "", 3),
        ];
        let mut parser = Parser::new(tokens.clone());

        let current_token = parser.peek().unwrap();
        assert_eq!(current_token, tokens[0]);

        let next_token = parser.peek_next().unwrap();
        assert_eq!(next_token, tokens[1]);

        parser.state.current += 1;

        let current_token = parser.peek().unwrap();
        assert_eq!(current_token, tokens[1]);

        let next_token = parser.peek_next().unwrap();
        assert_eq!(next_token, tokens[2]);
    }

    #[test]
    fn test_scanner_peek_previous() {
        let tokens = vec![
            Token::new(TokenType::Text, "Text", 1),
            Token::new(TokenType::Text, "More Text", 2),
            Token::new(TokenType::Eof, "", 3),
        ];
        let mut parser = Parser::new(tokens.clone());

        parser.state.current = 2;

        let current_token = parser.peek().unwrap();
        assert_eq!(current_token, tokens[2]);

        let previous_token = parser.peek_previous().unwrap();
        assert_eq!(previous_token, tokens[1]);

        parser.state.current -= 1;

        let current_token = parser.peek().unwrap();
        assert_eq!(current_token, tokens[1]);

        let previous_token = parser.peek_previous().unwrap();
        assert_eq!(previous_token, tokens[0]);
    }

    #[test]
    #[should_panic]
    fn test_scanner_peek_previous_at_beginning() {
        let tokens = vec![
            Token::new(TokenType::Text, "Text", 1),
            Token::new(TokenType::Text, "More Text", 2),
            Token::new(TokenType::Eof, "", 3),
        ];
        let parser = Parser::new(tokens.clone());
        assert_eq!(parser.state.current, 0);

        parser.peek_previous().unwrap();
    }

    #[test]
    fn test_scanner_item_at() {
        let tokens = vec![
            Token::new(TokenType::Text, "Text", 1),
            Token::new(TokenType::Text, "More Text", 2),
            Token::new(TokenType::Eof, "", 3),
        ];
        let parser = Parser::new(tokens.clone());

        assert_eq!(parser.item_at(0).unwrap(), tokens[0]);
        assert_eq!(parser.item_at(1).unwrap(), tokens[1]);
        assert_eq!(parser.item_at(2).unwrap(), tokens[2]);
    }

    #[test]
    #[should_panic]
    fn test_scanner_item_at_empty() {
        let tokens = vec![];
        let parser = Parser::new(tokens.clone());

        assert_eq!(parser.item_at(0).unwrap(), tokens[0]);
    }

    #[test]
    #[should_panic]
    fn test_scanner_item_at_end_of_input() {
        let tokens = vec![
            Token::new(TokenType::Text, "Text", 1),
            Token::new(TokenType::Text, "More Text", 2),
            Token::new(TokenType::Eof, "", 3),
        ];
        let parser = Parser::new(tokens.clone());

        assert_eq!(parser.item_at(3).unwrap(), tokens[0]);
    }

    #[test]
    fn test_scanner_is_at_end() {
        let tokens = vec![
            Token::new(TokenType::Text, "Text", 1),
            Token::new(TokenType::Text, "More Text", 2),
            Token::new(TokenType::Eof, "", 3),
        ];
        let mut parser = Parser::new(tokens.clone());

        assert!(!parser.is_at_end());

        parser.state.current = 2;

        assert!(parser.is_at_end());
    }
}
