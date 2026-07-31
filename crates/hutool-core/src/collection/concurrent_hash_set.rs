//! Hutool-aligned collection types with Rust-native ownership and concurrency.

/// 对齐: `cn.hutool.core.collection.ConcurrentHashSet`
/// 并发哈希集合

use std::{
    collections::HashSet,
    hash::Hash,
};

use parking_lot::RwLock;

/// A shareable concurrent hash set.
#[derive(Debug, Default)]
pub struct ConcurrentHashSet<T> {
    values: RwLock<HashSet<T>>,
}

impl<T: Eq + Hash> ConcurrentHashSet<T> {
    /// Creates an empty set.
    #[must_use]
    pub fn new() -> Self {
        Self {
            values: RwLock::new(HashSet::new()),
        }
    }

    /// Creates an empty set with capacity for at least `capacity` values.
    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            values: RwLock::new(HashSet::with_capacity(capacity)),
        }
    }

    /// Inserts a value and reports whether it was newly added.
    pub fn insert(&self, value: T) -> bool {
        self.values.write().insert(value)
    }

    /// Returns whether the set contains `value`.
    #[must_use]
    pub fn contains(&self, value: &T) -> bool {
        self.values.read().contains(value)
    }

    /// Removes a value and reports whether it existed.
    pub fn remove(&self, value: &T) -> bool {
        self.values.write().remove(value)
    }

    /// Returns the current element count.
    #[must_use]
    pub fn len(&self) -> usize {
        self.values.read().len()
    }

    /// Returns whether the set is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.values.read().is_empty()
    }

    /// Clears the set.
    pub fn clear(&self) {
        self.values.write().clear();
    }

    /// Returns a point-in-time snapshot of the values.
    #[must_use]
    pub fn snapshot(&self) -> Vec<T>
    where
        T: Clone,
    {
        self.values.read().iter().cloned().collect()
    }
}

impl<T: Eq + Hash> FromIterator<T> for ConcurrentHashSet<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self {
            values: RwLock::new(iter.into_iter().collect()),
        }
    }
}

