//! Mutable Hutool-compatible word trie.

use crate::StopChar;
use std::{collections::HashMap, fmt, sync::Arc};

/// Controls dense, greedy, and bounded matching.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct MatchOptions {
    /// Maximum number of matches; `None` is unlimited.
    pub limit: Option<usize>,
    /// Starts another search at every accepted character.
    pub density: bool,
    /// Continues after the first terminal at one start position.
    pub greedy: bool,
}

use super::{CharFilter, Node};
