//! Provider capability model used by the thin Hutool-compatible facade.

#![allow(missing_docs, clippy::enum_glob_use, clippy::match_same_arms)]

use std::sync::Arc;

/// Thread-safe callback receiving one provider stream event.
pub type StreamCallback = Arc<dyn Fn(String) + Send + Sync + 'static>;
