//! 对齐: `cn.hutool.core.annotation.scanner.AnnotationScanner`

use std::sync::Arc;

use crate::element::{global_registry, ElementHandle};
use crate::mirror::{is_not_jdk_meta_annotation, AnnotationMirror, AnnotationTypeName};

/// 扫描回调。
pub type ScanConsumer<'a> = Box<dyn FnMut(i32, Arc<AnnotationMirror>) + 'a>;

/// 对齐 Java interface: `cn.hutool.core.annotation.scanner.AnnotationScanner`
pub trait AnnotationScanner: Send + Sync {
    /// 是否支持扫描该元素。
    fn support(&self, _element: ElementHandle) -> bool {
        false
    }

    /// 是否支持扫描注解类型（元注解扫描）。
    fn support_type(&self, _annotation_type: AnnotationTypeName) -> bool {
        false
    }

    /// 扫描元素。
    fn scan<'a>(&self, consumer: &mut ScanConsumer<'a>, element: ElementHandle);

    /// 扫描注解类型元注解。
    fn scan_meta<'a>(&self, _annotation_type: AnnotationTypeName, _consumer: &mut ScanConsumer<'a>) {}

    /// 获取全部注解。
    fn get_annotations(&self, element: ElementHandle) -> Vec<Arc<AnnotationMirror>> {
        let mut list = Vec::new();
        if self.support(element) {
            let list_ref = &mut list;
            let mut consumer: ScanConsumer<'_> =
                Box::new(move |_index, annotation| list_ref.push(annotation));
            self.scan(&mut consumer, element);
        }
        list
    }
}

/// 过滤 JDK 元注解。
pub fn accept_annotation(annotation: &AnnotationMirror) -> bool {
    is_not_jdk_meta_annotation(annotation.annotation_type())
}

/// 读取元素直接声明注解。
pub fn declared_annotations(element: ElementHandle) -> Vec<Arc<AnnotationMirror>> {
    global_registry()
        .read()
        .get(element)
        .map(|e| e.declared_annotations().to_vec())
        .unwrap_or_default()
}

/// 判断元素是否存在。
pub fn element_exists(element: ElementHandle) -> bool {
    global_registry().read().get(element).is_some()
}

/// 元素种类判断。
pub fn element_kind(element: ElementHandle) -> Option<super::super::element::ElementKind> {
    global_registry().read().get(element).map(|e| e.kind())
}

/// 预置扫描器常量访问。
pub struct Scanners;

impl Scanners {
    /// NOTHING
    pub fn nothing() -> Arc<dyn AnnotationScanner> {
        Arc::new(super::empty_annotation_scanner::EmptyAnnotationScanner)
    }

    /// DIRECTLY
    pub fn directly() -> Arc<dyn AnnotationScanner> {
        Arc::new(super::generic_annotation_scanner::GenericAnnotationScanner::new(
            false, false, false,
        ))
    }

    /// DIRECTLY_AND_META_ANNOTATION
    pub fn directly_and_meta() -> Arc<dyn AnnotationScanner> {
        Arc::new(super::generic_annotation_scanner::GenericAnnotationScanner::new(
            true, false, false,
        ))
    }

    /// TYPE_HIERARCHY
    pub fn type_hierarchy() -> Arc<dyn AnnotationScanner> {
        Arc::new(super::generic_annotation_scanner::GenericAnnotationScanner::new(
            false, true, true,
        ))
    }

    /// TYPE_HIERARCHY_AND_META_ANNOTATION
    pub fn type_hierarchy_and_meta() -> Arc<dyn AnnotationScanner> {
        Arc::new(super::generic_annotation_scanner::GenericAnnotationScanner::new(
            true, true, true,
        ))
    }
}
