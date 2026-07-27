//! Workspace-owned `AbstractTypeAnnotationScanner`.

use std::collections::{HashSet, VecDeque};

use hutool_macro::element::{AnnotatedElement, ElementHandle, global_registry};

use super::annotation_scanner::{ScanConsumer, accept_annotation, declared_annotations};

/// 类型层级扫描基类。
#[derive(Debug, Clone, Copy)]
pub struct AbstractTypeAnnotationScanner {
    include_super_class: bool,
    include_interfaces: bool,
}

impl AbstractTypeAnnotationScanner {
    /// 构造层级扫描器。
    pub fn new(include_super_class: bool, include_interfaces: bool) -> Self {
        Self {
            include_super_class,
            include_interfaces,
        }
    }

    /// 按层级扫描类型及其父类/接口上的注解。
    pub fn scan_type_hierarchy(&self, consumer: &mut ScanConsumer<'_>, start: ElementHandle) {
        let registry = global_registry().read();
        let mut visited = HashSet::new();
        let mut queue: VecDeque<Vec<ElementHandle>> = VecDeque::from([vec![start]]);
        let mut distance = 0i32;

        while let Some(level) = queue.pop_front() {
            let mut next_level = Vec::new();
            for handle in level {
                if !visited.insert(handle) {
                    continue;
                }

                for annotation in declared_annotations(handle) {
                    if accept_annotation(&annotation) {
                        consumer(distance, annotation);
                    }
                }

                if let Some(AnnotatedElement::Type(ty)) = registry.get(handle) {
                    if self.include_super_class && let Some(super_type) = ty.super_type {
                        next_level.push(super_type);
                    }
                    if self.include_interfaces {
                        next_level.extend(ty.interfaces.iter().copied());
                    }
                }
            }

            if !next_level.is_empty() {
                queue.push_back(next_level);
            }
            distance += 1;
        }
    }
}

/// 解析元素所属类型。
pub fn type_handle_of(element: ElementHandle) -> Option<ElementHandle> {
    let registry = global_registry().read();
    match registry.get(element)? {
        AnnotatedElement::Type(ty) => Some(ty.handle),
        AnnotatedElement::Method(method) => Some(method.declaring_type),
        AnnotatedElement::Field(field) => Some(field.declaring_type),
    }
}
