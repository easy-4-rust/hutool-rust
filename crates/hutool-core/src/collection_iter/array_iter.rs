//! Hutool-aligned iterator adapters with Rust-native ownership semantics.

use std::collections::VecDeque;

use super::resettable_iter::ResettableIter;

/// An iterator over a borrowed slice with Hutool-compatible range handling.
#[derive(Clone, Copy, Debug)]
pub struct ArrayIter<'a, T> {
    items: &'a [T],
    start_index: usize,
    end_index: usize,
    index: usize,
}

impl<'a, T> ArrayIter<'a, T> {
    /// Iterates the complete slice.
    #[must_use]
    pub fn new(items: &'a [T]) -> Self {
        Self::with_bounds(items, 0, -1)
    }

    /// Iterates from `start_index` to the end of the slice.
    #[must_use]
    pub fn from_index(items: &'a [T], start_index: isize) -> Self {
        Self::with_bounds(items, start_index, -1)
    }

    /// Iterates a normalized half-open range.
    ///
    /// Negative or out-of-range end positions mean the slice end. A negative
    /// start, or one at/after the normalized end, is reset to zero, matching
    /// Hutool's `ArrayIter` behavior.
    #[must_use]
    pub fn with_bounds(items: &'a [T], start_index: isize, end_index: isize) -> Self {
        let end_index = usize::try_from(end_index)
            .ok()
            .filter(|end| *end > 0 && *end < items.len())
            .unwrap_or(items.len());
        let start_index = usize::try_from(start_index)
            .ok()
            .filter(|start| *start < end_index)
            .unwrap_or_default();
        Self {
            items,
            start_index,
            end_index,
            index: start_index,
        }
    }

    /// Returns the original borrowed slice.
    #[must_use]
    pub const fn get_array(&self) -> &'a [T] {
        self.items
    }
}

impl<'a, T> Iterator for ArrayIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.end_index {
            return None;
        }
        let item = &self.items[self.index];
        self.index += 1;
        Some(item)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.end_index.saturating_sub(self.index);
        (remaining, Some(remaining))
    }
}

impl<T> ExactSizeIterator for ArrayIter<'_, T> {}

impl<T> ResettableIter for ArrayIter<'_, T> {
    fn reset(&mut self) {
        self.index = self.start_index;
    }
}
