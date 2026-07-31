//! Borrowed and streaming collection partitions aligned with Hutool.

/// 对齐: `cn.hutool.core.collection.RandomAccessAvgPartition`
/// 随机访问平均分区



use super::avg_partition::AvgPartition;

/// `AvgPartition` already has the random-access semantics of Hutool's marker type.
pub type RandomAccessAvgPartition<'a, T> = AvgPartition<'a, T>;

