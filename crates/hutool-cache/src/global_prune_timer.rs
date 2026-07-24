//! `GlobalPruneTimer` + `PruneHandle` — 对齐 `cn.hutool.cache.GlobalPruneTimer`。

use std::fmt;
use std::sync::mpsc;
use std::thread::{self, JoinHandle};
use std::time::Duration;

pub(crate) struct PruneHandle {
    pub(crate) stop: Option<mpsc::Sender<()>>,
    pub(crate) worker: Option<JoinHandle<()>>,
}
impl fmt::Debug for PruneHandle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { f.debug_struct("PruneHandle").finish() }
}
impl Drop for PruneHandle {
    fn drop(&mut self) {
        if let Some(stop) = self.stop.take() { let _ = stop.send(()); }
        if let Some(worker) = self.worker.take() { let _ = worker.join(); }
    }
}

pub struct GlobalPruneTimer;
impl GlobalPruneTimer {
    pub fn schedule<F>(task: F, delay: Duration) -> PruneHandle where F: FnMut() + Send + 'static {
        Self::schedule_boxed(Box::new(task), delay)
    }
    fn schedule_boxed(mut task: Box<dyn FnMut() + Send>, delay: Duration) -> PruneHandle {
        let delay = if delay.is_zero() { Duration::from_millis(1) } else { delay };
        let (stop, receiver) = mpsc::channel();
        let worker = thread::spawn(move || loop {
            match receiver.recv_timeout(delay) {
                Ok(()) | Err(mpsc::RecvTimeoutError::Disconnected) => break,
                Err(mpsc::RecvTimeoutError::Timeout) => task(),
            }
        });
        PruneHandle { stop: Some(stop), worker: Some(worker) }
    }
    pub const fn create() {}
    pub const fn shutdown() {}
    pub fn shutdown_now() -> Vec<JoinHandle<()>> { Vec::new() }
}
