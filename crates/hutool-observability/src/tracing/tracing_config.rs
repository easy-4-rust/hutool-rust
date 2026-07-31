//! Reloadable tracing configuration owned by the final application.

/// Default text tracing configuration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TracingConfig {
    /// Filter used when `RUST_LOG` is absent or invalid.
    pub default_filter: String,
    /// Whether terminal output may contain ANSI color.
    pub ansi: bool,
}

impl Default for TracingConfig {
    fn default() -> Self {
        Self {
            default_filter: "info".to_owned(),
            ansi: true,
        }
    }
}
