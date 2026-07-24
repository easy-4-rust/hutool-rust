//! Stream progress callbacks aligned with Hutool `StreamProgress`.
//!
//! Java: `cn.hutool.core.io.StreamProgress` (used by HttpUtil download overloads).

use super::stream_progress::StreamProgress;

/// No-op progress implementation for callers that pass `null` in Hutool.
#[derive(Debug, Clone, Copy, Default)]
pub struct NoopStreamProgress;

impl StreamProgress for NoopStreamProgress {}
