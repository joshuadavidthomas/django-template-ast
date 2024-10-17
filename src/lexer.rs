use crate::error::LexerError;
use crate::scanner::{LexerState, Scanner};
use crate::token::{Token, TokenType};

pub struct Lexer<'a> {
    source: &'a str,
    tokens: Vec<Token<'a>>,
    state: LexerState,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Lexer {
            source,
            tokens: Vec::new(),
            state: LexerState::new(),
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, LexerError> {
        while !self.is_at_end() {
            self.state.start = self.state.current;
            let (token, size, lines_consumed) = self.next_token()?;
            self.add_token(token);

            self.state.current += size;
            self.state.line += lines_consumed;
        }

        self.add_token(Token::new(TokenType::Eof, "", self.state.line));
        Ok(self.tokens.clone())
    }

    fn next_token(&mut self) -> Result<(Token<'a>, usize, usize), LexerError> {
        self.advance()?;
        let remaining_source = &self.source[self.state.current..];

        let (token, size, lines_traversed) = Token::from_input(remaining_source, self.state.line)?;

        Ok((token, size, lines_traversed))
    }

    fn add_token(&mut self, token: Token<'a>) {
        if token.token_type != TokenType::Whitespace {
            self.tokens.push(token);
        }
    }
}

impl<'a> Scanner for Lexer<'a> {
    type Item = char;
    type Error = LexerError;

    fn advance(&mut self) -> Result<Self::Item, Self::Error> {
        let current_char = self.peek()?;
        self.state.current += 1;
        Ok(current_char)
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
        if let Some(ch) = self.source.chars().nth(index) {
            Ok(ch)
        } else {
            let error = if self.source.is_empty() {
                LexerError::EmptySource
            } else if index < self.state.current {
                LexerError::AtBeginningOfSource
            } else if index >= self.source.len() {
                LexerError::AtEndOfSource
            } else {
                LexerError::InvalidCharacterAccess
            };
            Err(error)
        }
    }

    fn is_at_end(&self) -> bool {
        self.state.current >= self.source.len()
    }
}
