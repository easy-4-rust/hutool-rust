//! 被注解元素注册表，对齐 Java `AnnotatedElement` 及其层级结构。

use std::collections::HashMap;
use std::sync::Arc;

use crate::mirror::AnnotationMirror;

pub use crate::mirror::ElementHandle;

/// 类型元素。
#[derive(Debug, Clone)]
pub struct TypeElement {
    /// 元素句柄。
    pub handle: ElementHandle,
    /// 类型名。
    pub name: String,
    /// 类型上的注解。
    pub annotations: Vec<Arc<AnnotationMirror>>,
    /// 父类句柄。
    pub super_type: Option<ElementHandle>,
    /// 父接口句柄列表。
    pub interfaces: Vec<ElementHandle>,
    /// 方法表（方法名 -> 句柄）。
    pub methods: HashMap<String, ElementHandle>,
    /// 字段表（字段名 -> 句柄）。
    pub fields: HashMap<String, ElementHandle>,
}
