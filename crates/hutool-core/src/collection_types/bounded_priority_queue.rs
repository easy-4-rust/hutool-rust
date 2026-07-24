//! Hutool-aligned collection types with Rust-native ownership and concurrency.

use std::{
    cmp::Ordering,
    collections::HashSet,
    fmt,
    hash::Hash,
    sync::{
        Arc,
        atomic::{AtomicU64, AtomicUsize, Ordering as AtomicOrdering},
    },
};

use indexmap::IndexMap;
use parking_lot::RwLock;

use crate::{CoreError, Result};

/// A bounded priority queue that retains the best `capacity` values.
///
/// Values considered smaller by the comparator are retained, matching
/// Hutool's natural-order behavior.
pub struct BoundedPriorityQueue<T> {
    capacity: usize,
    comparator: Arc<Comparator<T>>,
    values: Vec<T>,
}

impl<T> BoundedPriorityQueue<T> {
    /// Creates a queue using a custom ordering function.
    pub fn with_comparator(
        capacity: usize,
        comparator: impl Fn(&T, &T) -> Ordering + Send + Sync + 'static,
    ) -> Result<Self> {
        if capacity == 0 {
            return Err(CoreError::InvalidArgument {
                name: "capacity",
                reason: "must be greater than zero",
            });
        }
        Ok(Self {
            capacity,
            comparator: Arc::new(comparator),
            values: Vec::with_capacity(capacity),
        })
    }

    /// Offers one value. An inferior value is ignored but still reports success.
    pub fn offer(&mut self, value: T) -> bool {
        if self.values.len() < self.capacity {
            self.values.push(value);
            return true;
        }
        let worst = self
            .values
            .iter()
            .enumerate()
            .max_by(|(_, left), (_, right)| (self.comparator)(left, right))
            .map(|(index, _)| index)
            .unwrap_or_default();
        if (self.comparator)(&value, &self.values[worst]) == Ordering::Less {
            self.values[worst] = value;
        }
        true
    }

    /// Offers every value and reports whether the input was non-empty.
    pub fn extend_values(&mut self, values: impl IntoIterator<Item = T>) -> bool {
        let mut changed = false;
        for value in values {
            self.offer(value);
            changed = true;
        }
        changed
    }

    /// Returns the number of retained values.
    #[must_use]
    pub fn len(&self) -> usize {
        self.values.len()
    }

    /// Returns whether no values are retained.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    /// Returns the configured capacity.
    #[must_use]
    pub const fn capacity(&self) -> usize {
        self.capacity
    }

    /// Returns the currently worst retained value.
    #[must_use]
    pub fn peek_worst(&self) -> Option<&T> {
        self.values
            .iter()
            .max_by(|left, right| (self.comparator)(left, right))
    }

    /// Removes and returns the currently worst retained value.
    pub fn pop_worst(&mut self) -> Option<T> {
        let index = self
            .values
            .iter()
            .enumerate()
            .max_by(|(_, left), (_, right)| (self.comparator)(left, right))
            .map(|(index, _)| index)?;
        Some(self.values.swap_remove(index))
    }

    /// Returns a comparator-sorted snapshot.
    #[must_use]
    pub fn to_sorted_vec(&self) -> Vec<T>
    where
        T: Clone,
    {
        let mut values = self.values.clone();
        values.sort_by(|left, right| (self.comparator)(left, right));
        values
    }

    /// Clears all retained values.
    pub fn clear(&mut self) {
        self.values.clear();
    }
}

impl<T: Ord + 'static> BoundedPriorityQueue<T> {
    /// Creates a natural-order queue.
    pub fn new(capacity: usize) -> Result<Self> {
        Self::with_comparator(capacity, Ord::cmp)
    }
}

impl<T: fmt::Debug + Clone> fmt::Debug for BoundedPriorityQueue<T> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("BoundedPriorityQueue")
            .field("capacity", &self.capacity)
            .field("values", &self.to_sorted_vec())
            .finish_non_exhaustive()
    }
}

use super::{Comparator};
