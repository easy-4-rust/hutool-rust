//! General collection operations aligned with Hutool's `CollUtil` capability model.

/// 对齐: `cn.hutool.core.collection.CollUtil`
/// 创建集合

use std::collections::{BTreeSet, HashSet, VecDeque};

use indexmap::IndexSet;


/// A statically typed result for [`CollUtil::create`].
#[derive(Debug, Clone)]
pub enum CreatedCollection<T> {
    /// [`Vec`] collection.
    List(Vec<T>),
    /// [`VecDeque`] collection.
    Deque(VecDeque<T>),
    /// [`HashSet`] collection.
    Set(HashSet<T>),
    /// [`IndexSet`] collection.
    OrderedSet(IndexSet<T>),
    /// [`BTreeSet`] collection.
    SortedSet(BTreeSet<T>),
}
