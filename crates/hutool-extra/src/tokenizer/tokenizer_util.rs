//! 中文分词 facade，对齐 hutool 的 `cn.hutool.extra.tokenizer.TokenizerUtil`。

use crate::HutoolException;

use super::tokenizer_engine::TokenizerEngine;

/// 分词工具类，对齐 `cn.hutool.extra.tokenizer.TokenizerUtil`。
pub struct TokenizerUtil;

impl TokenizerUtil {
    /// 对齐 `TokenizerUtil.createEngine()`：根据默认配置创建分词引擎。
    ///
    /// Java 通过 SPI 加载首个引擎（默认 jieba-anjs）；Rust 侧启用 `tokenizer`
    /// feature 时返回内置 `JiebaEngine`，否则返回未启用错误。
    #[cfg(feature = "tokenizer")]
    pub fn create_engine() -> std::result::Result<Box<dyn TokenizerEngine>, HutoolException> {
        Ok(Box::new(super::JiebaEngine::new()))
    }

    #[cfg(not(feature = "tokenizer"))]
    pub fn create_engine() -> std::result::Result<Box<dyn TokenizerEngine>, HutoolException> {
        Err(HutoolException::Message(
            "tokenizer feature not enabled; add `hutool-extra/tokenizer` to create_engine".into(),
        ))
    }
}
