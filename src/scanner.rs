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

    fn advance(&mut self) -> Self::Item;
    fn peek(&self) -> Self::Item;
    fn peek_next(&self) -> Self::Item;
    fn previous(&self) -> Self::Item;
    fn is_at_end(&self) -> bool;
}
