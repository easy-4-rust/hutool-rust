//! Hutool-aligned collection types with Rust-native ownership and concurrency.

/// 对齐: `cn.hutool.core.collection.CollectionUtil`
/// 集合类型定义

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

mod bounded_priority_queue;
mod concurrent_hash_set;
mod unique_key_set;

pub use bounded_priority_queue::BoundedPriorityQueue;
pub use concurrent_hash_set::ConcurrentHashSet;
pub use unique_key_set::UniqueKeySet;

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

pub fn ring_next_for_len<T>(items: &[T], index: &AtomicUsize) -> Result<usize> {
    ring_next_index(items.len(), index)
}

type Comparator<T> = dyn Fn(&T, &T) -> Ordering + Send + Sync;
