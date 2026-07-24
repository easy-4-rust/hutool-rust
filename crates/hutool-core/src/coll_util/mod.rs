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

mod collection_kind;
mod created_collection;
mod blocking_queue;
mod coll_util;

pub use collection_kind::CollectionKind;
pub use created_collection::CreatedCollection;
pub use blocking_queue::BlockingQueue;
pub use coll_util::CollUtil;
