//! 结构化注解模型核心类型，对齐 Java `Annotation` / `AnnotatedElement` 语义。
//!
//! Rust 无 JVM 运行时注解，通过 [`AnnotationMirror`] + [`ElementHandle`] 表达注解实例与被注解元素。

use std::sync::Arc;

use super::annotation_mirror::AnnotationMirror;
use super::value_kind::ValueKind;

/// 注解属性值。
#[derive(Debug, Clone, PartialEq)]
pub enum AnnotationValue {
    /// 空值。
    Unit,
    /// 布尔值。
    Bool(bool),
    /// 32 位整数。
    I32(i32),
    /// 64 位整数。
    I64(i64),
    /// 浮点数。
    F64(f64),
    /// 字符串。
    String(String),
    /// Class 类型名。
    Class(String),
    /// 值数组。
    Array(Vec<AnnotationValue>),
    /// 嵌套注解。
    Annotation(Arc<AnnotationMirror>),
}

impl AnnotationValue {
    /// 返回值的类型种类。
    pub fn kind(&self) -> ValueKind {
        match self {
            Self::Unit => ValueKind::Void,
            Self::Bool(_) => ValueKind::Bool,
            Self::I32(_) => ValueKind::I32,
            Self::I64(_) => ValueKind::I64,
            Self::F64(_) => ValueKind::F64,
            Self::String(_) => ValueKind::String,
            Self::Class(_) => ValueKind::Class,
            Self::Array(_) => ValueKind::Array,
            Self::Annotation(_) => ValueKind::Annotation,
        }
    }

    /// 作为字符串取值；非字符串类型返回 `None`。
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Self::String(s) => Some(s),
            _ => None,
        }
    }

    /// 作为字符串数组取值。
    pub fn as_str_array(&self) -> Option<Vec<&str>> {
        match self {
            Self::Array(items) => {
                let mut out = Vec::with_capacity(items.len());
                for item in items {
                    out.push(item.as_str()?);
                }
                Some(out)
            }
            _ => None,
        }
    }
}
