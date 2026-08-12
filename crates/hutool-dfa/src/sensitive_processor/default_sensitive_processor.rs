//! 对齐: `cn.hutool.dfa.DefaultSensitiveProcessor` (Rust 独有)
//! 中文说明: 默认敏感词处理器，将匹配到的敏感词替换为等长的 * 号

use super::sensitive_processor::SensitiveProcessor;

/// Default asterisk-sensitive-word processor.
#[derive(Debug, Default, Clone, Copy)]
pub struct DefaultSensitiveProcessor;

impl SensitiveProcessor for DefaultSensitiveProcessor {}
