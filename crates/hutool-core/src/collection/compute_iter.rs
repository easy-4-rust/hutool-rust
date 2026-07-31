//! Stateful and view-based collection adapters aligned with Hutool.

/// 对齐: `cn.hutool.core.collection.ComputeIter`
/// 计算迭代器



/// An iterator whose next value is supplied by a stateful computation.
pub struct ComputeIter<T, F>
where
    F: FnMut() -> Option<T>,
{
    compute_next: F,
    next_item: Option<T>,
    finished: bool,
}

impl<T, F> ComputeIter<T, F>
where
    F: FnMut() -> Option<T>,
{
    /// Creates a computed iterator.
    #[must_use]
    pub fn new(compute_next: F) -> Self {
        Self {
            compute_next,
            next_item: None,
            finished: false,
        }
    }

    /// Reports whether another computed value is available without consuming it.
    pub fn has_next(&mut self) -> bool {
        if self.next_item.is_some() {
            return true;
        }
        if self.finished {
            return false;
        }
        self.next_item = (self.compute_next)();
        if self.next_item.is_none() {
            self.finished = true;
            return false;
        }
        true
    }

    /// Manually finishes iteration and clears a cached value.
    pub fn finish(&mut self) {
        self.finished = true;
        self.next_item = None;
    }

    /// Resets the finished/cache state; caller-owned computation state is unchanged.
    pub fn reset_state(&mut self) {
        self.finished = false;
        self.next_item = None;
    }
}

impl<T, F> Iterator for ComputeIter<T, F>
where
    F: FnMut() -> Option<T>,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.has_next().then(|| self.next_item.take()).flatten()
    }
}
