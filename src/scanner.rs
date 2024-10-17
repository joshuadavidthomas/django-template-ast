use std::fmt::Debug;

use crate::error::ScannerError;

pub struct LexerState {
    start: usize,
    current: usize,
    line: usize,
    length: usize,
}

impl<'a> LexerState {
    pub fn new(source: &'a str) -> Self {
        LexerState {
            start: 0,
            current: 0,
            line: 1,
            length: source.len(),
        }
    }

    pub fn prepare(&mut self) {
        self.start = self.current;
    }

    pub fn advance(&mut self, chars: usize, lines: usize) {
        self.current += chars;
        self.line += lines;
    }

    pub fn next(&self) -> usize {
        self.current + 1
    }

    pub fn current(&self) -> usize {
        self.current
    }

    pub fn previous(&self) -> usize {
        self.current - 1
    }

    pub fn current_line(&self) -> usize {
        self.line
    }

    pub fn last_line(&self) -> Result<usize, ScannerError> {
        if self.is_at_end() {
            Ok(self.line)
        } else {
            Err(ScannerError::NotAtEnd)
        }
    }

    pub fn length(&self) -> usize {
        self.length
    }

    pub fn is_at_end(&self) -> bool {
        self.current >= self.length
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
