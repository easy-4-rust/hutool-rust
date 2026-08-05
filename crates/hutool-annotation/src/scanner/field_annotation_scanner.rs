//! Workspace-owned `FieldAnnotationScanner`.

use std::sync::Arc;

use hutool_macro::AnnotationMirror;
use hutool_macro::element::{ElementHandle, ElementKind, global_registry};

use super::annotation_scanner::{AnnotationScanner, ScanConsumer};
use super::element_annotation_scanner::ElementAnnotationScanner;

/// 对齐 Java 类: `cn.hutool.core.annotation.scanner.FieldAnnotationScanner`
#[derive(Debug, Default)]
pub struct FieldAnnotationScanner;

impl AnnotationScanner for FieldAnnotationScanner {
    fn support(&self, element: ElementHandle) -> bool {
        global_registry()
            .read()
            .get(element)
            .map(|annotated| annotated.kind() == ElementKind::Field)
            .unwrap_or(false)
    }

    fn scan(&self, consumer: &mut ScanConsumer<'_>, element: ElementHandle) {
        ElementAnnotationScanner.scan(consumer, element);
    }
}

impl FieldAnnotationScanner {
    /// 获取字段声明注解。
    pub fn get_annotations(&self, element: ElementHandle) -> Vec<Arc<AnnotationMirror>> {
        AnnotationScanner::get_annotations(self, element)
    }
}
