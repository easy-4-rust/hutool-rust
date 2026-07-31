//! Hutool-aligned iterator adapters with Rust-native ownership semantics.

/// 对齐: `cn.hutool.core.collection.FilterIter`
/// 过滤迭代器


/// A filtering iterator with non-consuming lookahead.
pub struct FilterIter<I, P>
where
    I: Iterator,
{
    source: I,
    filter: Option<P>,
    next_item: Option<I::Item>,
}

impl<I, P> FilterIter<I, P>
where
    I: Iterator,
    P: FnMut(&I::Item) -> bool,
{
    /// Creates a filtering adapter. `None` accepts every value.
    #[must_use]
    pub fn new(source: I, filter: Option<P>) -> Self {
        Self {
            source,
            filter,
            next_item: None,
        }
    }

    /// Reports whether an accepted value remains without consuming it.
    pub fn has_next(&mut self) -> bool {
        self.next_item.is_some() || self.set_next_item()
    }

    /// Returns the wrapped iterator.
    #[must_use]
    pub const fn get_iterator(&self) -> &I {
        &self.source
    }

    /// Returns the optional predicate.
    #[must_use]
    pub const fn get_filter(&self) -> Option<&P> {
        self.filter.as_ref()
    }

    fn set_next_item(&mut self) -> bool {
        self.next_item = self
            .source
            .find(|item| self.filter.as_mut().is_none_or(|filter| filter(item)));
        self.next_item.is_some()
    }
}

impl<I, P> Iterator for FilterIter<I, P>
where
    I: Iterator,
    P: FnMut(&I::Item) -> bool,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_item.take().or_else(|| {
            self.set_next_item();
            self.next_item.take()
        })
    }
}
