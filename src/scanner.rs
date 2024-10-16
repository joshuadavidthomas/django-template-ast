use std::fmt::Debug;

pub struct LexerState {
    pub start: usize,
    pub current: usize,
    pub line: usize,
}

impl LexerState {
    pub fn new() -> Self {
        LexerState {
            start: 0,
            current: 0,
            line: 1,
        }
    }
}

pub trait Scanner {
    type Item: Debug;
    type Error: Debug + std::error::Error;

    fn advance(&mut self) -> Result<Self::Item, Self::Error>;
    fn peek(&self) -> Result<Self::Item, Self::Error>;
    fn peek_next(&self) -> Result<Self::Item, Self::Error>;
    fn peek_previous(&self) -> Result<Self::Item, Self::Error>;
    fn item_at(&self, index: usize) -> Result<Self::Item, Self::Error>;
    fn is_at_end(&self) -> bool;
}
