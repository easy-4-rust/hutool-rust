//! Workspace-owned `ElementAnnotationScanner`.

use hutool_macro::element::{ElementHandle, ElementKind, global_registry};

use super::annotation_scanner::{
    AnnotationScanner, ScanConsumer, accept_annotation, declared_annotations, element_exists,
};

/// 对齐 Java 类: `cn.hutool.core.annotation.scanner.ElementAnnotationScanner`
#[derive(Debug, Default)]
pub struct ElementAnnotationScanner;

impl AnnotationScanner for ElementAnnotationScanner {
    fn support(&self, element: ElementHandle) -> bool {
        element_exists(element)
    }

    fn scan(&self, consumer: &mut ScanConsumer<'_>, element: ElementHandle) {
        for annotation in declared_annotations(element) {
            if accept_annotation(&annotation) {
                consumer(0, annotation);
            }
        }
    }
}

impl ElementAnnotationScanner {
    /// support 测试辅助。
    pub fn support_element(element: ElementHandle) -> bool {
        global_registry()
            .read()
            .get(element)
            .map(|annotated| {
                matches!(
                    annotated.kind(),
                    ElementKind::Type | ElementKind::Method | ElementKind::Field
                )
            })
            .unwrap_or(false)
    }
}
