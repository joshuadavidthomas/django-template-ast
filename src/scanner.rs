use std::fmt::Debug;

pub trait Scanner {
    type Item: Debug;

    fn advance(&mut self) -> Self::Item;
    fn peek(&self) -> Self::Item;
    fn peek_next(&self) -> Self::Item;
    fn is_at_end(&self) -> bool;
}
