//! 中文分词 facade，对齐 hutool 的 `cn.hutool.extra.tokenizer.*`。
//!
//! 提供 TokenizerEngine trait + TokenizerResult 抽象 + Word trait + TokenizerUtil 静态门面。
//! 各具体分词引擎（Ansj/HanLP/IKAnalyzer/Jcseg/Jieba/MMSeg/MynNLP/Word/Analysis）
//! 需要外部 Java crate，属于 unsafe-to-copy，暂不在 hutool-extra 中实现。

use crate::HutoolException;

use super::word::Word;

/// 分词结果接口，对齐 `cn.hutool.extra.tokenizer.Result`。
///
/// 注意：模块内部使用 `TokenizerResult` 而非 `Result`，避免与 `std::result::Result` 冲突。
pub trait TokenizerResult: Send + Sync {
    /// 是否还有下一个词
    fn has_next(&self) -> bool;

    /// 获取下一个词
    fn next_word(&self) -> Option<Box<dyn Word>>;
}
