//! Stateful and view-based collection adapters aligned with Hutool.

/// 对齐: `cn.hutool.core.collection.TransCollection`
/// 转换集合

use std::marker::PhantomData;


/// A mutable collection view that transforms values only when observed.
pub struct TransCollection<'a, T, U, F>
where
    F: Fn(&T) -> U,
{
    source: &'a mut Vec<T>,
    transform: F,
    target: PhantomData<fn() -> U>,
}

impl<'a, T, U, F> TransCollection<'a, T, U, F>
where
    F: Fn(&T) -> U,
{
    /// Creates a live transforming view over `source`.
    pub fn new(source: &'a mut Vec<T>, transform: F) -> Self {
        Self {
            source,
            transform,
            target: PhantomData,
        }
    }

    /// Iterates lazily transformed values.
    pub fn iter(&self) -> impl ExactSizeIterator<Item = U> + '_ {
        self.source.iter().map(&self.transform)
    }

    /// Clears the source collection.
    pub fn clear(&mut self) {
        self.source.clear();
    }

    /// Returns whether the source is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.source.is_empty()
    }

    /// Returns the source length.
    #[must_use]
    pub fn len(&self) -> usize {
        self.source.len()
    }

    /// Visits every lazily transformed value.
    pub fn for_each(&self, action: impl FnMut(U)) {
        self.iter().for_each(action);
    }

    /// Removes source values whose transformed form matches `predicate`.
    pub fn remove_if(&mut self, mut predicate: impl FnMut(&U) -> bool) -> bool {
        let original_len = self.source.len();
        let transform = &self.transform;
        self.source.retain(|value| !predicate(&transform(value)));
        self.source.len() != original_len
    }
}
