//! `AbstractGenerator` 对象的独立模块。
//!
//! 对齐: `cn.hutool.captcha.generator.AbstractGenerator`
//! 来源: `hutool-captcha/src/main/java/cn/hutool/captcha/generator/AbstractGenerator.java`
//! 中文说明: Hutool 风格随机验证码生成器的抽象基类，负责维护基础字符集与验证码长度配置。

use crate::CaptchaError;

const HUTOOL_ALPHABET: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";

/// Hutool `AbstractGenerator` 的 Rust 对应实现。
///
/// 持有基础字符集合和验证码长度，供具体生成器复用。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AbstractGenerator {
    pub(crate) alphabet: Vec<u8>,
    pub(crate) length: usize,
}

impl AbstractGenerator {
    /// 使用 Hutool 默认字母数字字符集创建生成器配置。
    pub fn new(length: usize) -> Result<Self, CaptchaError> {
        Self::with_alphabet(HUTOOL_ALPHABET, length)
    }

    /// 使用调用方提供的 ASCII 字符集创建生成器配置。
    pub fn with_alphabet(alphabet: &[u8], length: usize) -> Result<Self, CaptchaError> {
        if length == 0 {
            return Err(CaptchaError::InvalidLength);
        }
        if alphabet.is_empty() || !alphabet.is_ascii() {
            return Err(CaptchaError::InvalidAlphabet);
        }
        Ok(Self {
            alphabet: alphabet.to_vec(),
            length,
        })
    }

    /// 返回验证码长度。
    #[must_use]
    pub const fn length(&self) -> usize {
        self.length
    }
}
