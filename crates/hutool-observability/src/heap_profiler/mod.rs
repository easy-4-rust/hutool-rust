use std::{
    panic::{AssertUnwindSafe, catch_unwind},
    sync::atomic::{AtomicBool, Ordering},
};

use thiserror::Error;

use crate::{AuthorizationError, DiagnosticAction, DiagnosticPermit};

mod heap_profile_session;
mod heap_profile_error;

pub use heap_profile_session::HeapProfileSession;
pub use heap_profile_error::HeapProfileError;
