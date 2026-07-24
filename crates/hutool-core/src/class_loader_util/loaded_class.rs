//! 对齐: `cn.hutool.core.util.ClassLoaderUtil`
//! 来源: hutool-core/src/main/java/cn/hutool/core/util/ClassLoaderUtil.java
//!
//! Rust 侧对齐 Java 类名规范化（内部类 `.` → `$`）与 `Class.getName()` 语义。

use crate::{CoreError, Result};

/// 对齐 Java `Class.getName()` 的轻量返回值。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoadedClass {
    /// JVM 风格类名。
    pub name: String,
}
