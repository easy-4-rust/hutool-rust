//! `MathGenerator` 对象的独立模块。
//!
//! 对齐: `cn.hutool.captcha.generator.MathGenerator`
//! 来源: `hutool-captcha/src/main/java/cn/hutool/captcha/generator/MathGenerator.java`
//! 中文说明: Hutool 算术验证码生成器，负责构造加减乘表达式并校验用户输入结果。

use rand::RngExt;

use crate::{CaptchaError, CodeGenerator};

/// Hutool `MathGenerator` 的 Rust 对应实现。
///
/// 支持加法、减法和乘法表达式，并可选择是否允许负数结果。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MathGenerator {
    number_length: u8,
    allow_negative: bool,
}

impl MathGenerator {
    /// 创建算术验证码生成器。
    pub fn new(number_length: u8, allow_negative: bool) -> Result<Self, CaptchaError> {
        if number_length == 0 || number_length > 8 {
            return Err(CaptchaError::InvalidLength);
        }
        Ok(Self {
            number_length,
            allow_negative,
        })
    }

    /// 使用 Hutool 默认操作数宽度创建生成器。
    #[must_use]
    pub const fn with_negative_results(allow_negative: bool) -> Self {
        Self {
            number_length: 2,
            allow_negative,
        }
    }

    /// 返回生成表达式的固定长度。
    #[must_use]
    pub const fn length(&self) -> usize {
        self.number_length as usize * 2 + 2
    }

    /// 计算由 [`Self::generate`] 生成的表达式结果。
    ///
    /// 对齐 Java `Calculator.conversion(code)` 的校验入口。
    #[must_use]
    pub fn evaluate(code: &str) -> Option<i64> {
        let expression = code.strip_suffix('=')?;
        for operator in ['+', '-', '*'] {
            if let Some((left, right)) = expression.split_once(operator) {
                let left = left.trim().parse::<i64>().ok()?;
                let right = right.trim().parse::<i64>().ok()?;
                return if operator == '+' {
                    left.checked_add(right)
                } else if operator == '-' {
                    left.checked_sub(right)
                } else {
                    left.checked_mul(right)
                };
            }
        }
        None
    }
}

impl Default for MathGenerator {
    fn default() -> Self {
        Self::with_negative_results(true)
    }
}

impl CodeGenerator for MathGenerator {
    fn generate(&self) -> String {
        let limit = 10_i64.pow(u32::from(self.number_length));
        let mut rng = rand::rng();
        let left = rng.random_range(0..limit);
        let operator = ['+', '-', '*'][rng.random_range(0..3)];
        let right = if !self.allow_negative && operator == '-' {
            rng.random_range(0..=left)
        } else {
            rng.random_range(0..limit)
        };
        format!(
            "{left:<width$}{operator}{right:<width$}=",
            width = usize::from(self.number_length)
        )
    }

    fn verify(&self, generated: &str, input: &str) -> bool {
        input.trim().parse::<i64>().ok() == Self::evaluate(generated)
    }
}
