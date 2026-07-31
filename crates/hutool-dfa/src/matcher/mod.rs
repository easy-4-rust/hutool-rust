//! Immutable high-throughput matching backed by `aho-corasick`.

mod pattern_match;
mod dfa_matcher;

pub use pattern_match::PatternMatch;
pub use dfa_matcher::DfaMatcher;
