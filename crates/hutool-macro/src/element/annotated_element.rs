//! 被注解元素注册表，对齐 Java `AnnotatedElement` 及其层级结构。

use std::sync::Arc;

use crate::mirror::AnnotationMirror;

pub use crate::mirror::ElementHandle;

use super::element_kind::ElementKind;
use super::field_element::FieldElement;
use super::method_element::MethodElement;
use super::type_element::TypeElement;

/// 被注解元素枚举。
#[derive(Debug, Clone)]
pub enum AnnotatedElement {
    /// 类型元素。
    Type(TypeElement),
    /// 方法元素。
    Method(MethodElement),
    /// 字段元素。
    Field(FieldElement),
}

impl AnnotatedElement {
    /// 元素句柄。
    pub fn handle(&self) -> ElementHandle {
        match self {
            Self::Type(e) => e.handle,
            Self::Method(e) => e.handle,
            Self::Field(e) => e.handle,
        }
    }

    /// 直接声明的注解。
    pub fn declared_annotations(&self) -> &[Arc<AnnotationMirror>] {
        match self {
            Self::Type(e) => &e.annotations,
            Self::Method(e) => &e.annotations,
            Self::Field(e) => &e.annotations,
        }
    }

    /// 元素种类。
    pub fn kind(&self) -> ElementKind {
        match self {
            Self::Type(_) => ElementKind::Type,
            Self::Method(_) => ElementKind::Method,
            Self::Field(_) => ElementKind::Field,
        }
    }
}
