//! 中文分词 facade，对齐 hutool 的 `cn.hutool.extra.tokenizer.*`。
//!
//! 提供 TokenizerEngine trait + TokenizerResult 抽象 + Word trait + TokenizerUtil 静态门面。
//! 各具体分词引擎（Ansj/HanLP/IKAnalyzer/Jcseg/Jieba/MMSeg/MynNLP/Word/Analysis）
//! 需要外部 Java crate，属于 unsafe-to-copy，暂不在 hutool-extra 中实现。

use crate::HutoolException;

use super::tokenizer_result::TokenizerResult;

/// 分词引擎抽象，对齐 `cn.hutool.extra.tokenizer.TokenizerEngine`。
pub trait TokenizerEngine: Send + Sync {
    /// 对文本进行分词
    fn parse(&self, text: &str) -> std::result::Result<Box<dyn TokenizerResult>, HutoolException>;
}
