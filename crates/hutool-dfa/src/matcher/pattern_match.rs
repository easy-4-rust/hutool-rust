//! Immutable high-throughput matching backed by `aho-corasick`.

use crate::DfaError;
use aho_corasick::{AhoCorasick, AhoCorasickBuilder, MatchKind};

/// One immutable-engine match in UTF-8 byte offsets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PatternMatch {
    /// Index of the matching pattern supplied at construction time.
    pub pattern_index: usize,
    /// Inclusive UTF-8 byte start offset.
    pub start: usize,
    /// Exclusive UTF-8 byte end offset.
    pub end: usize,
    /// Matching pattern text.
    pub pattern: String,
}
