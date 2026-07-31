//! 结构化注解模型核心类型，对齐 Java `Annotation` / `AnnotatedElement` 语义。
//!
//! Rust 无 JVM 运行时注解，通过 [`AnnotationMirror`] + [`ElementHandle`] 表达注解实例与被注解元素。

use std::sync::Arc;

use super::annotation_mirror::AnnotationMirror;
use super::annotation_type_name::AnnotationTypeName;
use super::attribute_def::AttributeDef;

/// 注解类型 schema。
#[derive(Debug, Clone)]
pub struct AnnotationSchema {
    /// 注解类型名。
    pub type_name: AnnotationTypeName,
    /// 属性定义列表。
    pub attributes: Vec<AttributeDef>,
    /// 元注解镜像列表。
    pub meta: Vec<Arc<AnnotationMirror>>,
    /// 是否随继承传播。
    pub inherited: bool,
}

impl AnnotationSchema {
    /// 查找属性定义。
    pub fn attribute(&self, name: &str) -> Option<&AttributeDef> {
        self.attributes.iter().find(|a| a.name == name)
    }
}
