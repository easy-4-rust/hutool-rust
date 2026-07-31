//! 对齐: `cn.hutool.core.annotation.scanner.MetaAnnotationScanner`

use std::collections::{HashSet, VecDeque};
use std::sync::Arc;

use super::annotation_scanner::{accept_annotation, AnnotationScanner, ScanConsumer};
use crate::element::{global_registry, ElementHandle};
use crate::mirror::{AnnotationMirror, AnnotationTypeName};

/// 对齐 Java 类: `cn.hutool.core.annotation.scanner.MetaAnnotationScanner`
pub struct MetaAnnotationScanner {
    include_super: bool,
}

impl MetaAnnotationScanner {
    /// 构造元注解扫描器。
    pub fn new(include_super: bool) -> Self {
        Self {
            include_super: include_super,
        }
    }

    /// 获取注解类型上的元注解列表。
    pub fn get_meta_annotations(&self, annotation_type: AnnotationTypeName) -> Vec<Arc<AnnotationMirror>> {
        let mut list = Vec::new();
        {
            let list_ref = &mut list;
            let mut consumer: ScanConsumer<'_> = Box::new(move |_i, a| list_ref.push(a));
            self.scan_meta_impl(annotation_type, &mut consumer);
        }
        list
    }
}

impl Default for MetaAnnotationScanner {
    fn default() -> Self {
        Self::new(true)
    }
}

impl AnnotationScanner for MetaAnnotationScanner {
    fn support_type(&self, annotation_type: AnnotationTypeName) -> bool {
        global_registry().read().schema(annotation_type).is_some()
    }

    fn scan_meta(&self, annotation_type: AnnotationTypeName, consumer: &mut ScanConsumer<'_>) {
        self.scan_meta_impl(annotation_type, consumer);
    }

    fn scan(&self, _consumer: &mut ScanConsumer<'_>, _element: ElementHandle) {}
}

impl MetaAnnotationScanner {
    /// 扫描元注解内部实现。
    pub fn scan_meta_impl(&self, annotation_type: AnnotationTypeName, consumer: &mut ScanConsumer<'_>) {
        let registry = global_registry().read();
        let Some(_schema) = registry.schema(annotation_type) else {
            return;
        };
        let mut accessed = HashSet::new();
        let mut deque: VecDeque<Vec<AnnotationTypeName>> = VecDeque::new();
        deque.push_back(vec![annotation_type]);
        let mut distance = 0i32;
        while let Some(level) = deque.pop_front() {
            let mut next = Vec::new();
            for ty in level {
                if accessed.contains(&ty) {
                    continue;
                }
                accessed.insert(ty);
                let Some(s) = registry.schema(ty) else {
                    continue;
                };
                for meta in &s.meta {
                    if !accept_annotation(meta) {
                        continue;
                    }
                    consumer(distance, Arc::clone(meta));
                    next.push(meta.annotation_type());
                }
            }
            if self.include_super && !next.is_empty() {
                deque.push_back(next);
            }
            distance += 1;
        }
    }
}
