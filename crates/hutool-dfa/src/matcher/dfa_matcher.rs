//! 对齐: `cn.hutool.dfa.DfaMatcher` (Rust 独有，基于 aho-corasick)
//! 中文说明: 基于 DFA/Aho-Corasick 算法的高性能敏感词匹配器

use crate::DfaError;
use aho_corasick::{AhoCorasick, AhoCorasickBuilder, MatchKind};

use super::pattern_match::PatternMatch;

/// Immutable, thread-safe leftmost-longest multi-pattern matcher.
#[derive(Debug, Clone)]
pub struct DfaMatcher {
    automaton: AhoCorasick,
    patterns: Vec<String>,
}

impl DfaMatcher {
    /// Builds a matcher from non-empty, defensively bounded patterns.
    ///
    /// # Panics
    ///
    /// Panics only if a future `aho-corasick` release rejects a pattern set
    /// that is within `HiTool`'s count and byte bounds; the pinned engine accepts
    /// every input that passes these checks.
    pub fn new<I, S>(patterns: I) -> Result<Self, DfaError>
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let patterns: Vec<String> = patterns.into_iter().map(Into::into).collect();
        if patterns.is_empty() {
            return Err(DfaError::EmptyPatterns);
        }
        if patterns.iter().any(String::is_empty) {
            return Err(DfaError::EmptyPattern);
        }
        if patterns.len() > MAX_PATTERN_COUNT {
            return Err(DfaError::PatternSetTooLarge {
                actual: patterns.len(),
                maximum: MAX_PATTERN_COUNT,
            });
        }
        let pattern_bytes = patterns.iter().map(String::len).sum::<usize>();
        if pattern_bytes > MAX_PATTERN_BYTES {
            return Err(DfaError::PatternSetTooLarge {
                actual: pattern_bytes,
                maximum: MAX_PATTERN_BYTES,
            });
        }
        let automaton = AhoCorasickBuilder::new()
            .match_kind(MatchKind::LeftmostLongest)
            .build(&patterns)
            .expect("bounded non-empty patterns fit Aho-Corasick identifiers");
        Ok(Self {
            automaton,
            patterns,
        })
    }

    /// Returns whether any pattern occurs in the input.
    #[must_use]
    pub fn is_match(&self, input: &str) -> bool {
        self.automaton.is_match(input)
    }

    /// Finds non-overlapping leftmost-longest matches.
    #[must_use]
    pub fn find_all(&self, input: &str) -> Vec<PatternMatch> {
        self.automaton
            .find_iter(input)
            .map(|found| {
                let pattern_index = found.pattern().as_usize();
                PatternMatch {
                    pattern_index,
                    start: found.start(),
                    end: found.end(),
                    pattern: self.patterns[pattern_index].clone(),
                }
            })
            .collect()
    }

    /// Replaces each match with the supplied marker.
    #[must_use]
    pub fn replace_all(&self, input: &str, replacement: &str) -> String {
        let replacements = vec![replacement; self.patterns.len()];
        self.automaton.replace_all(input, &replacements)
    }
}

const MAX_PATTERN_BYTES: usize = 16 * 1024 * 1024;

const MAX_PATTERN_COUNT: usize = 100_000;
