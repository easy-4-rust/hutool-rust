//! 对齐: `cn.hutool.core.annotation.scanner.MethodAnnotationScanner`

use super::abstract_type_annotation_scanner::AbstractTypeAnnotationScanner;
use super::annotation_scanner::{
    AnnotationScanner, ScanConsumer, accept_annotation, declared_annotations,
};
use crate::element::{AnnotatedElement, ElementHandle, ElementKind, global_registry};

/// 对齐 Java 类: `cn.hutool.core.annotation.scanner.MethodAnnotationScanner`
pub struct MethodAnnotationScanner {
    // 镜像 Java 类结构；当前实现直接走注册表方法覆盖链，未读取该字段。
    #[allow(dead_code)]
    inner: AbstractTypeAnnotationScanner,
}

impl MethodAnnotationScanner {
    /// 构造方法扫描器。
    pub fn new(include_super_class: bool, include_interfaces: bool) -> Self {
        Self {
            inner: AbstractTypeAnnotationScanner::new(include_super_class, include_interfaces),
        }
    }
}

impl AnnotationScanner for MethodAnnotationScanner {
    fn support(&self, element: ElementHandle) -> bool {
        global_registry()
            .read()
            .get(element)
            .map(|e| e.kind() == ElementKind::Method)
            .unwrap_or(false)
    }

    fn scan(&self, consumer: &mut ScanConsumer, element: ElementHandle) {
        let registry = global_registry().read();
        let AnnotatedElement::Method(_method) = registry.get(element).expect("method") else {
            return;
        };
        let chain = registry.method_override_chain(element);
        for (idx, mh) in chain.iter().enumerate() {
            for annotation in declared_annotations(*mh) {
                if accept_annotation(&annotation) {
                    consumer(idx as i32, annotation);
                }
            }
        }
    }
}
