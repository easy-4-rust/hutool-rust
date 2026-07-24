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

/// Concrete collection kinds replacing Java's reflective `Class<?>` factory.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CollectionKind {
    /// Contiguous growable list.
    List,
    /// Double-ended linked-style list.
    Deque,
    /// Unordered unique collection.
    Set,
    /// Insertion-ordered unique collection.
    OrderedSet,
    /// Key-ordered unique collection.
    SortedSet,
}
