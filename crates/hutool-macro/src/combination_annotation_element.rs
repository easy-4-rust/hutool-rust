//! 对齐: `cn.hutool.core.annotation.CombinationAnnotationElement`

use std::collections::HashMap;
use std::sync::Arc;

use parking_lot::RwLock;

use super::element::{ElementHandle, global_registry};
use super::mirror::{AnnotationMirror, AnnotationTypeName, is_not_jdk_meta_annotation};

/// 对齐 Java 类: `cn.hutool.core.annotation.CombinationAnnotationElement`
pub struct CombinationAnnotationElement {
    element: ElementHandle,
    annotation_map: HashMap<AnnotationTypeName, Arc<AnnotationMirror>>,
}

impl CombinationAnnotationElement {
    /// 构造组合注解元素。
    pub fn new(element: ElementHandle) -> Self {
        let mut this = Self {
            element,
            annotation_map: HashMap::new(),
        };
        this.init();
        this
    }

    fn init(&mut self) {
        let registry = global_registry().read();
        let Some(el) = registry.get(self.element) else {
            return;
        };
        let declared = el.declared_annotations();
        self.parse_declared(declared);
        let all = Self::collect_all_annotations(self.element);
        if declared.len() != all.len() {
            self.annotation_map.clear();
            self.parse_all(&all);
        }
    }

    fn parse_declared(&mut self, annotations: &[Arc<AnnotationMirror>]) {
        for annotation in annotations {
            let ty = annotation.annotation_type();
            if is_not_jdk_meta_annotation(ty) && !self.annotation_map.contains_key(&ty) {
                self.annotation_map.insert(ty, Arc::clone(annotation));
            }
            if let Some(schema) = global_registry().read().schema(ty) {
                self.parse_declared(&schema.meta);
            }
        }
    }

    fn parse_all(&mut self, annotations: &[Arc<AnnotationMirror>]) {
        for annotation in annotations {
            let ty = annotation.annotation_type();
            if is_not_jdk_meta_annotation(ty) && !self.annotation_map.contains_key(&ty) {
                self.annotation_map.insert(ty, Arc::clone(annotation));
            }
            if let Some(schema) = global_registry().read().schema(ty) {
                let meta_annos: Vec<_> = schema.meta.clone();
                self.parse_all(&meta_annos);
            }
        }
    }

    fn collect_all_annotations(element: ElementHandle) -> Vec<Arc<AnnotationMirror>> {
        let registry = global_registry().read();
        registry
            .get(element)
            .map(|e| e.declared_annotations().to_vec())
            .unwrap_or_default()
    }

    /// 是否包含注解。
    pub fn is_annotation_present(&self, annotation_type: AnnotationTypeName) -> bool {
        self.annotation_map.contains_key(&annotation_type)
    }

    /// 获取指定注解。
    pub fn get_annotation(
        &self,
        annotation_type: AnnotationTypeName,
    ) -> Option<Arc<AnnotationMirror>> {
        self.annotation_map.get(&annotation_type).cloned()
    }

    /// 获取全部注解。
    pub fn get_annotations(&self) -> Vec<Arc<AnnotationMirror>> {
        self.annotation_map.values().cloned().collect()
    }
}

/// 组合元素缓存。
static COMBINATION_CACHE: std::sync::OnceLock<
    RwLock<HashMap<ElementHandle, Arc<CombinationAnnotationElement>>>,
> = std::sync::OnceLock::new();

fn combination_cache() -> &'static RwLock<HashMap<ElementHandle, Arc<CombinationAnnotationElement>>>
{
    COMBINATION_CACHE.get_or_init(|| RwLock::new(HashMap::new()))
}

/// 清空组合元素缓存（仅测试）。
pub fn clear_combination_cache_for_test() {
    combination_cache().write().clear();
}

/// 转换为组合注解元素（带缓存）。
pub fn to_combination(element: ElementHandle) -> Arc<CombinationAnnotationElement> {
    if let Some(cached) = combination_cache().read().get(&element).cloned() {
        return cached;
    }
    let combo = Arc::new(CombinationAnnotationElement::new(element));
    combination_cache()
        .write()
        .insert(element, Arc::clone(&combo));
    combo
}
