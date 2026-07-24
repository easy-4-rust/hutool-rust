//! Hutool-aligned iterator adapters with Rust-native ownership semantics.

use std::collections::VecDeque;

/// An owned snapshot of another iterator.
#[derive(Debug)]
pub struct CopiedIter<T> {
    snapshot: std::iter::Peekable<std::vec::IntoIter<T>>,
}

impl<T> CopiedIter<T> {
    /// Consumes a source and snapshots all remaining values.
    #[must_use]
    pub fn new(source: impl IntoIterator<Item = T>) -> Self {
        Self {
            snapshot: source
                .into_iter()
                .collect::<Vec<_>>()
                .into_iter()
                .peekable(),
        }
    }

    /// Named constructor aligned with Hutool's `copyOf` factory.
    #[must_use]
    pub fn copy_of(source: impl IntoIterator<Item = T>) -> Self {
        Self::new(source)
    }

    /// Reports whether another snapshot value remains.
    pub fn has_next(&mut self) -> bool {
        self.snapshot.peek().is_some()
    }

    /// Returns the remaining snapshot length.
    #[must_use]
    pub fn len(&self) -> usize {
        self.snapshot.len()
    }

    /// Returns whether the snapshot is exhausted.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.snapshot.len() == 0
    }
}

impl<T> Iterator for CopiedIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.snapshot.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.snapshot.size_hint()
    }
}

impl<T> ExactSizeIterator for CopiedIter<T> {}
