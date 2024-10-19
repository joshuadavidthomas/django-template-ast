use std::fmt::Debug;

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
