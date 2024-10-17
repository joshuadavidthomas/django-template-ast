use crate::error::LexerError;
use crate::scanner::{LexerState, Scanner};
use crate::token::{Token, TokenStream};

pub struct Lexer<'a> {
    source: &'a str,
    tokens: TokenStream<'a>,
    state: LexerState,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Lexer {
            source,
            tokens: TokenStream::new(),
            state: LexerState::new(source),
        }
    }

    pub fn tokenize(&mut self) -> Result<TokenStream, LexerError> {
        while !self.is_at_end() {
            let token = self.next_token()?;
            self.add_token(token);
        }
        let tokens = self.tokens.finalize(self.state.last_line()?);
        Ok(tokens)
    }

    fn next_token(&mut self) -> Result<Token<'a>, LexerError> {
        self.advance()?;
        let remaining_source = &self.source[self.state.current()..];
        let token = Token::from_source(remaining_source, self.state.current_line())?;
        self.state.advance(token.size(), token.lines());
        Ok(token)
    }

    fn add_token(&mut self, token: Token<'a>) {
        if !token.is_throwaway() {
            self.tokens.add_token(token);
        }
    }
}

impl<'a> Scanner for Lexer<'a> {
    type Item = char;
    type Error = LexerError;

    fn advance(&mut self) -> Result<Self::Item, Self::Error> {
        self.state.prepare();
        let current_char = self.peek()?;
        self.state.advance(1, 0);
        Ok(current_char)
    }

    fn peek(&self) -> Result<Self::Item, Self::Error> {
        self.item_at(self.state.current())
    }

    fn peek_next(&self) -> Result<Self::Item, Self::Error> {
        self.item_at(self.state.next())
    }

    fn peek_previous(&self) -> Result<Self::Item, Self::Error> {
        self.item_at(self.state.previous())
    }

    fn item_at(&self, index: usize) -> Result<Self::Item, Self::Error> {
        if let Some(ch) = self.source.chars().nth(index) {
            Ok(ch)
        } else {
            let error = if self.source.is_empty() {
                LexerError::EmptySource
            } else if index < self.state.current() {
                LexerError::AtBeginningOfSource
            } else if index >= self.state.length() {
                LexerError::AtEndOfSource
            } else {
                LexerError::InvalidCharacterAccess
            };
            Err(error)
        }
    }

    fn is_at_end(&self) -> bool {
        self.state.is_at_end()
    }
}
