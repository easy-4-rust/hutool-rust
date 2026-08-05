//! Borrowed and streaming collection partitions aligned with Hutool.

/// 对齐: `cn.hutool.core.collection.AvgPartition`
/// 平均分区
use crate::{CoreError, Result};

/// A borrowed view that distributes a slice across exactly `limit` partitions.
///
/// Remainder elements are assigned one each to the earliest partitions. When
/// `limit` is greater than the slice length, trailing partitions are empty.
#[derive(Clone, Copy, Debug)]
pub struct AvgPartition<'a, T> {
    items: &'a [T],
    limit: usize,
}

impl<'a, T> AvgPartition<'a, T> {
    /// Creates an average partition view.
    ///
    /// # Errors
    ///
    /// Returns [`CoreError::InvalidArgument`] when `limit` is zero.
    pub fn new(items: &'a [T], limit: usize) -> Result<Self> {
        if limit == 0 {
            return Err(CoreError::InvalidArgument {
                name: "limit",
                reason: "must be greater than zero",
            });
        }
        // 对齐 Hutool `ListUtil.splitAvg(List, int)`:当 list 为空时,
        // 返回 empty(零分片),而不是 limit 个空分片。
        // 见 hutool ListUtil.splitAvg 实现:`if (CollUtil.isEmpty(list)) return empty();`
        let limit = if items.is_empty() { 0 } else { limit };
        Ok(Self { items, limit })
    }

    /// Returns the requested partition count, including empty trailing views.
    #[must_use]
    pub const fn len(&self) -> usize {
        self.limit
    }

    /// Average partitions are never empty because `limit` must be positive.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        false
    }

    /// Returns one average partition by index.
    #[must_use]
    pub fn get(&self, index: usize) -> Option<&'a [T]> {
        if index >= self.limit {
            return None;
        }
        let base_size = self.items.len() / self.limit;
        let remainder = self.items.len() % self.limit;
        let start = index * base_size + index.min(remainder);
        let end = start + base_size + usize::from(index < remainder);
        Some(&self.items[start..end])
    }

    /// Iterates exactly `limit` partitions.
    pub fn iter(&self) -> impl DoubleEndedIterator<Item = &'a [T]> + ExactSizeIterator + '_ {
        (0..self.limit).map(|index| {
            // Every value in this range is valid by construction.
            self.get(index).unwrap_or(&self.items[0..0])
        })
    }
}
