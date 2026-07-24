//! 中文分词 facade，对齐 hutool 的 `cn.hutool.extra.tokenizer.*`。
//!
//! 提供 TokenizerEngine trait + TokenizerResult 抽象 + Word trait + TokenizerUtil 静态门面。
//! 各具体分词引擎（Ansj/HanLP/IKAnalyzer/Jcseg/Jieba/MMSeg/MynNLP/Word/Analysis）
//! 需要外部 Java crate，属于 unsafe-to-copy，暂不在 hutool-extra 中实现。

use crate::HutoolException;

mod tokenizer_result;
mod abstract_result;
mod word;
mod tokenizer_engine;
mod tokenizer_util;

pub use tokenizer_result::TokenizerResult;
pub use abstract_result::AbstractResult;
pub use word::Word;
pub use tokenizer_engine::TokenizerEngine;
pub use tokenizer_util::TokenizerUtil;
