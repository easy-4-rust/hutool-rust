//! General collection operations aligned with Hutool's `CollUtil` capability model.

/// 对齐: `cn.hutool.core.collection.CollUtil`
/// 阻塞队列

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

/// A bounded multi-producer queue with blocking send and receive operations.
#[derive(Debug)]
pub struct BlockingQueue<T> {
    pub(crate) sender: SyncSender<T>,
    pub(crate) receiver: Mutex<Receiver<T>>,
}

impl<T> BlockingQueue<T> {
    /// Sends a value, waiting while the queue is full.
    pub fn send(&self, value: T) -> std::result::Result<(), SendError<T>> {
        self.sender.send(value)
    }

    /// Receives a value, waiting while the queue is empty.
    pub fn recv(&self) -> std::result::Result<T, RecvError> {
        self.receiver.lock().recv()
    }
}
