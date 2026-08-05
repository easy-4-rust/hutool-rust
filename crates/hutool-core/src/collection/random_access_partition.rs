//! Borrowed and streaming collection partitions aligned with Hutool.

/// 对齐: `cn.hutool.core.collection.RandomAccessPartition`
/// 随机访问分区
use super::partition::Partition;

/// `Partition` already has the random-access semantics of Hutool's marker type.
pub type RandomAccessPartition<'a, T> = Partition<'a, T>;
