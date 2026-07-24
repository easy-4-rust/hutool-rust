//! 结构化注解模型核心类型，对齐 Java `Annotation` / `AnnotatedElement` 语义。
//!
//! Rust 无 JVM 运行时注解，通过 [`AnnotationMirror`] + [`ElementHandle`] 表达注解实例与被注解元素。

use std::collections::HashMap;
use std::fmt;
use std::sync::Arc;

/// 被注解元素句柄，对齐 Java `AnnotatedElement` 身份。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ElementHandle(u64);

impl ElementHandle {
    /// 构造元素句柄。
    pub const fn new(id: u64) -> Self {
        Self(id)
    }

    /// 内部 id。
    pub const fn id(self) -> u64 {
        self.0
    }
}
