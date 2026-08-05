//! 对齐: `cn.hutool.core.thread.ThreadUtil`
//! 来源: hutool-core/src/main/java/cn/hutool/core/thread/ThreadUtil.java
//!
//! 以 `std::thread` 提供可移植子集；JVM `ThreadLocal` / `ThreadGroup` 全局语义保持 planned。

use std::sync::{Condvar, Mutex};

/// 对齐 `CountDownLatch` 可移植子集。
#[derive(Debug)]
pub struct CountDownLatch {
    inner: Mutex<usize>,
    cv: Condvar,
}

impl CountDownLatch {
    /// 创建门闩。
    pub fn new(count: usize) -> Self {
        Self {
            inner: Mutex::new(count),
            cv: Condvar::new(),
        }
    }

    /// 计数减一。
    pub fn count_down(&self) {
        let mut g = self.inner.lock().unwrap();
        if *g > 0 {
            *g -= 1;
            if *g == 0 {
                self.cv.notify_all();
            }
        }
    }

    /// 等待归零。
    pub fn await_zero(&self) {
        let mut g = self.inner.lock().unwrap();
        while *g > 0 {
            g = self.cv.wait(g).unwrap();
        }
    }
}
