//! 对齐: `cn.hutool.core.annotation.scanner.GenericAnnotationScanner`

use std::sync::Arc;

use super::annotation_scanner::{AnnotationScanner, ScanConsumer};
use super::element_annotation_scanner::ElementAnnotationScanner;
use super::meta_annotation_scanner::MetaAnnotationScanner;
use super::method_annotation_scanner::MethodAnnotationScanner;
use super::type_annotation_scanner::TypeAnnotationScanner;
use crate::element::{ElementHandle, ElementKind, global_registry};
use crate::mirror::{AnnotationMirror, AnnotationTypeName};

/// 对齐 Java 类: `cn.hutool.core.annotation.scanner.GenericAnnotationScanner`
pub struct GenericAnnotationScanner {
    type_scanner: TypeAnnotationScanner,
    method_scanner: MethodAnnotationScanner,
    element_scanner: ElementAnnotationScanner,
    meta_scanner: Arc<MetaAnnotationScanner>,
}

impl GenericAnnotationScanner {
    /// 构造通用扫描器。
    pub fn new(
        enable_scan_meta_annotation: bool,
        enable_scan_super_class: bool,
        enable_scan_super_interface: bool,
    ) -> Self {
        Self {
            type_scanner: TypeAnnotationScanner::new(
                enable_scan_super_class,
                enable_scan_super_interface,
            ),
            method_scanner: MethodAnnotationScanner::new(
                enable_scan_super_class,
                enable_scan_super_interface,
            ),
            element_scanner: ElementAnnotationScanner,
            meta_scanner: Arc::new(MetaAnnotationScanner::new(enable_scan_meta_annotation)),
        }
    }
}

impl AnnotationScanner for GenericAnnotationScanner {
    fn support(&self, _element: ElementHandle) -> bool {
        true
    }

    fn support_type(&self, annotation_type: AnnotationTypeName) -> bool {
        self.meta_scanner.support_type(annotation_type)
    }

    fn scan(&self, consumer: &mut ScanConsumer<'_>, element: ElementHandle) {
        let kind = global_registry().read().get(element).map(|e| e.kind());
        let mut collected: Vec<(i32, Arc<AnnotationMirror>)> = Vec::new();
        {
            let collected_ref = &mut collected;
            let mut inner: ScanConsumer<'_> =
                Box::new(move |index, annotation| collected_ref.push((index, annotation)));
            match kind {
                Some(ElementKind::Type) => self.type_scanner.scan(&mut inner, element),
                Some(ElementKind::Method) => self.method_scanner.scan(&mut inner, element),
                _ => self.element_scanner.scan(&mut inner, element),
            }
        }
        for (index, annotation) in collected {
            consumer(index, Arc::clone(&annotation));
            self.meta_scanner
                .scan_meta(annotation.annotation_type(), consumer);
        }
    }

    fn scan_meta(&self, annotation_type: AnnotationTypeName, consumer: &mut ScanConsumer<'_>) {
        self.meta_scanner.scan_meta(annotation_type, consumer);
    }
}
