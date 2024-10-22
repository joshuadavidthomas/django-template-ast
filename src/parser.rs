use crate::ast::{Ast, AttributeValue, Node};
use crate::error::{NodeError, ParserError};
use crate::token::{Token, TokenStream, TokenType, TokenVecToString};

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
            TokenType::LeftAngle => {
                let next = self.peek_next()?;
                match next.token_type {
                    TokenType::Bang => self.html_doctype(),
                    TokenType::Text => self.html_tag(),
                    _ => Err(ParserError::UnexpectedToken(next.clone())),
                }
            }
            TokenType::LeftAngleBangDashDash => self.html_comment(),
            TokenType::LeftBracePercent => self.django_block(),
            TokenType::DoubleLeftBrace => self.django_variable(),
            TokenType::LeftBraceHash => self.django_comment(),
            TokenType::Text => self.text(),
            TokenType::Eof => Err(ParserError::AtEndOfStream),
            _ => Err(ParserError::UnexpectedToken(token.clone())),
        };
        self.advance()?;
        node
    }

    fn html_tag(&mut self) -> Result<Node, ParserError> {
        self.consume(TokenType::LeftAngle)?;
        let tag = self.consume(TokenType::Text)?.lexeme;
        let attributes = self.parse_attributes()?;

        let node = if self.peek()?.token_type == TokenType::SlashRightAngle {
            self.consume(TokenType::SlashRightAngle)?;
            Node::new_html_void_element(tag, Some(attributes))?
        } else {
            self.consume(TokenType::RightAngle)?;
            Node::new_html_element(tag, Some(attributes), None)?
        };
        Ok(node)
    }

    fn parse_attributes(&mut self) -> Result<Vec<(String, AttributeValue)>, ParserError> {
        let mut attributes = Vec::new();
        while self.peek()?.token_type == TokenType::Text {
            let name = self.consume(TokenType::Text)?.lexeme;
            let value = if self.peek()?.token_type == TokenType::Equal {
                self.consume(TokenType::Equal)?;
                AttributeValue::Value(self.parse_attribute_value()?)
            } else {
                AttributeValue::Boolean
            };
            attributes.push((name, value));
        }
        Ok(attributes)
    }

    fn parse_attribute_value(&mut self) -> Result<String, ParserError> {
        match self.peek()?.token_type {
            TokenType::SingleQuote | TokenType::DoubleQuote => {
                let quote = self.advance()?;
                let value = self.consume_until(quote.token_type)?;
                self.consume(quote.token_type)?;
                Ok(value.into_iter().map(|t| t.lexeme).collect::<String>())
            }
            _ => Ok(self.consume(TokenType::Text)?.lexeme),
        }
    }

    fn html_comment(&mut self) -> Result<Node, ParserError> {
        self.consume(TokenType::LeftAngleBangDashDash)?;
        let content = self.consume_until(TokenType::DashDashRightAngle)?;
        let node = Node::new_html_comment(content.to_string())?;
        self.consume(TokenType::DashDashRightAngle)?;
        Ok(node)
    }

    fn html_doctype(&mut self) -> Result<Node, ParserError> {
        self.consume(TokenType::LeftAngle)?;
        self.consume(TokenType::Bang)?;
        self.consume(TokenType::Text)?;
        let content = self.consume_until(TokenType::RightAngle)?;
        let node = Node::new_html_doctype(content.to_string())?;
        self.consume(TokenType::RightAngle)?;
        Ok(node)
    }

    fn django_block(&mut self) -> Result<Node, ParserError> {
        self.consume(TokenType::LeftBracePercent)?;
        let content = self.consume_until(TokenType::PercentRightBrace)?;
        self.consume(TokenType::PercentRightBrace)?;

        if content.is_empty() {
            return Err(ParserError::NodeError(NodeError::EmptyDjangoBlock));
        }

        let name = content[0].lexeme.clone();
        let arguments = if content.len() > 1 {
            Some(content[1..].iter().map(|token| token.to_string()).collect())
        } else {
            None
        };

        let node = Node::new_django_block(name, arguments, None)?;

        Ok(node)
    }

    fn django_variable(&mut self) -> Result<Node, ParserError> {
        self.consume(TokenType::DoubleLeftBrace)?;
        let content = self.consume_until(TokenType::DoubleRightBrace)?;
        let node = Node::new_django_variable(content.to_string())?;
        self.consume(TokenType::DoubleRightBrace)?;
        Ok(node)
    }

    fn django_comment(&mut self) -> Result<Node, ParserError> {
        self.consume(TokenType::LeftBraceHash)?;
        let content = self.consume_until(TokenType::HashRightBrace)?;
        let node = Node::new_django_comment(content.to_string())?;
        self.consume(TokenType::HashRightBrace)?;
        Ok(node)
    }

    fn text(&self) -> Result<Node, ParserError> {
        let token = self.peek()?;
        let node = Node::new_text(token.lexeme)?;
        Ok(node)
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

    fn peek_sequence(&self, count: usize) -> Result<Vec<Token>, ParserError> {
        (0..count).map(|i| self.peek_at(i as isize)).collect()
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

    fn advance(&mut self) -> Result<Token, ParserError> {
        if self.is_at_end() {
            return Err(ParserError::AtEndOfStream);
        }
        self.current += 1;
        self.peek_previous()
    }

    fn backtrack(&mut self, steps: usize) -> Result<Token, ParserError> {
        if self.current < steps {
            return Err(ParserError::AtBeginningOfStream);
        }
        self.current -= steps;
        self.peek_next()
    }

    fn lookahead(&self, types: &[TokenType]) -> Result<bool, ParserError> {
        for (i, &t) in types.iter().enumerate() {
            if self.peek_at(i as isize)?.token_type != t {
                return Ok(false);
            }
        }
        Ok(true)
    }

    fn consume(&mut self, token_type: TokenType) -> Result<Token, ParserError> {
        let token = self.peek()?;
        if token.token_type != token_type {
            return Err(ParserError::ExpectedTokenType(
                self.peek()?.clone(),
                token_type,
            ));
        }
        self.advance()?;
        Ok(token)
    }

    fn consume_until(&mut self, end_type: TokenType) -> Result<Vec<Token>, ParserError> {
        let mut consumed = Vec::new();
        while !self.is_at_end() && self.peek()?.token_type != end_type {
            let token = self.advance()?;
            consumed.push(token);
        }
        Ok(consumed)
    }

    fn synchronize(&mut self, sync_types: &[TokenType]) -> Result<(), ParserError> {
        while !self.is_at_end() {
            if sync_types.contains(&self.peek()?.token_type) {
                return Ok(());
            }
            self.advance()?;
        }
        Ok(())
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
    fn test_html_element() {
        let stream = create_token_stream(vec![
            (TokenType::LeftAngle, "<", 1),
            (TokenType::Text, "p", 1),
            (TokenType::RightAngle, ">", 1),
        ]);
        let mut parser = Parser::new(stream);

        let node = parser.html_tag().unwrap();

        assert_eq!(
            node,
            Node::HTMLElement {
                tag: "p".to_string(),
                attributes: vec![],
                children: vec![],
            }
        );
        assert_eq!(parser.peek().unwrap().token_type, TokenType::Eof);
    }

    #[test]
    fn test_html_void_element() {
        let stream = create_token_stream(vec![
            (TokenType::LeftAngle, "<", 1),
            (TokenType::Text, "img", 1),
            (TokenType::Text, "src", 1),
            (TokenType::Equal, "=", 1),
            (TokenType::DoubleQuote, "\"", 1),
            (TokenType::Text, "https://example.com/image.jpg", 1),
            (TokenType::DoubleQuote, "\"", 1),
            (TokenType::SlashRightAngle, "/>", 1),
        ]);
        let mut parser = Parser::new(stream);

        let node = parser.html_tag().unwrap();

        assert_eq!(
            node,
            Node::HTMLVoidElement {
                tag: "img".to_string(),
                attributes: vec![(
                    "src".to_string(),
                    AttributeValue::Value("https://example.com/image.jpg".to_string())
                )],
            }
        );
        assert_eq!(parser.peek().unwrap().token_type, TokenType::Eof);
    }

    #[test]
    fn test_html_doctype() {
        let stream = create_token_stream(vec![
            (TokenType::LeftAngle, "<", 1),
            (TokenType::Bang, "!", 1),
            (TokenType::Text, "DOCTYPE", 1),
            (TokenType::Text, "html", 1),
            (TokenType::RightAngle, ">", 1),
        ]);
        let mut parser = Parser::new(stream);

        let node = parser.html_doctype().unwrap();

        assert_eq!(
            node,
            Node::HTMLDoctype {
                doctype: "html".to_string()
            }
        );
        assert_eq!(parser.peek().unwrap().token_type, TokenType::Eof);
    }

    #[test]
    fn test_html_comment() {
        let stream = create_token_stream(vec![
            (TokenType::LeftAngleBangDashDash, "<!--", 1),
            (TokenType::Text, "Hello", 1),
            (TokenType::Text, "World", 1),
            (TokenType::DashDashRightAngle, "-->", 1),
        ]);
        let mut parser = Parser::new(stream);

        let node = parser.html_comment().unwrap();

        assert_eq!(node, Node::HTMLComment("Hello World".to_string()));
        assert_eq!(parser.peek().unwrap().token_type, TokenType::Eof);
    }

    #[test]
    fn test_django_block() {
        let stream = create_token_stream(vec![
            (TokenType::LeftBracePercent, "{%", 1),
            (TokenType::Text, "block", 1),
            (TokenType::Text, "hello='world'", 1),
            (TokenType::PercentRightBrace, "%}", 1),
        ]);
        let mut parser = Parser::new(stream);

        let node = parser.django_block().unwrap();

        assert_eq!(
            node,
            Node::DjangoBlock {
                name: "block".to_string(),
                arguments: vec!["hello='world'".to_string()],
                children: vec![],
            }
        );
        assert_eq!(parser.peek().unwrap().token_type, TokenType::Eof);
    }

    #[test]
    fn test_django_variable() {
        let stream = create_token_stream(vec![
            (TokenType::DoubleLeftBrace, "{{", 1),
            (TokenType::Text, "Hello", 1),
            (TokenType::Text, "World", 1),
            (TokenType::DoubleRightBrace, "}}", 1),
        ]);
        let mut parser = Parser::new(stream);

        let node = parser.django_variable().unwrap();

        assert_eq!(node, Node::DjangoVariable("Hello World".to_string()));
        assert_eq!(parser.peek().unwrap().token_type, TokenType::Eof);
    }

    #[test]
    fn test_django_comment() {
        let stream = create_token_stream(vec![
            (TokenType::LeftBraceHash, "{#", 1),
            (TokenType::Text, "Hello", 1),
            (TokenType::Text, "World", 1),
            (TokenType::HashRightBrace, "#}", 1),
        ]);
        let mut parser = Parser::new(stream);

        let node = parser.django_comment().unwrap();

        assert_eq!(node, Node::DjangoComment("Hello World".to_string()));
        assert_eq!(parser.peek().unwrap().token_type, TokenType::Eof);
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
