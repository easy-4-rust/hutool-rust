//! Mutable Hutool-compatible word trie.

use crate::StopChar;
use std::{fmt, sync::Arc};

use super::found_word::FoundWord;
use super::match_options::MatchOptions;

/// 可变字典树，支持 Hutool 停止字符匹配语义。
///
/// 对齐 Java 类: `cn.hutool.dfa.WordTree`
#[derive(Clone)]
pub struct WordTree {
    root: Node,
    char_filter: Arc<CharFilter>,
    word_count: usize,
}

impl fmt::Debug for WordTree {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("WordTree")
            .field("word_count", &self.word_count)
            .finish_non_exhaustive()
    }
}

impl Default for WordTree {
    fn default() -> Self {
        Self {
            root: Node::default(),
            char_filter: Arc::new(StopChar::is_not_stop_char),
            word_count: 0,
        }
    }
}

impl WordTree {
    /// Creates an empty word tree.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a tree and inserts all words.
    pub fn from_words<I, S>(words: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let mut tree = Self::new();
        tree.add_words(words);
        tree
    }

    /// Replaces the accepted-character predicate.
    pub fn set_char_filter<F>(&mut self, filter: F) -> &mut Self
    where
        F: Fn(char) -> bool + Send + Sync + 'static,
    {
        self.char_filter = Arc::new(filter);
        self
    }

    /// Inserts distinct words.
    pub fn add_words<I, S>(&mut self, words: I) -> &mut Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        for word in words {
            self.add_word(word.as_ref());
        }
        self
    }

    /// Inserts one word after applying the current character filter.
    pub fn add_word(&mut self, word: &str) -> &mut Self {
        let accepted: Vec<char> = word
            .chars()
            .filter(|character| (self.char_filter)(*character))
            .collect();
        if accepted.is_empty() {
            return self;
        }
        let mut current = &mut self.root;
        for character in accepted {
            current = current.children.entry(character).or_default();
        }
        if !current.terminal {
            current.terminal = true;
            self.word_count += 1;
        }
        self
    }

    /// Returns whether the tree has no effective words.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.word_count == 0
    }

    /// Returns the number of distinct effective words.
    #[must_use]
    pub const fn len(&self) -> usize {
        self.word_count
    }

    /// Removes every word without changing the character filter.
    pub fn clear(&mut self) {
        self.root = Node::default();
        self.word_count = 0;
    }

    /// Returns whether any word matches.
    #[must_use]
    pub fn is_match(&self, text: &str) -> bool {
        self.match_word(text).is_some()
    }

    /// Returns the exact text of the first match.
    #[must_use]
    pub fn match_first(&self, text: &str) -> Option<String> {
        self.match_word(text).map(|found| found.source_match)
    }

    /// Returns the first detailed match.
    #[must_use]
    pub fn match_word(&self, text: &str) -> Option<FoundWord> {
        self.match_all_words_with_options(
            text,
            MatchOptions {
                limit: Some(1),
                ..MatchOptions::default()
            },
        )
        .into_iter()
        .next()
    }

    /// Returns all default non-dense, shortest matches as strings.
    #[must_use]
    pub fn match_all(&self, text: &str) -> Vec<String> {
        self.match_all_with_options(text, MatchOptions::default())
    }

    /// Returns all default non-dense, shortest detailed matches.
    #[must_use]
    pub fn match_all_words(&self, text: &str) -> Vec<FoundWord> {
        self.match_all_words_with_options(text, MatchOptions::default())
    }

    /// Returns at most `limit` default matches as strings.
    #[must_use]
    pub fn match_all_limit(&self, text: &str, limit: usize) -> Vec<String> {
        self.match_all_with_options(
            text,
            MatchOptions {
                limit: Some(limit),
                ..MatchOptions::default()
            },
        )
    }

    /// Returns matches as exact source strings with explicit options.
    #[must_use]
    pub fn match_all_with_options(&self, text: &str, options: MatchOptions) -> Vec<String> {
        self.match_all_words_with_options(text, options)
            .into_iter()
            .map(|found| found.source_match)
            .collect()
    }

    /// Returns detailed matches with explicit density, greed, and limit.
    #[must_use]
    pub fn match_all_words_with_options(
        &self,
        text: &str,
        options: MatchOptions,
    ) -> Vec<FoundWord> {
        if options.limit == Some(0) || text.is_empty() || self.is_empty() {
            return Vec::new();
        }
        let characters: Vec<(usize, char)> = text.char_indices().collect();
        let mut found = Vec::new();
        let mut start_position = 0;
        while start_position < characters.len() {
            if !(self.char_filter)(characters[start_position].1) {
                start_position += 1;
                continue;
            }
            let start_byte = characters[start_position].0;
            let mut current = &self.root;
            let mut effective_word = String::new();
            let mut next_start = start_position + 1;
            for (position, &(byte, character)) in characters.iter().enumerate().skip(start_position)
            {
                if !(self.char_filter)(character) {
                    continue;
                }
                let Some(child) = current.children.get(&character) else {
                    break;
                };
                effective_word.push(character);
                current = child;
                if !current.terminal {
                    continue;
                }
                let end = byte + character.len_utf8();
                found.push(FoundWord::new(
                    effective_word.clone(),
                    &text[start_byte..end],
                    start_byte,
                    end,
                ));
                if options.limit.is_some_and(|limit| found.len() >= limit) {
                    return found;
                }
                if !options.density {
                    next_start = position + 1;
                    break;
                }
                if !options.greedy {
                    break;
                }
            }
            start_position = next_start;
        }
        found
    }
}

use super::{CharFilter, Node};
