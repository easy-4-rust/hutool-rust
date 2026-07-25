//! 对齐: `cn.hutool.core.io.NoopStreamProgress`
//! 来源: hutool-core/src/main/java/cn/hutool/core/io/NoopStreamProgress.java
//! 中文说明: 空操作流进度实现，用于不需要进度通知的场景

use super::stream_progress::StreamProgress;

/// No-op progress implementation for callers that pass `null` in Hutool.
#[derive(Debug, Clone, Copy, Default)]
pub struct NoopStreamProgress;

impl StreamProgress for NoopStreamProgress {}
