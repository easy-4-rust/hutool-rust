//! 被注解元素注册表，对齐 Java `AnnotatedElement` 及其层级结构。

use std::sync::Arc;

use crate::mirror::AnnotationMirror;

pub use crate::mirror::ElementHandle;

use super::annotation_registry::AnnotationRegistry;
use super::method_element::MethodElement;

/// 方法 builder。
pub struct MethodBuilder<'a> {
    registry: &'a mut AnnotationRegistry,
    handle: ElementHandle,
    name: String,
    declaring_type: ElementHandle,
    annotations: Vec<Arc<AnnotationMirror>>,
}

impl<'a> MethodBuilder<'a> {
    /// 开始构建方法。
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

    /// 完成构建并挂到声明类。
    pub fn build(self) -> ElementHandle {
        let signature = format!("{}#{}", self.declaring_type.id(), self.name);
        let element = MethodElement {
            handle: self.handle,
            name: self.name.clone(),
            declaring_type: self.declaring_type,
            annotations: self.annotations,
            signature,
        };
        let handle = self.registry.register_method(element);
        if let Some(ty) = self.registry.get_type_mut(self.declaring_type) {
            ty.methods.insert(self.name, handle);
        }
        handle
    }
}
