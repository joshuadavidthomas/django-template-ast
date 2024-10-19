use crate::error::LexerError;
use crate::scanner::Scanner;
use crate::token::{Token, TokenStream};

pub struct Lexer {
    source: String,
    tokens: TokenStream,
    start: usize,
    current: usize,
    line: usize,
}

impl Lexer {
    pub fn new(source: &str) -> Self {
        Lexer {
            source: String::from(source),
            tokens: TokenStream::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn tokenize(&mut self) -> Result<TokenStream, LexerError> {
        while !self.is_at_end() {
            let token = self.next_token()?;
            self.add_token(token);
        }
        let tokens = self.tokens.finalize(self.line);
        Ok(tokens)
    }

    fn next_token(&mut self) -> Result<Token, LexerError> {
        self.advance()?;
        let remaining_source = &self.source[self.current..];
        let token = Token::from_source(remaining_source, self.line)?;
        self.current += token.size();
        self.line += token.lines();
        Ok(token)
    }

    fn add_token(&mut self, token: Token) {
        if !token.is_throwaway() {
            self.tokens.add_token(token);
        }
    }
}

impl Scanner for Lexer {
    type Item = char;
    type Error = LexerError;

    fn advance(&mut self) -> Result<Self::Item, Self::Error> {
        self.start = self.current;
        let current_char = self.peek()?;
        self.current += 1;
        Ok(current_char)
    }

    fn peek(&self) -> Result<Self::Item, Self::Error> {
        self.item_at(self.current)
    }

    fn peek_next(&self) -> Result<Self::Item, Self::Error> {
        self.item_at(self.current + 1)
    }

    fn peek_previous(&self) -> Result<Self::Item, Self::Error> {
        self.item_at(self.current - 1)
    }

    fn item_at(&self, index: usize) -> Result<Self::Item, Self::Error> {
        if let Some(ch) = self.source.chars().nth(index) {
            Ok(ch)
        } else {
            let error = if self.source.is_empty() {
                LexerError::EmptySource
            } else if index < self.current {
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
        self.current >= self.source.len()
    }
}
