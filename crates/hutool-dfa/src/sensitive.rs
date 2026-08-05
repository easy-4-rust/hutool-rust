//! 对齐: `cn.hutool.dfa.SensitiveUtil`
//! 来源: hutool-dfa/src/main/java/cn/hutool/dfa/SensitiveUtil.java
//!
//! 敏感词过滤工具，支持线程安全的敏感词查找、替换和序列化。

use crate::{DfaError, FoundWord, MatchOptions, WordTree};
use parking_lot::RwLock;
use serde::{Serialize, de::DeserializeOwned};
use std::{collections::BTreeMap, sync::Arc, thread::JoinHandle};

use crate::sensitive_processor::{DefaultSensitiveProcessor, SensitiveProcessor};
/// 敏感词过滤工具，支持线程安全的敏感词查找、替换和序列化。
///
/// 对齐 Java 类: `cn.hutool.dfa.SensitiveUtil`
#[derive(Debug, Default, Clone)]
pub struct SensitiveUtil {
    tree: Arc<RwLock<WordTree>>,
}

impl SensitiveUtil {
    /// 创建空的敏感词服务。
    ///
    /// 对齐 Java: `SensitiveUtil.SensitiveUtil()`
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// 创建并初始化敏感词服务。
    ///
    /// 对齐 Java: `SensitiveUtil.SensitiveUtil(Set<String>)`
    pub fn from_words<I, S>(words: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let service = Self::new();
        service.init(words);
        service
    }

    /// 返回是否已初始化敏感词。
    ///
    /// 对齐 Java: `SensitiveUtil.isInitialized()`
    #[must_use]
    pub fn is_initialized(&self) -> bool {
        !self.tree.read().is_empty()
    }

