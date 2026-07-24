//! 对齐: `cn.hutool.core.util.ClassLoaderUtil`
//! 来源: hutool-core/src/main/java/cn/hutool/core/util/ClassLoaderUtil.java
//!
//! Rust 侧对齐 Java 类名规范化（内部类 `.` → `$`）与 `Class.getName()` 语义。

use crate::{CoreError, Result};

mod loaded_class;
mod class_loader_util;

pub use loaded_class::LoadedClass;
pub use class_loader_util::ClassLoaderUtil;
