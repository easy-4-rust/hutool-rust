//! 被注解元素注册表，对齐 Java `AnnotatedElement` 及其层级结构。

use std::collections::HashMap;
use std::sync::Arc;

use crate::mirror::{AnnotationMirror, AnnotationSchema, AnnotationTypeName, AnnotationValue};

pub use crate::mirror::ElementHandle;

use super::annotated_element::AnnotatedElement;
use super::field_element::FieldElement;
use super::method_element::MethodElement;
use super::type_element::TypeElement;

/// 全局注解/元素注册表。
#[derive(Default)]
pub struct AnnotationRegistry {
    next_id: u64,
    schemas: HashMap<AnnotationTypeName, AnnotationSchema>,
    elements: HashMap<ElementHandle, AnnotatedElement>,
}

impl AnnotationRegistry {
    /// 创建空注册表。
    pub fn new() -> Self {
        Self::default()
    }

    /// 注册注解 schema。
    pub fn register_schema(&mut self, schema: AnnotationSchema) {
        self.schemas.insert(schema.type_name, schema);
    }

    /// 获取 schema。
    pub fn schema(&self, type_name: AnnotationTypeName) -> Option<&AnnotationSchema> {
        self.schemas.get(type_name)
    }

    pub(crate) fn next_handle(&mut self) -> ElementHandle {
        self.next_id += 1;
        ElementHandle::new(self.next_id)
    }

    /// 构造注解镜像。
    pub fn annotation(
        &self,
        type_name: AnnotationTypeName,
        values: HashMap<String, AnnotationValue>,
    ) -> Arc<AnnotationMirror> {
        Arc::new(AnnotationMirror::new(type_name, values))
    }

    /// 注册类型元素。
    pub fn register_type(&mut self, element: TypeElement) -> ElementHandle {
        let handle = element.handle;
        self.elements
            .insert(handle, AnnotatedElement::Type(element));
        handle
    }

    /// 注册方法元素。
    pub fn register_method(&mut self, element: MethodElement) -> ElementHandle {
        let handle = element.handle;
        self.elements
            .insert(handle, AnnotatedElement::Method(element));
        handle
    }

    /// 注册字段元素。
    pub fn register_field(&mut self, element: FieldElement) -> ElementHandle {
        let handle = element.handle;
        self.elements
            .insert(handle, AnnotatedElement::Field(element));
        handle
    }

    /// 获取元素。
    pub fn get(&self, handle: ElementHandle) -> Option<&AnnotatedElement> {
        self.elements.get(&handle)
    }

    /// 获取可变类型元素。
    pub fn get_type_mut(&mut self, handle: ElementHandle) -> Option<&mut TypeElement> {
        match self.elements.get_mut(&handle) {
            Some(AnnotatedElement::Type(t)) => Some(t),
            _ => None,
        }
    }

    /// 解析类型层级（广度优先）：返回 (distance, type_handle) 列表。
    pub fn type_hierarchy(&self, start: ElementHandle) -> Vec<(i32, ElementHandle)> {
        let mut result = Vec::new();
        let mut visited = HashMap::new();
        let mut queue = vec![(0i32, start)];

        while let Some((distance, handle)) = queue.first().cloned() {
            queue.remove(0);
            if visited.contains_key(&handle) {
                continue;
            }
            visited.insert(handle, distance);
            result.push((distance, handle));

            let Some(AnnotatedElement::Type(ty)) = self.get(handle) else {
                continue;
            };

            let mut next_level = Vec::new();
            if let Some(super_type) = ty.super_type {
                next_level.push(super_type);
            }
            next_level.extend(ty.interfaces.iter().copied());

            if !next_level.is_empty() {
                queue.push((distance + 1, next_level[0]));
                for item in next_level.into_iter().skip(1) {
                    if !visited.contains_key(&item) {
                        queue.push((distance + 1, item));
                    }
                }
            }
        }
        result
    }

    /// 解析类型名（测试注册表内必须为 static 名）。
    pub fn resolve_type_name(&self, name: &str) -> Option<AnnotationTypeName> {
        self.schemas.keys().find(|k| **k == name).copied()
    }

    /// 查找同名方法链（沿类型层级）。
    pub fn method_override_chain(&self, method: ElementHandle) -> Vec<ElementHandle> {
        let AnnotatedElement::Method(m) = self.get(method).cloned().expect("method") else {
            return vec![method];
        };
        let mut chain = vec![method];
        for (_, type_handle) in self.type_hierarchy(m.declaring_type).into_iter().skip(1) {
            if let Some(AnnotatedElement::Type(ty)) = self.get(type_handle) {
                if let Some(mh) = ty.methods.get(&m.name) {
                    chain.push(*mh);
                }
            }
        }
        chain
    }
}
