use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    time::Duration,
};

use thiserror::Error;

use crate::{AuthorizationError, DiagnosticAction, DiagnosticPermit};

/// Loopback-only Tokio console server configuration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TokioConsoleConfig {
    /// Local address used by the console gRPC server.
    pub bind: SocketAddr,
    /// Retention window for completed async resources.
    pub retention: Duration,
}

impl Default for TokioConsoleConfig {
    fn default() -> Self {
        Self {
            bind: SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 6_669),
            retention: Duration::from_secs(60),
        }
    }
}

/// Creates authorized Tokio console parts without spawning a runtime task.
///
/// The layer always binds to a loopback address. Remote access must use an
/// authenticated tunnel or management proxy owned by the application.
///
/// Building applications with this feature also requires
/// `RUSTFLAGS="--cfg tokio_unstable"` so Tokio emits task instrumentation.
pub fn tokio_console_parts(
    config: &TokioConsoleConfig,
    permit: &DiagnosticPermit,
) -> Result<TokioConsoleParts, TokioConsoleError> {
    permit.require(DiagnosticAction::TokioConsole)?;
    if !config.bind.ip().is_loopback() {
        return Err(TokioConsoleError::NonLoopback(config.bind));
    }
    if config.retention.is_zero() {
        return Err(TokioConsoleError::ZeroRetention);
    }
    let (layer, server) = console_subscriber::ConsoleLayer::builder()
        .server_addr(config.bind)
        .retention(config.retention)
        .build();
    Ok(TokioConsoleParts { layer, server })
}

/// Authorized console components. The application must add `layer` to its
/// subscriber and spawn `server.serve()` on its own Tokio runtime.
pub struct TokioConsoleParts {
    /// Console layer to add to the tracing subscriber.
    pub layer: console_subscriber::ConsoleLayer,
    /// Console server to spawn on the application's own Tokio runtime.
    pub server: console_subscriber::Server,
}

/// Tokio console setup failures.
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum TokioConsoleError {
    /// The operation was not authorized.
    #[error(transparent)]
    Authorization(#[from] AuthorizationError),
    /// The console gRPC server must bind to a loopback address.
    #[error("tokio console bind address must be loopback, got {0}")]
    NonLoopback(SocketAddr),
    /// The retention window must be non-zero.
    #[error("tokio console retention must be non-zero")]
    ZeroRetention,
}
