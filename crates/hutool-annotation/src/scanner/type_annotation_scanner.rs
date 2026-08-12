//! Workspace-owned `TypeAnnotationScanner`.

use std::sync::Arc;

use hutool_macro::AnnotationMirror;
use hutool_macro::element::{ElementHandle, ElementKind, global_registry};

use super::abstract_type_annotation_scanner::{AbstractTypeAnnotationScanner, type_handle_of};
use super::annotation_scanner::{AnnotationScanner, ScanConsumer};

/// 对齐 Java 类: `cn.hutool.core.annotation.scanner.TypeAnnotationScanner`
#[derive(Debug, Clone, Copy)]
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

    /// 获取类型层级上的注解。
    pub fn get_annotations(&self, element: ElementHandle) -> Vec<Arc<AnnotationMirror>> {
        AnnotationScanner::get_annotations(self, element)
    }
}

impl AnnotationScanner for TypeAnnotationScanner {
    fn support(&self, element: ElementHandle) -> bool {
        global_registry()
            .read()
            .get(element)
            .map(|annotated| annotated.kind() == ElementKind::Type)
            .unwrap_or(false)
    }

    fn scan(&self, consumer: &mut ScanConsumer<'_>, element: ElementHandle) {
        if let Some(type_handle) = type_handle_of(element) {
            self.inner.scan_type_hierarchy(consumer, type_handle);
        }
    }
}
