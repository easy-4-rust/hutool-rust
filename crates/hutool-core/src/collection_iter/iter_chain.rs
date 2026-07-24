//! Hutool-aligned iterator adapters with Rust-native ownership semantics.

use std::collections::VecDeque;

/// A dynamically extensible chain of iterators.
pub struct IterChain<'a, T> {
    iterators: VecDeque<Box<dyn Iterator<Item = T> + 'a>>,
    next_item: Option<T>,
}

impl<'a, T> IterChain<'a, T> {
    /// Creates an empty chain.
    #[must_use]
    pub fn new() -> Self {
        Self {
            iterators: VecDeque::new(),
            next_item: None,
        }
    }

    /// Creates a chain from boxed iterators.
    #[must_use]
    pub fn with_iterators(
        iterators: impl IntoIterator<Item = Box<dyn Iterator<Item = T> + 'a>>,
    ) -> Self {
        Self {
            iterators: iterators.into_iter().collect(),
            next_item: None,
        }
    }

    /// Appends an iterator and returns the chain for fluent construction.
    pub fn add_chain(&mut self, iterator: impl Iterator<Item = T> + 'a) -> &mut Self {
        // Rust ownership prevents adding the same iterator object twice.
        self.iterators.push_back(Box::new(iterator));
        self
    }

    /// Returns the number of source iterators not yet fully discarded.
    #[must_use]
    pub fn chain_count(&self) -> usize {
        self.iterators.len()
    }

    /// Reports whether another value remains without consuming it.
    pub fn has_next(&mut self) -> bool {
        self.next_item.is_some() || self.fill_next()
    }

    fn fill_next(&mut self) -> bool {
        while let Some(iterator) = self.iterators.front_mut() {
            if let Some(item) = iterator.next() {
                self.next_item = Some(item);
                return true;
            }
            self.iterators.pop_front();
        }
        false
    }
}

impl<T> Default for IterChain<'_, T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Iterator for IterChain<'_, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next_item.is_none() {
            self.fill_next();
        }
        self.next_item.take()
    }
}
