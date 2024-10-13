use std::fmt::Debug;

pub struct ScannerState {
    pub start: usize,
    pub current: usize,
    pub line: usize,
}

impl ScannerState {
    pub fn new() -> Self {
        ScannerState {
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
    fn is_at_end(&self) -> bool;
}
