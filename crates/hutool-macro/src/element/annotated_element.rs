//! 被注解元素注册表，对齐 Java `AnnotatedElement` 及其层级结构。

use std::collections::HashMap;
use std::sync::Arc;

use parking_lot::RwLock;

use crate::mirror::{AnnotationMirror, AnnotationSchema, AnnotationTypeName, AnnotationValue};

pub use crate::mirror::ElementHandle;

use super::element_kind::ElementKind;
use super::field_element::FieldElement;
use super::method_element::MethodElement;
use super::type_element::TypeElement;

/// 被注解元素枚举。
#[derive(Debug, Clone)]
pub enum AnnotatedElement {
    Type(TypeElement),
    Method(MethodElement),
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

use super::{GLOBAL_REGISTRY, global_registry};
