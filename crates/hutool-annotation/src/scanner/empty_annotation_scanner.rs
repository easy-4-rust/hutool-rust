//! Workspace-owned `EmptyAnnotationScanner`.

use std::sync::Arc;

use hutool_macro::AnnotationMirror;
use hutool_macro::element::ElementHandle;

use super::annotation_scanner::{AnnotationScanner, ScanConsumer};

/// 对齐 Java 类: `cn.hutool.core.annotation.scanner.EmptyAnnotationScanner`
#[derive(Debug, Default)]
pub struct EmptyAnnotationScanner;

impl AnnotationScanner for EmptyAnnotationScanner {
    fn scan(&self, _consumer: &mut ScanConsumer<'_>, _element: ElementHandle) {}
}

impl EmptyAnnotationScanner {
    /// 恒定返回空注解列表。
    pub fn get_annotations(&self, _element: ElementHandle) -> Vec<Arc<AnnotationMirror>> {
        Vec::new()
    }
}
