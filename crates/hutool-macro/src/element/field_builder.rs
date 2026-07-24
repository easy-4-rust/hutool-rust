//! 被注解元素注册表，对齐 Java `AnnotatedElement` 及其层级结构。

use std::collections::HashMap;
use std::sync::Arc;

use parking_lot::RwLock;

use crate::mirror::{AnnotationMirror, AnnotationSchema, AnnotationTypeName, AnnotationValue};

pub use crate::mirror::ElementHandle;

use super::annotation_registry::AnnotationRegistry;
use super::field_element::FieldElement;

/// 字段 builder。
pub struct FieldBuilder<'a> {
    registry: &'a mut AnnotationRegistry,
    handle: ElementHandle,
    name: String,
    declaring_type: ElementHandle,
    annotations: Vec<Arc<AnnotationMirror>>,
}

impl<'a> FieldBuilder<'a> {
    /// 开始构建字段。
    pub fn begin(
        registry: &'a mut AnnotationRegistry,
        declaring_type: ElementHandle,
        name: impl Into<String>,
    ) -> Self {
        let handle = registry.next_handle();
        Self {
            registry,
            handle,
            name: name.into(),
            declaring_type,
            annotations: Vec::new(),
        }
    }

    /// 添加注解。
    pub fn annotate(mut self, annotation: Arc<AnnotationMirror>) -> Self {
        self.annotations.push(annotation);
        self
    }

    /// 完成构建。
    pub fn build(self) -> ElementHandle {
        let element = FieldElement {
            handle: self.handle,
            name: self.name.clone(),
            declaring_type: self.declaring_type,
            annotations: self.annotations,
        };
        let handle = self.registry.register_field(element);
        if let Some(ty) = self.registry.get_type_mut(self.declaring_type) {
            ty.fields.insert(self.name, handle);
        }
        handle
    }
}

use super::{GLOBAL_REGISTRY, global_registry};
