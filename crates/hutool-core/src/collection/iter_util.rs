//! 对齐: `cn.hutool.core.collection.IterUtil`
//! 来源: hutool-core/src/main/java/cn/hutool/core/collection/IterUtil.java
//!
//! 中文说明: Java 包镜像入口，委托至 idiomatic 实现 `crate::IterUtil`
//! （`src/iter_util.rs`）。保留 `crate::collection::IterUtil` 路径，
//! 1:1 镜像 Java 包结构。

#![allow(unused_imports)] // 包路径镜像再导出，保留 crate::collection::IterUtil 路径

pub use crate::IterUtil;
