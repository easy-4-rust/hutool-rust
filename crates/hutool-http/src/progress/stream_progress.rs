//! 对齐: `cn.hutool.core.io.StreamProgress`
//! 来源: hutool-core/src/main/java/cn/hutool/core/io/StreamProgress.java
//! 中文说明: 流进度回调trait，定义开始、进度和结束通知接口

/// Callback notified while downloading / copying a stream.
///
/// Java: `cn.hutool.core.io.StreamProgress`
pub trait StreamProgress: Send {
    /// Called once before transfer starts with the known total size (`-1` if unknown).
    ///
    /// Java: `StreamProgress.start()`
    fn start(&self) {}

    /// Called after each chunk with cumulative bytes transferred and total size.
    ///
    /// Java: `StreamProgress.progress(long total, long progressSize)`
    fn progress(&self, _total: i64, _progress_size: i64) {}

    /// Called when the transfer completes successfully.
    ///
    /// Java: `StreamProgress.finish()`
    fn finish(&self) {}
}
