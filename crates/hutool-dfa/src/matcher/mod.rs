//! Immutable high-throughput matching backed by `aho-corasick`.

use crate::DfaError;
use aho_corasick::{AhoCorasick, AhoCorasickBuilder, MatchKind};

mod pattern_match;
mod dfa_matcher;

pub use pattern_match::PatternMatch;
pub use dfa_matcher::DfaMatcher;
