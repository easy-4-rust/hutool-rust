//! 中文分词 facade，对齐 hutool 的 `cn.hutool.extra.tokenizer.*`。
//!
//! 提供 TokenizerEngine trait + TokenizerResult 抽象 + Word trait + TokenizerUtil 静态门面。
//! 各具体分词引擎（Ansj/HanLP/IKAnalyzer/Jcseg/Jieba/MMSeg/MynNLP/Word/Analysis）
//! 需要外部 Java crate，属于 unsafe-to-copy，暂不在 hutool-extra 中实现。

use crate::HutoolException;

use super::tokenizer_engine::TokenizerEngine;

/// 分词工具类，对齐 `cn.hutool.extra.tokenizer.TokenizerUtil`。
pub struct TokenizerUtil;

impl TokenizerUtil {
    /// 对齐 `TokenizerUtil.createEngine()`：根据默认配置创建分词引擎
    pub fn create_engine() -> std::result::Result<Box<dyn TokenizerEngine>, HutoolException> {
        Err(HutoolException::Message(
            "TokenizerUtil::create_engine requires a concrete TokenizerEngine implementation".into(),
        ))
    }
}
