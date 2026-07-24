//! Hutool-aligned iterator adapters with Rust-native ownership semantics.

use std::collections::VecDeque;

/// An iterator that transforms values lazily.
pub struct TransIter<I, F>
where
    I: Iterator,
{
    source: std::iter::Peekable<I>,
    transform: F,
}

impl<I, F> TransIter<I, F>
where
    I: Iterator,
{
    /// Creates a transforming iterator.
    #[must_use]
    pub fn new(source: I, transform: F) -> Self {
        Self {
            source: source.peekable(),
            transform,
        }
    }

    /// Reports whether another source value remains.
    pub fn has_next(&mut self) -> bool {
        self.source.peek().is_some()
    }
}

impl<I, F, T> Iterator for TransIter<I, F>
where
    I: Iterator,
    F: FnMut(I::Item) -> T,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.source.next().map(&mut self.transform)
    }
}
