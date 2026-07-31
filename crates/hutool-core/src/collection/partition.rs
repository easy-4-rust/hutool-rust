//! Borrowed and streaming collection partitions aligned with Hutool.

/// 对齐: `cn.hutool.core.collection.Partition`
/// 分区


use crate::Result;

use super::validate_partition_size;


/// A borrowed view that divides a slice into fixed-size partitions.
///
/// This is the Rust-native counterpart of Hutool's `Partition` and
/// `RandomAccessPartition`: slices already provide constant-time random access.
#[derive(Clone, Copy, Debug)]
pub struct Partition<'a, T> {
    items: &'a [T],
    partition_size: usize,
}

impl<'a, T> Partition<'a, T> {
    /// Creates a partition view.
    ///
    /// # Errors
    ///
    /// Returns [`CoreError::InvalidArgument`] when `partition_size` is zero.
    pub fn new(items: &'a [T], partition_size: usize) -> Result<Self> {
        validate_partition_size(partition_size)?;
        Ok(Self {
            items,
            partition_size,
        })
    }

    /// Returns the number of partitions.
    #[must_use]
    pub fn len(&self) -> usize {
        self.items.len().div_ceil(self.partition_size)
    }

    /// Returns whether this view has no partitions.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Returns one partition by index.
    #[must_use]
    pub fn get(&self, index: usize) -> Option<&'a [T]> {
        if index >= self.len() {
            return None;
        }
        let start = index * self.partition_size;
        let end = (start + self.partition_size).min(self.items.len());
        Some(&self.items[start..end])
    }

    /// Iterates all partitions without allocating their elements.
    pub fn iter(&self) -> impl DoubleEndedIterator<Item = &'a [T]> + ExactSizeIterator + '_ {
        self.items.chunks(self.partition_size)
    }
}

