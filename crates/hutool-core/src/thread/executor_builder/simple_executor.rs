//! 对齐: `cn.hutool.core.thread.ExecutorBuilder`
//! 来源: hutool-core/src/main/java/cn/hutool/core/thread/ExecutorBuilder.java

use crate::thread::reject_policy::RejectPolicy;
use std::sync::mpsc;
use std::sync::Mutex;
use std::thread::{self, JoinHandle};

/// 简易线程池（对齐 ExecutorBuilder 构建结果的可运行子集）
pub struct SimpleExecutor {
    pub(crate) tx: Mutex<Option<JobChannel>>,
    pub(crate) handles: Mutex<Vec<JoinHandle<()>>>,
    pub(crate) reject: RejectPolicy,
}

impl SimpleExecutor {
    /// 提交任务
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let mut guard = self.tx.lock().unwrap();
        if let Some(tx) = guard.as_ref() {
            if let Err(job) = tx.try_send(Box::new(f)) {
                Self::reject_job(self.reject, job);
            }
        } else {
            // shutdown 后：CALLER_RUNS / BLOCK 在调用线程执行
            match self.reject {
                RejectPolicy::ABORT | RejectPolicy::DISCARD | RejectPolicy::DISCARD_OLDEST => {}
                RejectPolicy::CALLER_RUNS | RejectPolicy::BLOCK => f(),
            }
        }
    }

    fn reject_job(policy: RejectPolicy, job: Box<dyn FnOnce() + Send + 'static>) {
        match policy {
            RejectPolicy::ABORT => {
                // 对齐 AbortPolicy：丢弃并视为拒绝（Rust 不抛 RejectedExecutionException）
            }
            RejectPolicy::DISCARD | RejectPolicy::DISCARD_OLDEST => {}
            RejectPolicy::CALLER_RUNS | RejectPolicy::BLOCK => job(),
        }
    }

    /// 关闭线程池
    pub fn shutdown(&self) {
        let mut guard = self.tx.lock().unwrap();
        *guard = None;
        drop(guard);
        let mut handles = self.handles.lock().unwrap();
        for h in handles.drain(..) {
            let _ = h.join();
        }
    }

    /// 提交并返回 JoinHandle（供 GlobalThreadPool.submit 使用）。
    pub fn submit<F, T>(&self, f: F) -> thread::JoinHandle<T>
    where
        F: FnOnce() -> T + Send + 'static,
        T: Send + 'static,
    {
        let (tx, rx) = mpsc::channel();
        self.execute(move || {
            let _ = tx.send(f());
        });
        thread::spawn(move || rx.recv().expect("task result"))
    }
}

impl Drop for SimpleExecutor {
    fn drop(&mut self) {
        // Arc 共享时仅最后一个 owner drop；避免对全局池误关。
        // 显式 shutdown() 仍可用。
    }
}

use super::JobChannel;
