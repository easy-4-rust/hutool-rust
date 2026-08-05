//! 对齐: `cn.hutool.core.annotation.scanner.TypeAnnotationScanner`

use std::sync::Arc;

use super::abstract_type_annotation_scanner::{AbstractTypeAnnotationScanner, type_handle_of};
use super::annotation_scanner::{AnnotationScanner, ScanConsumer};
use crate::element::{ElementHandle, ElementKind, global_registry};

/// 对齐 Java 类: `cn.hutool.core.annotation.scanner.TypeAnnotationScanner`
pub struct TypeAnnotationScanner {
    inner: AbstractTypeAnnotationScanner,
}

impl TypeAnnotationScanner {
    /// 构造类型扫描器。
    pub fn new(include_super_class: bool, include_interfaces: bool) -> Self {
        Self {
            inner: AbstractTypeAnnotationScanner::new(include_super_class, include_interfaces),
        }
    }
}

impl AnnotationScanner for TypeAnnotationScanner {
    fn support(&self, element: ElementHandle) -> bool {
        global_registry()
            .read()
            .get(element)
            .map(|e| e.kind() == ElementKind::Type)
            .unwrap_or(false)
    }

    fn scan(&self, consumer: &mut ScanConsumer, element: ElementHandle) {
        if let Some(ty) = type_handle_of(element) {
            self.inner.scan_type_hierarchy(consumer, ty);
        }
    }
}

impl TypeAnnotationScanner {
    /// 获取注解列表。
    pub fn get_annotations(
        &self,
        element: ElementHandle,
    ) -> Vec<Arc<super::super::mirror::AnnotationMirror>> {
        AnnotationScanner::get_annotations(self, element)
    }
}
