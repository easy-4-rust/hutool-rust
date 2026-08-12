//! Immutable high-throughput matching backed by `aho-corasick`.

mod dfa_matcher;
mod pattern_match;

pub use dfa_matcher::DfaMatcher;
pub use pattern_match::PatternMatch;
