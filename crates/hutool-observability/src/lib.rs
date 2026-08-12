//! Composable observability primitives for Hutool-Rust applications.
//!
//! The default feature set provides tracing, Prometheus-compatible metrics,
//! and health reporting. CPU profiling, Tokio console telemetry, and heap
//! profiling are separate compile-time features and require a runtime
//! [`DiagnosticPermit`] before they can be activated.
//!
//! This crate never starts an HTTP management server. Applications own the
//! transport, authentication, runtime, and network exposure of diagnostic
//! output.

#![forbid(unsafe_code)]

mod authorization;
pub mod prelude;
pub use authorization::{
    AuthorizationError, DiagnosticAction, DiagnosticPermit, DiagnosticsAccess,
    DiagnosticsAuthorizer, StaticTokenAuthorizer,
};

#[cfg(feature = "health")]
mod health;
#[cfg(feature = "health")]
pub use health::{HealthCheck, HealthError, HealthRegistry, HealthReport, HealthStatus};

#[cfg(feature = "metrics")]
pub mod metrics;

#[cfg(feature = "tracing")]
pub mod tracing;

#[cfg(all(feature = "pprof", unix))]
mod cpu_profiler;
#[cfg(all(feature = "pprof", unix))]
pub use cpu_profiler::{CpuProfileConfig, CpuProfileError, CpuProfileSession};
#[cfg(all(feature = "pprof", not(unix)))]
mod cpu_profiler_unsupported;
#[cfg(all(feature = "pprof", not(unix)))]
pub use cpu_profiler_unsupported::{CpuProfileConfig, CpuProfileError, CpuProfileSession};

#[cfg(feature = "tokio-console")]
mod tokio_console;
#[cfg(feature = "tokio-console")]
pub use tokio_console::{
    TokioConsoleConfig, TokioConsoleError, TokioConsoleParts, tokio_console_parts,
};

#[cfg(feature = "heap-profiler")]
mod heap_profiler;
#[cfg(feature = "heap-profiler")]
pub use heap_profiler::{HeapProfileError, HeapProfileSession};

/// Re-export of the DHAT allocator for explicit selection by the final binary.
///
/// Enabling `heap-profiler` does not install this allocator. The application
/// must deliberately declare it as its `#[global_allocator]`.
#[cfg(feature = "heap-profiler")]
pub use dhat::Alloc as DhatAllocator;
