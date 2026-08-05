//! 对齐: `cn.hutool.core.annotation.scanner.AbstractTypeAnnotationScanner`

use std::collections::{HashSet, VecDeque};

use super::annotation_scanner::{ScanConsumer, accept_annotation, declared_annotations};
use crate::element::{AnnotatedElement, ElementHandle, global_registry};

/// 类型层级扫描基类。
pub struct AbstractTypeAnnotationScanner {
    include_super_class: bool,
    include_interfaces: bool,
}

impl AbstractTypeAnnotationScanner {
    /// 构造。
    pub fn new(include_super_class: bool, include_interfaces: bool) -> Self {
        Self {
            include_super_class,
            include_interfaces,
        }
    }

    /// 扫描类型层级。
    pub fn scan_type_hierarchy(&self, consumer: &mut ScanConsumer, start: ElementHandle) {
        let registry = global_registry().read();
        let mut accessed = HashSet::new();
        let mut deque: VecDeque<Vec<ElementHandle>> = VecDeque::new();
        deque.push_back(vec![start]);
        let mut index = 0i32;
        while let Some(level) = deque.pop_front() {
            let mut next = Vec::new();
            for handle in level {
                if accessed.contains(&handle) {
                    continue;
                }
                accessed.insert(handle);
                for annotation in declared_annotations(handle) {
                    if accept_annotation(&annotation) {
                        consumer(index, annotation);
                    }
                }
                if let Some(AnnotatedElement::Type(ty)) = registry.get(handle) {
                    if self.include_super_class {
                        if let Some(s) = ty.super_type {
                            next.push(s);
                        }
                    }
                    if self.include_interfaces {
                        next.extend(ty.interfaces.iter().copied());
                    }
                }
            }
            if !next.is_empty() {
                deque.push_back(next);
            }
            index += 1;
        }
    }
}

/// 获取类型 handle（元素必须是 Type/Method/Field 所属类型）。
pub fn type_handle_of(element: ElementHandle) -> Option<ElementHandle> {
    let registry = global_registry().read();
    match registry.get(element)? {
        AnnotatedElement::Type(t) => Some(t.handle),
        AnnotatedElement::Method(m) => Some(m.declaring_type),
        AnnotatedElement::Field(f) => Some(f.declaring_type),
    }
}
