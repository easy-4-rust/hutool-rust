//! 对齐: `cn.hutool.core.io.FnStreamProgress`
//! 来源: hutool-core/src/main/java/cn/hutool/core/io/FnStreamProgress.java
//! 中文说明: 基于闭包的流进度回调实现，用于简单的下载进度通知

use super::stream_progress::StreamProgress;

/// Progress adapter over a mutable closure (tests / simple callbacks).
pub struct FnStreamProgress<F>
where
    F: Fn(i64, i64) + Send,
{
    on_progress: F,
}

impl<F> FnStreamProgress<F>
where
    F: Fn(i64, i64) + Send,
{
    /// Creates a progress callback that invokes `on_progress(total, progress_size)`.
    pub fn new(on_progress: F) -> Self {
        Self { on_progress }
    }
}

impl<F> StreamProgress for FnStreamProgress<F>
where
    F: Fn(i64, i64) + Send,
{
    fn progress(&self, total: i64, progress_size: i64) {
        (self.on_progress)(total, progress_size);
    }
}
