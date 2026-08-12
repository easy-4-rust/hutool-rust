//! jieba-rs 分词引擎，对齐 hutool 默认 jieba 引擎语义。
//!
//! 对齐 Java `cn.hutool.extra.tokenizer.engine.jieba.JiebaEngine`：
//! - `parse(text)` 返回带字符偏移的词序列
//! - `Word` 含 text/startOffset/endOffset
//!
//! jieba-rs 的 `Token` 是 char 偏移（非 byte），与 Java jieba-anjs 的偏移语义一致。

use crate::HutoolException;

use super::tokenizer_engine::TokenizerEngine;
use super::tokenizer_result::TokenizerResult;
use super::word::Word;

/// jieba-rs 引擎封装。
pub struct JiebaEngine {
    jieba: jieba_rs::Jieba,
}

impl JiebaEngine {
    /// 创建默认词典的引擎实例。
    #[must_use]
    pub fn new() -> Self {
        Self {
            jieba: jieba_rs::Jieba::new(),
        }
    }
}

impl Default for JiebaEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl TokenizerEngine for JiebaEngine {
    fn parse(&self, text: &str) -> std::result::Result<Box<dyn TokenizerResult>, HutoolException> {
        Ok(Box::new(JiebaResult::new(&self.jieba, text)))
    }
}

/// jieba 分词结果（迭代器式 `has_next`/`next_word`，对齐 Java `Result`）。
struct JiebaResult {
    words: Vec<JiebaWord>,
    cursor: std::sync::atomic::AtomicUsize,
}

impl JiebaResult {
    fn new(jieba: &jieba_rs::Jieba, text: &str) -> Self {
        // Search 模式对齐 Java jieba 的细粒度切分（召回更长复合词）
        let tokens = jieba.tokenize(text, jieba_rs::TokenizeMode::Search, true);
        let words: Vec<JiebaWord> = tokens
            .into_iter()
            .map(|token| {
                let chars: Vec<char> = text.chars().collect();
                let word_text: String = chars[token.start..token.end].iter().collect();
                JiebaWord {
                    text: word_text,
                    start: i32::try_from(token.start).unwrap_or_default(),
                    end: i32::try_from(token.end).unwrap_or_default(),
                }
            })
            .collect();
        Self {
            words,
            cursor: std::sync::atomic::AtomicUsize::new(0),
        }
    }
}

impl TokenizerResult for JiebaResult {
    fn has_next(&self) -> bool {
        self.cursor.load(std::sync::atomic::Ordering::Relaxed) < self.words.len()
    }

    fn next_word(&self) -> Option<Box<dyn Word>> {
        let idx = self
            .cursor
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        self.words
            .get(idx)
            .map(|w| Box::new(w.clone()) as Box<dyn Word>)
    }
}

/// jieba 词单元。
#[derive(Debug, Clone)]
struct JiebaWord {
    text: String,
    start: i32,
    end: i32,
}

impl Word for JiebaWord {
    fn get_text(&self) -> String {
        self.text.clone()
    }
    fn get_start_offset(&self) -> i32 {
        self.start
    }
    fn get_end_offset(&self) -> i32 {
        self.end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_chinese_sentence() {
        let engine = JiebaEngine::new();
        let result = engine.parse("我爱北京天安门").expect("parse");
        let mut texts = Vec::new();
        while result.has_next() {
            if let Some(word) = result.next_word() {
                texts.push(word.get_text());
            }
        }
        assert!(texts.iter().any(|t| t == "北京"), "应切出 北京: {texts:?}");
        assert!(texts.iter().any(|t| t == "我"), "应切出 我: {texts:?}");
    }

    #[test]
    fn word_offsets_are_char_indices() {
        let engine = JiebaEngine::new();
        let result = engine.parse("北京").expect("parse");
        // jieba 把 "北京" 整体识别为一个词（词典词）
        let word = result.next_word().expect("first word");
        assert_eq!(word.get_text(), "北京");
        assert_eq!(word.get_start_offset(), 0);
        assert_eq!(word.get_end_offset(), 2);
    }

    #[test]
    fn empty_text_returns_empty_result() {
        let engine = JiebaEngine::new();
        let result = engine.parse("").expect("parse");
        assert!(!result.has_next());
    }

    #[test]
    fn mixed_cn_en_tokenization() {
        let engine = JiebaEngine::new();
        let result = engine.parse("Java编程很有趣").expect("parse");
        let mut texts = Vec::new();
        while result.has_next() {
            if let Some(word) = result.next_word() {
                texts.push(word.get_text());
            }
        }
        assert!(!texts.is_empty());
        assert!(
            texts
                .iter()
                .any(|t| t.contains("编程") || t.contains("程序")),
            "应切出编程相关词: {texts:?}"
        );
    }
}
