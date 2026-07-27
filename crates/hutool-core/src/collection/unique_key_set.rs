//! Hutool-aligned collection types with Rust-native ownership and concurrency.

/// 对齐: `cn.hutool.core.collection.UniqueKeySet`
/// 唯一键集合

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

/// A set whose uniqueness is determined by a caller-provided key function.
pub struct UniqueKeySet<K, V, F> {
    values: IndexMap<K, V>,
    key_of: F,
}

impl<K, V, F> UniqueKeySet<K, V, F>
where
    K: Eq + Hash,
    F: Fn(&V) -> K,
{
    /// Creates an empty set.
    #[must_use]
    pub fn new(key_of: F) -> Self {
        Self {
            values: IndexMap::new(),
            key_of,
        }
    }

    /// Creates an empty set with initial capacity.
    #[must_use]
    pub fn with_capacity(capacity: usize, key_of: F) -> Self {
        Self {
            values: IndexMap::with_capacity(capacity),
            key_of,
        }
    }

    /// Creates a set from values, replacing earlier duplicate keys.
    #[must_use]
    pub fn from_values(key_of: F, values: impl IntoIterator<Item = V>) -> Self {
        let mut set = Self::new(key_of);
        set.extend_replace(values);
        set
    }

    /// Inserts or replaces a value and reports whether its key was new.
    pub fn insert(&mut self, value: V) -> bool {
        let key = (self.key_of)(&value);
        self.values.insert(key, value).is_none()
    }

    /// Inserts a value only when its generated key is absent.
    pub fn insert_if_absent(&mut self, value: V) -> bool {
        let key = (self.key_of)(&value);
        if self.values.contains_key(&key) {
            return false;
        }
        self.values.insert(key, value);
        true
    }

    /// Inserts every absent value and reports whether at least one was added.
    pub fn extend_if_absent(&mut self, values: impl IntoIterator<Item = V>) -> bool {
        let mut changed = false;
        for value in values {
            changed = self.insert_if_absent(value) || changed;
        }
        changed
    }

    /// Inserts all values, replacing values with duplicate generated keys.
    pub fn extend_replace(&mut self, values: impl IntoIterator<Item = V>) {
        for value in values {
            self.insert(value);
        }
    }

    /// Returns whether a value with the same generated key exists.
    #[must_use]
    pub fn contains_value(&self, value: &V) -> bool {
        self.values.contains_key(&(self.key_of)(value))
    }

    /// Removes and returns the value with the same generated key.
    pub fn remove_value(&mut self, value: &V) -> Option<V> {
        self.values.shift_remove(&(self.key_of)(value))
    }

    /// Gets a value by its generated key.
    #[must_use]
    pub fn get(&self, key: &K) -> Option<&V> {
        self.values.get(key)
    }

    /// Iterates values in first-key insertion order.
    pub fn iter(&self) -> impl ExactSizeIterator<Item = &V> {
        self.values.values()
    }

    /// Returns the number of unique keys.
    #[must_use]
    pub fn len(&self) -> usize {
        self.values.len()
    }

    /// Returns whether no values are present.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    /// Clears all values.
    pub fn clear(&mut self) {
        self.values.clear();
    }
}

impl<K: Clone, V: Clone, F: Clone> Clone for UniqueKeySet<K, V, F> {
    fn clone(&self) -> Self {
        Self {
            values: self.values.clone(),
            key_of: self.key_of.clone(),
        }
    }
}

impl<K: fmt::Debug, V: fmt::Debug, F> fmt::Debug for UniqueKeySet<K, V, F> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("UniqueKeySet")
            .field("values", &self.values)
            .finish_non_exhaustive()
    }
}

use super::{Comparator};
