//! 对齐: `cn.hutool.core.annotation.scanner.FieldAnnotationScanner`

use std::sync::Arc;

use super::annotation_scanner::{AnnotationScanner, ScanConsumer};
use super::element_annotation_scanner::ElementAnnotationScanner;
use crate::element::{ElementHandle, ElementKind, global_registry};

/// 对齐 Java 类: `cn.hutool.core.annotation.scanner.FieldAnnotationScanner`
pub struct FieldAnnotationScanner;

impl AnnotationScanner for FieldAnnotationScanner {
    fn support(&self, element: ElementHandle) -> bool {
        global_registry()
            .read()
            .get(element)
            .map(|e| e.kind() == ElementKind::Field)
            .unwrap_or(false)
    }

    fn scan(&self, consumer: &mut ScanConsumer, element: ElementHandle) {
        ElementAnnotationScanner.scan(consumer, element);
    }
}

impl FieldAnnotationScanner {
    /// 获取字段声明注解。
    pub fn get_annotations(
        &self,
        element: ElementHandle,
    ) -> Vec<Arc<crate::mirror::AnnotationMirror>> {
        AnnotationScanner::get_annotations(self, element)
    }
}

impl Default for FieldAnnotationScanner {
    fn default() -> Self {
        Self
    }
}
