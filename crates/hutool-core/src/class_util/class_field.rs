//! 对齐: `cn.hutool.core.util.ClassUtil`
//! 来源: hutool-core/src/main/java/cn/hutool/core/util/ClassUtil.java
//!
//! Rust 无 JVM 反射；此处用 `std::any::type_name` 与测试元数据注册表对齐 Hutool 语义。

use std::path::PathBuf;
use std::sync::LazyLock;

use crate::text::str_splitter::StrSplitter;
use crate::Result;

/// 对齐 Java `java.lang.reflect.Field` 的轻量描述。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClassField {
    /// 字段名。
    pub name: String,
}
