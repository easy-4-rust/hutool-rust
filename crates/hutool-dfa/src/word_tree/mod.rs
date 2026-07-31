//! 对齐: `cn.hutool.dfa.WordTree`
//! 来源: hutool-dfa/src/main/java/cn/hutool/dfa/WordTree.java
//!
//! Hutool 兼容的可变字典树。

use std::collections::HashMap;

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
