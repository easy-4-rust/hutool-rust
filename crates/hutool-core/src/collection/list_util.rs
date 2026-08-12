//! 对齐: `cn.hutool.core.collection.ListUtil`
//! 来源: hutool-core/src/main/java/cn/hutool/core/collection/ListUtil.java
//!
//! 中文说明: Java 包镜像入口，委托至 idiomatic 实现 `crate::ListUtil`
//! （`src/list_util.rs`）。保留 `crate::collection::ListUtil` 路径，
//! 1:1 镜像 Java 包结构。

#![allow(unused_imports)] // 包路径镜像再导出，保留 crate::collection::ListUtil 路径

pub use crate::ListUtil;
