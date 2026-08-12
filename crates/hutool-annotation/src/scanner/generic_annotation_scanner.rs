//! Workspace-owned `GenericAnnotationScanner`.

use std::sync::Arc;

use hutool_macro::element::{ElementHandle, ElementKind, global_registry};
use hutool_macro::scanner::MetaAnnotationScanner;
use hutool_macro::{AnnotationMirror, AnnotationTypeName};

use super::annotation_scanner::{AnnotationScanner, ScanConsumer};
use super::element_annotation_scanner::ElementAnnotationScanner;
use super::method_annotation_scanner::MethodAnnotationScanner;
use super::type_annotation_scanner::TypeAnnotationScanner;

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
        let kind = global_registry()
            .read()
            .get(element)
            .map(|annotated| annotated.kind());

        let mut collected: Vec<(i32, Arc<AnnotationMirror>)> = Vec::new();
        {
            let collected_ref = &mut collected;
            let mut collector: ScanConsumer<'_> =
                Box::new(move |distance, annotation| collected_ref.push((distance, annotation)));
            match kind {
                Some(ElementKind::Type) => self.type_scanner.scan(&mut collector, element),
                Some(ElementKind::Method) => self.method_scanner.scan(&mut collector, element),
                _ => self.element_scanner.scan(&mut collector, element),
            }
        }

        for (distance, annotation) in collected {
            consumer(distance, Arc::clone(&annotation));
            self.meta_scanner
                .scan_meta(annotation.annotation_type(), consumer);
        }
    }

    fn scan_meta(&self, annotation_type: AnnotationTypeName, consumer: &mut ScanConsumer<'_>) {
        self.meta_scanner.scan_meta(annotation_type, consumer);
    }
}
