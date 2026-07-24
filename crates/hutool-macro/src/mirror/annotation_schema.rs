//! 结构化注解模型核心类型，对齐 Java `Annotation` / `AnnotatedElement` 语义。
//!
//! Rust 无 JVM 运行时注解，通过 [`AnnotationMirror`] + [`ElementHandle`] 表达注解实例与被注解元素。

use std::collections::HashMap;
use std::fmt;
use std::sync::Arc;

use super::annotation_mirror::AnnotationMirror;
use super::annotation_type_name::AnnotationTypeName;
use super::attribute_def::AttributeDef;

/// 注解类型 schema。
#[derive(Debug, Clone)]
pub struct AnnotationSchema {
    pub type_name: AnnotationTypeName,
    pub attributes: Vec<AttributeDef>,
    pub meta: Vec<Arc<AnnotationMirror>>,
    pub inherited: bool,
}

impl AnnotationSchema {
    /// 查找属性定义。
    pub fn attribute(&self, name: &str) -> Option<&AttributeDef> {
        self.attributes.iter().find(|a| a.name == name)
    }
}