    /// 原子替换所有敏感词。
    ///
    /// 对齐 Java: `SensitiveUtil.init(Set<String>)`
    pub fn init<I, S>(&self, words: I)
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let mut tree = self.tree.write();
        tree.clear();
        tree.add_words(words);
    }

    /// Replaces all words on a background thread and returns its join handle.
    pub fn init_async(&self, words: Vec<String>) -> JoinHandle<()> {
        let service = self.clone();
        std::thread::spawn(move || service.init(words))
    }

    /// Parses and initializes separator-delimited words; blank input is ignored.
    pub fn init_from_str(&self, words: &str, separator: char) {
        if words.trim().is_empty() {
            return;
        }
        self.init(words.split(separator).map(str::trim));
    }

    /// Parses comma-delimited words.
    pub fn init_comma_separated(&self, words: &str) {
        self.init_from_str(words, ',');
    }

    /// Replaces the accepted-character predicate used by subsequent operations.
    pub fn set_char_filter<F>(&self, filter: F)
    where
        F: Fn(char) -> bool + Send + Sync + 'static,
    {
        self.tree.write().set_char_filter(filter);
    }

    /// 返回文本是否包含敏感词。
    ///
    /// 对齐 Java: `SensitiveUtil.containsSensitive(String)`
    #[must_use]
    pub fn contains_sensitive(&self, text: &str) -> bool {
        self.tree.read().is_match(text)
    }

    /// Serializes a value and searches its JSON representation.
    pub fn contains_serialized<T: Serialize>(&self, value: &T) -> Result<bool, DfaError> {
        Ok(self.contains_sensitive(&serde_json::to_string(value)?))
    }

    /// 返回第一个敏感词匹配。
    ///
    /// 对齐 Java: `SensitiveUtil.findFirstSensitive(String)`
    #[must_use]
    pub fn find_first_sensitive(&self, text: &str) -> Option<FoundWord> {
        self.tree.read().match_word(text)
    }

    /// Serializes a value and returns its first sensitive match.
    pub fn find_first_serialized<T: Serialize>(
        &self,
        value: &T,
    ) -> Result<Option<FoundWord>, DfaError> {
        Ok(self.find_first_sensitive(&serde_json::to_string(value)?))
    }

    /// 返回所有敏感词匹配。
    ///
    /// 对齐 Java: `SensitiveUtil.findAllSensitive(String)`
    #[must_use]
    pub fn find_all_sensitive(&self, text: &str) -> Vec<FoundWord> {
        self.tree.read().match_all_words(text)
    }

    /// Returns sensitive matches with explicit options.
    #[must_use]
    pub fn find_all_sensitive_with_options(
        &self,
        text: &str,
        options: MatchOptions,
    ) -> Vec<FoundWord> {
        self.tree.read().match_all_words_with_options(text, options)
    }

    /// Serializes a value and returns its sensitive matches.
    pub fn find_all_serialized<T: Serialize>(
        &self,
        value: &T,
        options: MatchOptions,
    ) -> Result<Vec<FoundWord>, DfaError> {
        Ok(self.find_all_sensitive_with_options(&serde_json::to_string(value)?, options))
    }

    /// 用星号替换敏感词（贪婪模式）。
    ///
    /// 对齐 Java: `SensitiveUtil.filterSensitive(String)`
    #[must_use]
    pub fn filter_sensitive(&self, text: &str) -> String {
        self.filter_sensitive_with(text, true, &DefaultSensitiveProcessor)
    }

    /// 用自定义处理器替换敏感词。
    ///
    /// 对齐 Java: `SensitiveUtil.filterSensitive(String, SensitiveProcessor)`
    #[must_use]
    pub fn filter_sensitive_with(
        &self,
        text: &str,
        greedy: bool,
        processor: &dyn SensitiveProcessor,
    ) -> String {
        if text.is_empty() {
            return String::new();
        }
        let matches = self.find_all_sensitive_with_options(
            text,
            MatchOptions {
                limit: None,
                density: true,
                greedy,
            },
        );
        if matches.is_empty() {
            return text.to_owned();
        }
        let by_start: BTreeMap<usize, &FoundWord> =
            matches.iter().map(|found| (found.start(), found)).collect();
        let mut output = String::with_capacity(text.len());
        let mut cursor = 0;
        while cursor < text.len() {
            if let Some(found) = by_start.get(&cursor) {
                output.push_str(&processor.process(found));
                cursor = found.end();
            } else {
                let character = text[cursor..].chars().next().unwrap_or_default();
                output.push(character);
                cursor += character.len_utf8();
            }
        }
        output
    }

    /// Filters a value's JSON representation and deserializes the result.
    pub fn filter_serialized<T>(
        &self,
        value: &T,
        greedy: bool,
        processor: &dyn SensitiveProcessor,
    ) -> Result<T, DfaError>
    where
        T: Serialize + DeserializeOwned,
    {
        let json = serde_json::to_string(value)?;
        Ok(serde_json::from_str(
            &self.filter_sensitive_with(&json, greedy, processor),
        )?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Deserializer, Serializer};

    #[derive(Debug, PartialEq, Eq)]
    struct Payload {
        text: String,
        fail_serialization: bool,
    }

    impl Serialize for Payload {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            if self.fail_serialization {
                return Err(serde::ser::Error::custom("injected serialization failure"));
            }
            serializer.serialize_str(&self.text)
        }
    }

    impl<'de> Deserialize<'de> for Payload {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            Ok(Self {
                text: String::deserialize(deserializer)?,
                fail_serialization: false,
            })
        }
    }

    #[test]
    fn initialization_search_and_async_replacement_are_thread_safe() {
        let service = SensitiveUtil::new();
        assert!(!service.is_initialized());
        service.init_from_str("   ", ',');
        assert!(!service.is_initialized());
        service.init_comma_separated("敏感词, bad");
        assert!(service.is_initialized());
        assert!(service.contains_sensitive("有敏 感词"));
        assert_eq!(
            service.find_first_sensitive("bad 和敏感词").unwrap().word(),
            "bad"
        );
        assert_eq!(service.find_all_sensitive("bad 敏感词").len(), 2);

        let handle = service.init_async(vec!["new".to_owned()]);
        handle.join().unwrap();
        assert!(!service.contains_sensitive("bad"));
        assert!(service.contains_sensitive("new"));

        service.set_char_filter(|character| character != '-');
        service.init(["a-b"]);
        assert!(service.contains_sensitive("a-b"));
    }

    #[test]
    fn filtering_supports_default_custom_and_serialized_processors() {
        let service = SensitiveUtil::from_words(["bad", "敏感词"]);
        assert_eq!(service.filter_sensitive("bad 敏 感词"), "*** ****");
        assert_eq!(service.filter_sensitive(""), "");
        assert_eq!(service.filter_sensitive("clean"), "clean");

        let custom = |found: &FoundWord| format!("[{}]", found.word());
        assert_eq!(
            service.filter_sensitive_with("bad", false, &custom),
            "[bad]"
        );

        let payload = Payload {
            text: "bad".to_owned(),
            fail_serialization: false,
        };
        assert!(service.contains_serialized(&payload).unwrap());
        assert_eq!(
            service
                .find_first_serialized(&payload)
                .unwrap()
                .unwrap()
                .word(),
            "bad"
        );
        assert_eq!(
            service
                .find_all_serialized(&payload, MatchOptions::default())
                .unwrap()
                .len(),
            1
        );
        let filtered = service
            .filter_serialized(&payload, true, &|_: &FoundWord| "good".to_owned())
            .unwrap();
        assert_eq!(
            filtered,
            Payload {
                text: "good".to_owned(),
                fail_serialization: false,
            }
        );
    }

    struct Replacement {
        invalid_json: bool,
    }

    impl SensitiveProcessor for Replacement {
        fn process(&self, _found_word: &FoundWord) -> String {
            if self.invalid_json {
                "\"".to_owned()
            } else {
                "good".to_owned()
            }
        }
    }

    #[test]
    fn serialized_helpers_propagate_json_errors() {
        let service = SensitiveUtil::from_words(["bad"]);
        assert!(serde_json::from_str::<Payload>("123").is_err());
        let broken = Payload {
            text: "bad".to_owned(),
            fail_serialization: true,
        };
        assert!(service.contains_serialized(&broken).is_err());
        assert!(service.find_first_serialized(&broken).is_err());
        assert!(
            service
                .find_all_serialized(&broken, MatchOptions::default())
                .is_err()
        );
        assert!(
            service
                .filter_serialized(
                    &broken,
                    true,
                    &Replacement {
                        invalid_json: false
                    }
                )
                .is_err()
        );

        let payload = Payload {
            text: "bad".to_owned(),
            fail_serialization: false,
        };
        assert!(
            service
                .filter_serialized(&payload, true, &Replacement { invalid_json: true })
                .is_err()
        );
        assert_eq!(
            service
                .filter_serialized(
                    &payload,
                    true,
                    &Replacement {
                        invalid_json: false
                    }
                )
                .unwrap()
                .text,
            "good"
        );
    }
}
