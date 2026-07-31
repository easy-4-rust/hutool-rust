//! 对齐: `cn.hutool.core.annotation.scanner.EmptyAnnotationScanner`

use std::sync::Arc;

use super::annotation_scanner::{AnnotationScanner, ScanConsumer};
use crate::element::ElementHandle;
use crate::mirror::AnnotationMirror;

/// 对齐 Java 类: `cn.hutool.core.annotation.scanner.EmptyAnnotationScanner`
#[derive(Debug, Default)]
pub struct EmptyAnnotationScanner;

impl AnnotationScanner for EmptyAnnotationScanner {
    fn scan(&self, _consumer: &mut ScanConsumer, _element: ElementHandle) {}
}

impl EmptyAnnotationScanner {
    /// 空列表。
    pub fn get_annotations(&self, _element: ElementHandle) -> Vec<Arc<AnnotationMirror>> {
        Vec::new()
    }
}
