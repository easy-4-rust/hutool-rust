//! Mutable Hutool-compatible word trie.

use crate::StopChar;
use std::{collections::HashMap, fmt, sync::Arc};

mod found_word;
mod match_options;
mod word_tree;

pub use found_word::FoundWord;
pub use match_options::MatchOptions;
pub use word_tree::WordTree;

type CharFilter = dyn Fn(char) -> bool + Send + Sync;

#[derive(Debug, Default, Clone)]
struct Node {
    children: HashMap<char, Node>,
    terminal: bool,
}
