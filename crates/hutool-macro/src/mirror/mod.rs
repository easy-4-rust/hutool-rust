//! 结构化注解模型核心类型，对齐 Java `Annotation` / `AnnotatedElement` 语义。
//!
//! Rust 无 JVM 运行时注解，通过 [`AnnotationMirror`] + [`ElementHandle`] 表达注解实例与被注解元素。

use std::collections::HashMap;
use std::fmt;
use std::sync::Arc;

mod annotation_type_name;
mod value_kind;
mod annotation_value;
mod attribute_def;
mod annotation_schema;
mod annotation_mirror;
mod attribute_ref;
mod element_handle;

pub use annotation_type_name::AnnotationTypeName;
pub use value_kind::ValueKind;
pub use annotation_value::AnnotationValue;
pub use attribute_def::AttributeDef;
pub use annotation_schema::AnnotationSchema;
pub use annotation_mirror::AnnotationMirror;
pub use attribute_ref::AttributeRef;
pub use element_handle::ElementHandle;

pub fn is_jdk_meta_annotation(type_name: AnnotationTypeName) -> bool {
    matches!(
        type_name,
        "java.lang.annotation.Target"
            | "java.lang.annotation.Retention"
            | "java.lang.annotation.Inherited"
            | "java.lang.annotation.Documented"
            | "java.lang.SuppressWarnings"
            | "java.lang.Override"
            | "java.lang.Deprecated"
    )
}

pub fn is_not_jdk_meta_annotation(type_name: AnnotationTypeName) -> bool {
    !is_jdk_meta_annotation(type_name)
}

pub fn is_assignable(expected: ValueKind, actual: &AnnotationValue) -> bool {
    match (expected, actual.kind()) {
        (ValueKind::Void, ValueKind::Void) => true,
        (ValueKind::String, ValueKind::String) => true,
        (ValueKind::I32, ValueKind::I32) => true,
        (ValueKind::I64, ValueKind::I64) => true,
        (ValueKind::F64, ValueKind::F64) => true,
        (ValueKind::Bool, ValueKind::Bool) => true,
        (ValueKind::Class, ValueKind::Class) => true,
        (ValueKind::Array, ValueKind::Array) => true,
        (ValueKind::Annotation, ValueKind::Annotation) => true,
        (ValueKind::I32, ValueKind::I64) => true,
        _ => false,
    }
}
