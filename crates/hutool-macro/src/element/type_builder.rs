//! 被注解元素注册表，对齐 Java `AnnotatedElement` 及其层级结构。

use std::collections::HashMap;
use std::sync::Arc;

use crate::mirror::AnnotationMirror;

pub use crate::mirror::ElementHandle;

use super::annotation_registry::AnnotationRegistry;
use super::type_element::TypeElement;

/// 类型 builder。
pub struct TypeBuilder<'a> {
    registry: &'a mut AnnotationRegistry,
    handle: ElementHandle,
    name: String,
    annotations: Vec<Arc<AnnotationMirror>>,
    super_type: Option<ElementHandle>,
    interfaces: Vec<ElementHandle>,
}

impl<'a> TypeBuilder<'a> {
    /// 开始构建类型。
    pub fn begin(registry: &'a mut AnnotationRegistry, name: impl Into<String>) -> Self {
        let handle = registry.next_handle();
        Self {
            registry,
            handle,
            name: name.into(),
            annotations: Vec::new(),
            super_type: None,
            interfaces: Vec::new(),
        }
    }

    /// 添加注解。
    pub fn annotate(mut self, annotation: Arc<AnnotationMirror>) -> Self {
        self.annotations.push(annotation);
        self
    }

    /// 设置父类。
    pub fn super_type(mut self, handle: ElementHandle) -> Self {
        self.super_type = Some(handle);
        self
    }

    /// 添加父接口。
    pub fn interface(mut self, handle: ElementHandle) -> Self {
        self.interfaces.push(handle);
        self
    }

    /// 完成构建。
    pub fn build(self) -> ElementHandle {
        let element = TypeElement {
            handle: self.handle,
            name: self.name,
            annotations: self.annotations,
            super_type: self.super_type,
            interfaces: self.interfaces,
            methods: HashMap::new(),
            fields: HashMap::new(),
        };
        self.registry.register_type(element)
    }
}
