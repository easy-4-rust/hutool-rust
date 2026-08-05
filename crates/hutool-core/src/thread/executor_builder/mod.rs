//! 对齐: `cn.hutool.core.thread.ExecutorBuilder`
//! 来源: hutool-core/src/main/java/cn/hutool/core/thread/ExecutorBuilder.java

use std::sync::mpsc::{self, Sender, SyncSender};

mod executor_builder;
mod simple_executor;

pub use executor_builder::ExecutorBuilder;
pub use simple_executor::SimpleExecutor;

enum QueueKind {
    /// 无界（近似 LinkedBlockingQueue 大容量）。
    Unbounded,
    /// 有界 ArrayBlockingQueue。
    Bounded(usize),
    /// SynchronousQueue：无缓冲，直接 hand-off（此处用容量 0 同步通道近似）。
    Synchronous,
}

pub(crate) enum JobChannel {
    Unbounded(Sender<Box<dyn FnOnce() + Send + 'static>>),
    Bounded(SyncSender<Box<dyn FnOnce() + Send + 'static>>),
}

impl JobChannel {
    fn try_send(
        &self,
        job: Box<dyn FnOnce() + Send + 'static>,
    ) -> Result<(), Box<dyn FnOnce() + Send + 'static>> {
        match self {
            JobChannel::Unbounded(tx) => tx.send(job).map_err(|e| e.0),
            JobChannel::Bounded(tx) => tx.try_send(job).map_err(|e| match e {
                mpsc::TrySendError::Full(j) | mpsc::TrySendError::Disconnected(j) => j,
            }),
        }
    }
}
