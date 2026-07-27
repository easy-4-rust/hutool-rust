//! `cn.hutool.core.collection` 子包对齐（合并后唯一入口）
//!
//! 合并来源：
//! - `coll_util/`（CollUtil 主实现）
//! - `coll_stream_util.rs`（流式风格工具）
//! - `collection_iter/`（Iter 系列）
//! - `collection_partition/`（Partition 系列）
//! - `collection_types/`（类型定义）
//! - `collection_adapters/`（适配器）
//!
//! 详细对齐信息见各 `.rs` 文件头注释。

use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::iter::Peekable;
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering as AtomicOrdering};

use indexmap::IndexMap;
use parking_lot::RwLock;

use crate::{CoreError, Result};

pub mod array_iter;
pub mod avg_partition;
pub mod blocking_queue;
pub mod bounded_priority_queue;
pub mod coll_stream_util;
pub mod coll_util;
pub mod collection_kind;
pub mod collection_util;
pub mod compute_iter;
pub mod concurrent_hash_set;
pub mod copied_iter;
pub mod created_collection;
pub mod enumeration_iter;
pub mod filter_iter;
pub mod iter_chain;
pub mod iter_util;
pub mod iterable_iter;
pub mod iterator_enumeration;
pub mod line_iter;
pub mod list_util;
pub mod node_list_iter;
pub mod partition;
pub mod partition_iter;
pub mod random_access_avg_partition;
pub mod random_access_partition;
pub mod resettable_iter;
pub mod spliterator_util;
pub mod trans_collection;
pub mod trans_iter;
pub mod trans_spliterator;
pub mod unique_key_set;

pub use array_iter::ArrayIter;
pub use avg_partition::AvgPartition;
pub use blocking_queue::BlockingQueue;
pub use bounded_priority_queue::BoundedPriorityQueue;
pub use coll_stream_util::CollStreamUtil;
pub use coll_util::CollUtil;
pub use collection_kind::CollectionKind;
pub use collection_util::CollectionUtil;
pub use compute_iter::ComputeIter;
pub use concurrent_hash_set::ConcurrentHashSet;
pub use copied_iter::CopiedIter;
pub use created_collection::CreatedCollection;
pub use enumeration_iter::EnumerationIter;
pub use filter_iter::FilterIter;
pub use iter_chain::IterChain;
pub use iter_util::IterUtil;
pub use iterable_iter::IterableIter;
pub use iterator_enumeration::IteratorEnumeration;
pub use line_iter::LineIter;
pub use list_util::ListUtil;
pub use node_list_iter::NodeListIter;
pub use partition::Partition;
pub use partition_iter::PartitionIter;
pub use random_access_avg_partition::RandomAccessAvgPartition;
pub use random_access_partition::RandomAccessPartition;
pub use resettable_iter::ResettableIter;
pub use spliterator_util::SpliteratorUtil;
pub use trans_collection::TransCollection;
pub use trans_iter::TransIter;
pub use trans_spliterator::TransSpliterator;
pub use unique_key_set::UniqueKeySet;

// ── 合并自 collection_partition/mod.rs ──

pub(crate) fn validate_partition_size(partition_size: usize) -> Result<()> {
    if partition_size == 0 {
        return Err(CoreError::InvalidArgument {
            name: "partition_size",
            reason: "must be greater than zero",
        });
    }
    Ok(())
}

// ── 合并自 collection_types/mod.rs ──

/// Java `Comparator<T>` 的 Rust 镜像。
pub(crate) type Comparator<T> = dyn Fn(&T, &T) -> Ordering + Send + Sync;

/// 原子递增取模。
pub fn ring_next_index(modulo: usize, index: &AtomicUsize) -> Result<usize> {
    if modulo == 0 {
        return Err(CoreError::InvalidArgument {
            name: "modulo",
            reason: "must be greater than zero",
        });
    }
    if modulo == 1 {
        return Ok(0);
    }
    let current = index
        .fetch_update(AtomicOrdering::AcqRel, AtomicOrdering::Relaxed, |current| {
            Some(current.wrapping_add(1) % modulo)
        })
        .unwrap_or_default();
    Ok(current.wrapping_add(1) % modulo)
}

/// 64 位原子递增取模。
pub fn ring_next_u64(modulo: u64, index: &AtomicU64) -> Result<u64> {
    if modulo == 0 {
        return Err(CoreError::InvalidArgument {
            name: "modulo",
            reason: "must be greater than zero",
        });
    }
    if modulo == 1 {
        return Ok(0);
    }
    let current = index
        .fetch_update(AtomicOrdering::AcqRel, AtomicOrdering::Relaxed, |current| {
            Some(current.wrapping_add(1) % modulo)
        })
        .unwrap_or_default();
    Ok(current.wrapping_add(1) % modulo)
}

/// 切片下标原子递增取模。
pub fn ring_next_for_len<T>(items: &[T], index: &AtomicUsize) -> Result<usize> {
    ring_next_index(items.len(), index)
}

// ── idiomatic 工具函数（合并自原 collection/mod.rs） ──

/// Returns stable, first-seen distinct elements.
#[must_use]
pub fn distinct<T>(items: &[T]) -> Vec<T>
where
    T: Clone + Eq + Hash,
{
    let mut seen = HashSet::with_capacity(items.len());
    items
        .iter()
        .filter(|item| seen.insert((*item).clone()))
        .cloned()
        .collect()
}

/// Groups cloned elements by a derived key while preserving per-group order.
#[must_use]
pub fn group_by<T, K>(items: &[T], mut key: impl FnMut(&T) -> K) -> HashMap<K, Vec<T>>
where
    T: Clone,
    K: Eq + Hash,
{
    let mut groups = HashMap::new();
    for item in items {
        groups
            .entry(key(item))
            .or_insert_with(Vec::new)
            .push(item.clone());
    }
    groups
}

/// Partitions a slice into owned chunks of `size`.
///
/// # Errors
///
/// Returns [`CoreError::InvalidArgument`] when `size` is zero.
pub fn partition<T: Clone>(items: &[T], size: usize) -> Result<Vec<Vec<T>>> {
    if size == 0 {
        return Err(CoreError::InvalidArgument {
            name: "size",
            reason: "must be greater than zero",
        });
    }
    Ok(items.chunks(size).map(<[T]>::to_vec).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn distinct_is_stable() {
        assert_eq!(distinct(&[3, 1, 3, 2, 1]), [3, 1, 2]);
    }

    #[test]
    fn partition_rejects_zero_and_keeps_tail() {
        assert!(partition(&[1, 2], 0).is_err());
        assert_eq!(
            partition(&[1, 2, 3, 4, 5], 2).unwrap(),
            vec![vec![1, 2], vec![3, 4], vec![5]]
        );
    }

    #[test]
    fn grouping_preserves_values() {
        let groups = group_by(&[1, 2, 3, 4], |value| value % 2);
        assert_eq!(groups.get(&0), Some(&vec![2, 4]));
        assert_eq!(groups.get(&1), Some(&vec![1, 3]));
    }
}
