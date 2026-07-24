use pprof::{ProfilerGuard, ProfilerGuardBuilder, protos::Message};
use thiserror::Error;

use crate::{AuthorizationError, DiagnosticAction, DiagnosticPermit};

mod cpu_profile_config;
mod cpu_profile_session;
mod cpu_profile_error;

pub use cpu_profile_config::CpuProfileConfig;
pub use cpu_profile_session::CpuProfileSession;
pub use cpu_profile_error::CpuProfileError;
