//! Borrowed and streaming collection partitions aligned with Hutool.

/// 对齐: `cn.hutool.core.collection.PartitionIter`
/// 分区迭代器

use std::iter::Peekable;

use crate::Result;

/// An iterator adapter that collects source items into fixed-size vectors.
pub struct PartitionIter<I>
where
    I: Iterator,
{
    source: Peekable<I>,
    partition_size: usize,
}

impl<I> PartitionIter<I>
where
    I: Iterator,
{
    /// Creates a streaming partition adapter.
    ///
    /// # Errors
    ///
    /// Returns [`CoreError::InvalidArgument`] when `partition_size` is zero.
    pub fn new(source: I, partition_size: usize) -> Result<Self> {
        validate_partition_size(partition_size)?;
        Ok(Self {
            source: source.peekable(),
            partition_size,
        })
    }

    /// Reports whether another partition is available without consuming it.
    pub fn has_next(&mut self) -> bool {
        self.source.peek().is_some()
    }
}

impl<I> Iterator for PartitionIter<I>
where
    I: Iterator,
{
    type Item = Vec<I::Item>;

    fn next(&mut self) -> Option<Self::Item> {
        let first = self.source.next()?;
        let mut partition = Vec::with_capacity(self.partition_size);
        partition.push(first);
        partition.extend(self.source.by_ref().take(self.partition_size - 1));
        Some(partition)
    }
}

use super::{validate_partition_size};
