//! 被注解元素注册表，对齐 Java `AnnotatedElement` 及其层级结构。

use std::collections::HashMap;
use std::sync::Arc;

use parking_lot::RwLock;

use crate::mirror::{AnnotationMirror, AnnotationSchema, AnnotationTypeName, AnnotationValue};

pub use crate::mirror::ElementHandle;

/// 类型元素。
#[derive(Debug, Clone)]
pub struct TypeElement {
    pub handle: ElementHandle,
    pub name: String,
    pub annotations: Vec<Arc<AnnotationMirror>>,
    pub super_type: Option<ElementHandle>,
    pub interfaces: Vec<ElementHandle>,
    pub methods: HashMap<String, ElementHandle>,
    pub fields: HashMap<String, ElementHandle>,
}

use super::{GLOBAL_REGISTRY, global_registry};
