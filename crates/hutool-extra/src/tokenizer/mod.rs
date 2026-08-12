//! 中文分词 facade，对齐 hutool 的 `cn.hutool.extra.tokenizer.*`。
//!
//! - `TokenizerEngine`/`TokenizerResult`/`Word` trait（对齐 Java 分词抽象）
//! - `JiebaEngine`：基于 [`jieba-rs`] 的默认引擎（feature `tokenizer`）
//! - `TokenizerUtil` 静态门面

mod abstract_result;
mod tokenizer_engine;
mod tokenizer_result;
mod tokenizer_util;
mod word;

#[cfg(feature = "tokenizer")]
mod jieba_engine;

pub use abstract_result::AbstractResult;
pub use tokenizer_engine::TokenizerEngine;
pub use tokenizer_result::TokenizerResult;
pub use tokenizer_util::TokenizerUtil;
pub use word::Word;

#[cfg(feature = "tokenizer")]
pub use jieba_engine::JiebaEngine;
