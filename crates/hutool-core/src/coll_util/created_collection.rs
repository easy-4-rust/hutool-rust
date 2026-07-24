//! General collection operations aligned with Hutool's `CollUtil` capability model.

use std::{
    cmp::Ordering,
    collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque},
    fmt::Display,
    hash::Hash,
    sync::{
        Arc,
        mpsc::{Receiver, RecvError, SendError, SyncSender, sync_channel},
    },
};

use indexmap::{IndexMap, IndexSet};
use parking_lot::Mutex;

use crate::{CoreError, IterUtil, ListUtil, Result};

use super::coll_util::CollUtil;

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
