//! 被注解元素注册表，对齐 Java `AnnotatedElement` 及其层级结构。

use std::sync::Arc;

use crate::mirror::AnnotationMirror;

pub use crate::mirror::ElementHandle;

/// 字段元素。
#[derive(Debug, Clone)]
pub struct FieldElement {
    /// 元素句柄。
    pub handle: ElementHandle,
    /// 字段名。
    pub name: String,
    /// 声明类型句柄。
    pub declaring_type: ElementHandle,
    /// 字段上的注解。
    pub annotations: Vec<Arc<AnnotationMirror>>,
}
