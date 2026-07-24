//! Stream progress callbacks aligned with Hutool `StreamProgress`.
//!
//! Java: `cn.hutool.core.io.StreamProgress` (used by HttpUtil download overloads).

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
