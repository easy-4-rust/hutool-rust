use std::{
    panic::{AssertUnwindSafe, catch_unwind},
    sync::atomic::{AtomicBool, Ordering},
};

use thiserror::Error;

use crate::{AuthorizationError, DiagnosticAction, DiagnosticPermit};

/// Heap profiling failures.
#[derive(Debug, Error)]
pub enum HeapProfileError {
    /// The operation was not authorized.
    #[error(transparent)]
    Authorization(#[from] AuthorizationError),
    /// DHAT permits only one active session.
    #[error("a heap profile session is already active")]
    AlreadyActive,
    /// An independently started DHAT profiler conflicted with this session.
    #[error("the DHAT backend rejected the profiling session")]
    BackendPanic,
}
