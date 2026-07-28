//! `RandomGenerator` 对象的独立模块。
//!
//! 对齐: `cn.hutool.captcha.generator.RandomGenerator`
//! 来源: `hutool-captcha/src/main/java/cn/hutool/captcha/generator/RandomGenerator.java`
//! 中文说明: Hutool 兼容的随机字符验证码生成器，复用 `AbstractGenerator` 的字符集和长度配置。

use rand::RngExt;

use crate::generator::AbstractGenerator;
use crate::{CaptchaError, CodeGenerator, constant_time_ascii_case_eq};

/// Hutool `RandomGenerator` 的 Rust 对应实现。
///
/// 使用给定字符集随机生成验证码，并按 Hutool 习惯忽略大小写校验。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RandomGenerator(AbstractGenerator);

impl RandomGenerator {
    /// 使用默认字母数字字符集创建随机生成器。
    pub fn new(length: usize) -> Result<Self, CaptchaError> {
        AbstractGenerator::new(length).map(Self)
    }

    /// 使用自定义 ASCII 字符集创建随机生成器。
    pub fn with_alphabet(alphabet: &[u8], length: usize) -> Result<Self, CaptchaError> {
        AbstractGenerator::with_alphabet(alphabet, length).map(Self)
    }

    /// 返回验证码长度。
    #[must_use]
    pub const fn length(&self) -> usize {
        self.0.length()
    }
}

impl CodeGenerator for RandomGenerator {
    fn generate(&self) -> String {
        let mut rng = rand::rng();
        (0..self.0.length)
            .map(|_| {
                let index = rng.random_range(0..self.0.alphabet.len());
                char::from(self.0.alphabet[index])
            })
            .collect()
    }

    fn verify(&self, generated: &str, input: &str) -> bool {
        !input.trim().is_empty()
            && constant_time_ascii_case_eq(generated.as_bytes(), input.trim().as_bytes())
    }
}

impl Default for RandomGenerator {
    fn default() -> Self {
        Self::new(5).expect("the default CAPTCHA length is valid")
    }
}
