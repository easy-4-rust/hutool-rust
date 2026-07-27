//! Workspace-owned scanner facade for `cn.hutool.core.annotation.scanner.AnnotationScanner`.
//!
//! The trait contract stays aligned with `hutool-macro`, while helper
//! functions and concrete scanner wiring now live in this crate.

use std::sync::Arc;

pub use hutool_macro::scanner::annotation_scanner::{AnnotationScanner, ScanConsumer};

use hutool_macro::mirror::is_not_jdk_meta_annotation;
use hutool_macro::{AnnotationMirror, AnnotationTypeName};

use super::empty_annotation_scanner::EmptyAnnotationScanner;

/// 过滤 JDK 元注解。
pub fn accept_annotation(annotation: &AnnotationMirror) -> bool {
    is_not_jdk_meta_annotation(annotation.annotation_type())
}

/// 读取元素直接声明注解。
pub fn declared_annotations(
    element: hutool_macro::element::ElementHandle,
) -> Vec<Arc<AnnotationMirror>> {
    hutool_macro::element::global_registry()
        .read()
        .get(element)
        .map(|annotated| annotated.declared_annotations().to_vec())
        .unwrap_or_default()
}

/// 判断元素是否存在。
pub fn element_exists(element: hutool_macro::element::ElementHandle) -> bool {
    hutool_macro::element::global_registry()
        .read()
        .get(element)
        .is_some()
}

/// 元素种类判断。
pub fn element_kind(
    element: hutool_macro::element::ElementHandle,
) -> Option<hutool_macro::element::ElementKind> {
    hutool_macro::element::global_registry()
        .read()
        .get(element)
        .map(|annotated| annotated.kind())
}

/// 预置扫描器常量访问。
pub struct Scanners;

impl Scanners {
    /// `NOTHING`
    pub fn nothing() -> Arc<dyn AnnotationScanner> {
        Arc::new(EmptyAnnotationScanner)
    }

    /// `DIRECTLY`
    pub fn directly() -> Arc<dyn AnnotationScanner> {
        Arc::new(super::generic_annotation_scanner::GenericAnnotationScanner::new(
            false, false, false,
        ))
    }

    /// `DIRECTLY_AND_META_ANNOTATION`
    pub fn directly_and_meta() -> Arc<dyn AnnotationScanner> {
        Arc::new(super::generic_annotation_scanner::GenericAnnotationScanner::new(
            true, false, false,
        ))
    }

    /// `TYPE_HIERARCHY`
    pub fn type_hierarchy() -> Arc<dyn AnnotationScanner> {
        Arc::new(super::generic_annotation_scanner::GenericAnnotationScanner::new(
            false, true, true,
        ))
    }

    /// `TYPE_HIERARCHY_AND_META_ANNOTATION`
    pub fn type_hierarchy_and_meta() -> Arc<dyn AnnotationScanner> {
        Arc::new(super::generic_annotation_scanner::GenericAnnotationScanner::new(
            true, true, true,
        ))
    }

    /// 透传 trait 的类型扫描入口，便于入口层按 Java 语义调用。
    pub fn scan_meta(
        scanner: &dyn AnnotationScanner,
        annotation_type: AnnotationTypeName,
        consumer: &mut ScanConsumer<'_>,
    ) {
        scanner.scan_meta(annotation_type, consumer);
    }
}
