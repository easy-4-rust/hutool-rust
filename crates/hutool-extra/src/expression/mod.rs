//! 表达式引擎 facade，对齐 hutool 的 `cn.hutool.extra.expression.*`。
//!
//! 提供 ExpressionEngine trait + ExpressionUtil 静态门面 + 6 个构造器签名。
//! 各具体引擎（Aviator/JEXL/JfireEL/Mvel/QLExpress/Rhino/SpEL）需要外部 Java crate，
//! 属于 unsafe-to-copy，暂不在 hutool-extra 中实现。

use std::collections::HashMap;

use crate::HutoolException;

mod expression_engine;
mod expression_util;

pub use expression_engine::ExpressionEngine;
pub use expression_util::ExpressionUtil;
