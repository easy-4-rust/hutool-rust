use std::{
    panic::{AssertUnwindSafe, catch_unwind},
    sync::atomic::{AtomicBool, Ordering},
};

use thiserror::Error;

use crate::{AuthorizationError, DiagnosticAction, DiagnosticPermit};

use super::heap_profile_error::HeapProfileError;

/// Authorized DHAT heap profiling session.
///
/// The final binary must also select [`crate::DhatAllocator`] as its global
/// allocator. Dropping or finishing the session writes the DHAT report.
pub struct HeapProfileSession {
    profiler: Option<dhat::Profiler>,
}

impl std::fmt::Debug for HeapProfileSession {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("HeapProfileSession")
            .field("active", &self.profiler.is_some())
            .finish()
    }
}

impl HeapProfileSession {
    /// Starts heap profiling after checking the runtime permit.
    pub fn start(permit: &DiagnosticPermit) -> Result<Self, HeapProfileError> {
        permit.require(DiagnosticAction::HeapProfile)?;
        HEAP_PROFILE_ACTIVE
            .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
            .map_err(|_| HeapProfileError::AlreadyActive)?;

        if let Ok(profiler) = catch_unwind(AssertUnwindSafe(dhat::Profiler::new_heap)) {
            Ok(Self {
                profiler: Some(profiler),
            })
        } else {
            HEAP_PROFILE_ACTIVE.store(false, Ordering::Release);
            Err(HeapProfileError::BackendPanic)
        }
    }

    /// Returns whether this session is still collecting allocations.
    #[must_use]
    pub fn is_active(&self) -> bool {
        self.profiler.is_some()
    }

    /// Stops collection and writes the report.
    pub fn finish(&mut self, permit: &DiagnosticPermit) -> Result<(), HeapProfileError> {
        permit.require(DiagnosticAction::HeapProfile)?;
        self.stop();
        Ok(())
    }

    fn stop(&mut self) {
        if let Some(profiler) = self.profiler.take() {
            drop(profiler);
            HEAP_PROFILE_ACTIVE.store(false, Ordering::Release);
        }
    }
}

impl Drop for HeapProfileSession {
    fn drop(&mut self) {
        self.stop();
    }
}

static HEAP_PROFILE_ACTIVE: AtomicBool = AtomicBool::new(false);
