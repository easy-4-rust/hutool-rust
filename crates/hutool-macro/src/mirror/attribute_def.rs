//! 结构化注解模型核心类型，对齐 Java `Annotation` / `AnnotatedElement` 语义。
//!
//! Rust 无 JVM 运行时注解，通过 [`AnnotationMirror`] + [`ElementHandle`] 表达注解实例与被注解元素。

use std::collections::HashMap;
use std::fmt;
use std::sync::Arc;

use super::annotation_mirror::AnnotationMirror;
use super::annotation_value::AnnotationValue;
use super::value_kind::ValueKind;

/// 属性定义，对齐 Java 注解接口中的 attribute method。
#[derive(Debug, Clone)]
pub struct AttributeDef {
    pub name: &'static str,
    pub value_kind: ValueKind,
    pub default_value: AnnotationValue,
    /// 属性方法上的元注解（如 `@Alias`、`@MirrorFor`）。
    pub meta: Vec<Arc<AnnotationMirror>>,
}

impl AttributeDef {
    /// 创建字符串属性定义。
    pub fn string(name: &'static str, default: &str) -> Self {
        Self {
            name,
            value_kind: ValueKind::String,
            default_value: AnnotationValue::String(default.to_string()),
            meta: Vec::new(),
        }
    }

    /// 创建 Class 类型属性定义。
    pub fn class_type(name: &'static str, default: &str) -> Self {
        Self {
            name,
            value_kind: ValueKind::Class,
            default_value: AnnotationValue::Class(default.to_string()),
            meta: Vec::new(),
        }
    }

    /// 附加属性元注解。
    pub fn with_meta(mut self, meta: Arc<AnnotationMirror>) -> Self {
        self.meta.push(meta);
        self
    }
}
