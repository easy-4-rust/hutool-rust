//! 验证码生成器子模块声明。
//!
//! 对齐: `cn.hutool.captcha.generator`
//! 来源: `hutool-captcha/src/main/java/cn/hutool/captcha/generator/`
//! 中文说明: 按 Java 包结构拆分 Hutool 验证码生成器相关对象，仅负责模块声明与导出。

mod abstract_generator;
mod math_generator;
mod random_generator;

pub use abstract_generator::AbstractGenerator;
pub use math_generator::MathGenerator;
pub use random_generator::RandomGenerator;
