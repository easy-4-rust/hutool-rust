//! Stream progress callbacks aligned with Hutool `StreamProgress`.
//!
//! Java: `cn.hutool.core.io.StreamProgress` (used by HttpUtil download overloads).

mod stream_progress;
mod noop_stream_progress;
mod fn_stream_progress;

pub use stream_progress::StreamProgress;
pub use noop_stream_progress::NoopStreamProgress;
pub use fn_stream_progress::FnStreamProgress;
