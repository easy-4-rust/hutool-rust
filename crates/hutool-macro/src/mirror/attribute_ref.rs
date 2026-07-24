//! 结构化注解模型核心类型，对齐 Java `Annotation` / `AnnotatedElement` 语义。
//!
//! Rust 无 JVM 运行时注解，通过 [`AnnotationMirror`] + [`ElementHandle`] 表达注解实例与被注解元素。

use std::collections::HashMap;
use std::fmt;
use std::sync::Arc;

use super::annotation_type_name::AnnotationTypeName;

/// 属性引用，对齐 Java `Method`（注解 attribute method）。
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AttributeRef {
    pub annotation_type: AnnotationTypeName,
    pub name: String,
}

impl AttributeRef {
    /// 创建属性引用。
    pub fn new(annotation_type: AnnotationTypeName, name: impl Into<String>) -> Self {
        Self {
            annotation_type,
            name: name.into(),
        }
    }
}
