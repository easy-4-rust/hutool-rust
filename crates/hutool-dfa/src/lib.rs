//! Multi-pattern matching and Hutool-compatible sensitive-word processing.

#![forbid(unsafe_code)]

mod matcher;
pub mod prelude;
mod sensitive;
mod sensitive_processor;
mod stop_char;
mod word_tree;

pub use matcher::{DfaMatcher, PatternMatch};
pub use sensitive_processor::{DefaultSensitiveProcessor, SensitiveProcessor};
pub use sensitive::SensitiveUtil;
pub use stop_char::StopChar;
pub use word_tree::{FoundWord, MatchOptions, WordTree};

/// Errors returned by DFA construction and serialized filtering.
#[derive(Debug, thiserror::Error)]
pub enum DfaError {
    /// No patterns were provided.
    #[error("at least one pattern is required")]
    EmptyPatterns,
    /// An empty pattern was provided.
    #[error("patterns must not be empty")]
    EmptyPattern,
    /// The pattern set exceeds the defensive construction bound.
    #[error("pattern set is too large: {actual} exceeds {maximum}")]
    PatternSetTooLarge {
        /// Observed pattern count or byte count.
        actual: usize,
        /// Configured upper bound.
        maximum: usize,
    },
    /// JSON conversion failed.
    #[error("JSON conversion failed: {0}")]
    Json(#[from] serde_json::Error),
}
