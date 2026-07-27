//! Workspace-owned `MethodAnnotationScanner`.

use std::sync::Arc;

use hutool_macro::AnnotationMirror;
use hutool_macro::element::{ElementHandle, ElementKind, global_registry};

use super::annotation_scanner::{
    AnnotationScanner, ScanConsumer, accept_annotation, declared_annotations,
};

/// 对齐 Java 类: `cn.hutool.core.annotation.scanner.MethodAnnotationScanner`
#[derive(Debug, Default)]
pub struct MethodAnnotationScanner;

impl MethodAnnotationScanner {
    /// 构造方法扫描器。
    pub fn new(_include_super_class: bool, _include_interfaces: bool) -> Self {
        Self
    }
}

impl AnnotationScanner for MethodAnnotationScanner {
    fn support(&self, element: ElementHandle) -> bool {
        global_registry()
            .read()
            .get(element)
            .map(|annotated| annotated.kind() == ElementKind::Method)
            .unwrap_or(false)
    }

    fn scan(&self, consumer: &mut ScanConsumer<'_>, element: ElementHandle) {
        let registry = global_registry().read();
        if !matches!(
            registry.get(element),
            Some(hutool_macro::element::AnnotatedElement::Method(_))
        ) {
            return;
        }

        for (distance, method_handle) in registry.method_override_chain(element).into_iter().enumerate() {
            for annotation in declared_annotations(method_handle) {
                if accept_annotation(&annotation) {
                    consumer(distance as i32, annotation);
                }
            }
        }
    }
}

impl MethodAnnotationScanner {
    /// 获取方法及覆写链上的注解。
    pub fn get_annotations(&self, element: ElementHandle) -> Vec<Arc<AnnotationMirror>> {
        AnnotationScanner::get_annotations(self, element)
    }
}
