//! 中文分词 facade，对齐 hutool 的 `cn.hutool.extra.tokenizer.*`。
//!
//! 提供 TokenizerEngine trait + TokenizerResult 抽象 + Word trait + TokenizerUtil 静态门面。
//! 各具体分词引擎（Ansj/HanLP/IKAnalyzer/Jcseg/Jieba/MMSeg/MynNLP/Word/Analysis）
//! 需要外部 Java crate，属于 unsafe-to-copy，暂不在 hutool-extra 中实现。

use crate::HutoolException;

/// 词接口，对齐 `cn.hutool.extra.tokenizer.Word`。
pub trait Word: Send + Sync + std::fmt::Debug {
    /// 获取词文本
    fn get_text(&self) -> String;

    /// 获取起始位置
    fn get_start_offset(&self) -> i32;

    /// 获取结束位置
    fn get_end_offset(&self) -> i32;

    /// 转字符串
    fn to_string_repr(&self) -> String {
        self.get_text()
    }
}
