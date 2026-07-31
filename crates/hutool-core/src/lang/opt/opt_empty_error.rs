//! 对齐: `cn.hutool.core.lang.Opt`
//! 来源: hutool-core/src/main/java/cn/hutool/core/lang/Opt.java
//!
//! Hutool `Opt` 的 idiomatic Rust 实现：在 [`Option`] 之上附加可选异常上下文。

/// 对齐 Java: `NoSuchElementException` 空 Opt 抛出。
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
#[error("No value present")]
#[allow(dead_code)] // 对齐 Java Opt 空值异常，暂未接线，预留
pub struct OptEmptyError;
