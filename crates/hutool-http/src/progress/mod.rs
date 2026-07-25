//! 对齐: `cn.hutool.core.io` (流进度模块)
//! 来源: hutool-core/src/main/java/cn/hutool/core/io/StreamProgress.java
//! 中文说明: 流进度回调模块，用于下载/复制时的进度通知

mod stream_progress;
mod noop_stream_progress;
mod fn_stream_progress;

pub use stream_progress::StreamProgress;
pub use noop_stream_progress::NoopStreamProgress;
pub use fn_stream_progress::FnStreamProgress;
