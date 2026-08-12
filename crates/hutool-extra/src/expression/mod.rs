//! 表达式引擎 facade，对齐 hutool 的 `cn.hutool.extra.expression.*`。
//!
//! - `ExpressionEngine` trait + `RhaiEngine` 默认实现（基于 [`rhai`]）
//! - `ExpressionUtil` 静态门面（默认引擎单例 + 注入覆盖）
//! - 其他引擎（Aviator/JEXL/SpEL）为 Java 特有，通过 trait 扩展

mod expression_engine;
mod expression_util;

pub use expression_engine::{ExpressionEngine, RhaiEngine};
pub use expression_util::ExpressionUtil;
