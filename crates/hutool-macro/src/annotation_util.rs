//! 对齐: `cn.hutool.core.annotation.AnnotationUtil`
//! 来源: hutool-core/src/main/java/cn/hutool/core/annotation/AnnotationUtil.java

use std::collections::HashMap;
use std::sync::Arc;

use parking_lot::RwLock;

use super::annotation_synthesizer::AnnotationSynthesizer;
use super::combination_annotation_element::to_combination as combination_of;
use super::element::{ElementHandle, global_registry};
use super::generic_synthesized_aggregate_annotation::GenericSynthesizedAggregateAnnotation;
use super::mirror::{
    AnnotationMirror, AnnotationTypeName, AnnotationValue, ValueKind, is_jdk_meta_annotation,
    is_not_jdk_meta_annotation,
};
use super::scanner::annotation_scanner::{AnnotationScanner, Scanners};
use super::scanner::meta_annotation_scanner::MetaAnnotationScanner;

/// L1/L2 缓存哨兵。
enum CacheEntry {
    Missing,
    Present(Arc<AnnotationMirror>),
}

/// 注解查询键。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct AnnotationLookupKey {
    element: ElementHandle,
    annotation_type: AnnotationTypeName,
}

static L1_CACHE: std::sync::OnceLock<RwLock<HashMap<AnnotationLookupKey, CacheEntry>>> =
    std::sync::OnceLock::new();
static L2_CACHE: std::sync::OnceLock<RwLock<HashMap<AnnotationLookupKey, CacheEntry>>> =
    std::sync::OnceLock::new();

fn l1_cache() -> &'static RwLock<HashMap<AnnotationLookupKey, CacheEntry>> {
    L1_CACHE.get_or_init(|| RwLock::new(HashMap::new()))
}
fn l2_cache() -> &'static RwLock<HashMap<AnnotationLookupKey, CacheEntry>> {
    L2_CACHE.get_or_init(|| RwLock::new(HashMap::new()))
}

/// 对齐 Java 类: `cn.hutool.core.annotation.AnnotationUtil`
pub struct AnnotationUtil;

impl AnnotationUtil {
    /// 是否为 JDK 元注解。
    pub fn is_jdk_meta_annotation(type_name: AnnotationTypeName) -> bool {
        is_jdk_meta_annotation(type_name)
    }

    /// 是否非 JDK 元注解。
    pub fn is_not_jdk_mate_annotation(type_name: AnnotationTypeName) -> bool {
        is_not_jdk_meta_annotation(type_name)
    }

    /// 是否为合成注解。
    pub fn is_synthesized_annotation(annotation: &AnnotationMirror) -> bool {
        annotation.is_synthesized()
    }

    /// 获取组合注解数组。
    pub fn get_annotations(
        element: ElementHandle,
        to_combination: bool,
    ) -> Vec<Arc<AnnotationMirror>> {
        if to_combination {
            return combination_of(element).get_annotations();
        }
        global_registry()
            .read()
            .get(element)
            .map(|e| e.declared_annotations().to_vec())
            .unwrap_or_default()
    }

    /// 获取指定类型的组合注解。
    pub fn get_combination_annotations(
        element: ElementHandle,
        annotation_type: AnnotationTypeName,
    ) -> Vec<Arc<AnnotationMirror>> {
        Self::get_annotations(element, true)
            .into_iter()
            .filter(|a| a.annotation_type() == annotation_type)
            .collect()
    }

    /// 获取指定注解（L1 缓存）。
    pub fn get_annotation(
        element: ElementHandle,
        annotation_type: AnnotationTypeName,
    ) -> Option<Arc<AnnotationMirror>> {
        let key = AnnotationLookupKey {
            element,
            annotation_type,
        };
        if let Some(entry) = l1_cache().read().get(&key) {
            return match entry {
                CacheEntry::Missing => None,
                CacheEntry::Present(a) => Some(Arc::clone(a)),
            };
        }
        let result = combination_of(element).get_annotation(annotation_type);
        l1_cache().write().insert(
            key,
            match &result {
                None => CacheEntry::Missing,
                Some(a) => CacheEntry::Present(Arc::clone(a)),
            },
        );
        result
    }

    /// 是否包含注解。
    pub fn has_annotation(element: ElementHandle, annotation_type: AnnotationTypeName) -> bool {
        Self::get_annotation(element, annotation_type).is_some()
    }

    /// 获取注解 value 属性。
    pub fn get_annotation_value(
        element: ElementHandle,
        annotation_type: AnnotationTypeName,
    ) -> Option<AnnotationValue> {
        Self::get_annotation_value_named(element, annotation_type, "value")
    }

    /// 获取指定属性值。
    pub fn get_annotation_value_named(
        element: ElementHandle,
        annotation_type: AnnotationTypeName,
        property_name: &str,
    ) -> Option<AnnotationValue> {
        let annotation = Self::get_annotation(element, annotation_type)?;
        let registry = global_registry().read();
        let schema = registry.schema(annotation_type)?;
        Some(annotation.resolve_value(schema, property_name))
    }

    /// 获取别名同步后的注解（L2 缓存）。
    pub fn get_annotation_alias(
        element: ElementHandle,
        annotation_type: AnnotationTypeName,
    ) -> Option<Arc<AnnotationMirror>> {
        let key = AnnotationLookupKey {
            element,
            annotation_type,
        };
        if let Some(entry) = l2_cache().read().get(&key) {
            return match entry {
                CacheEntry::Missing => None,
                CacheEntry::Present(a) => Some(Arc::clone(a)),
            };
        }
        let annotation = Self::get_annotation(element, annotation_type)?;
        let aggregate = GenericSynthesizedAggregateAnnotation::with_scanner(
            vec![annotation],
            Arc::new(MetaAnnotationScanner::new(false)),
        );
        let result = aggregate.synthesize(annotation_type);
        l2_cache().write().insert(
            key,
            match &result {
                None => CacheEntry::Missing,
                Some(a) => CacheEntry::Present(Arc::clone(a)),
            },
        );
        result
    }

    /// 扫描元注解。
    pub fn scan_meta_annotation(annotation_type: AnnotationTypeName) -> Vec<Arc<AnnotationMirror>> {
        Scanners::directly_and_meta().get_annotations_for_type(annotation_type)
    }

    /// 扫描类层级注解。
    pub fn scan_class(element: ElementHandle) -> Vec<Arc<AnnotationMirror>> {
        Scanners::type_hierarchy().get_annotations(element)
    }

    /// 扫描方法层级注解。
    pub fn scan_method(element: ElementHandle) -> Vec<Arc<AnnotationMirror>> {
        Scanners::type_hierarchy().get_annotations(element)
    }

    /// 获取合成注解（L2）。
    pub fn get_synthesized_annotation(
        element: ElementHandle,
        annotation_type: AnnotationTypeName,
    ) -> Option<Arc<AnnotationMirror>> {
        if let Some(direct) = Self::get_annotation(element, annotation_type) {
            l2_cache().write().insert(
                AnnotationLookupKey {
                    element,
                    annotation_type,
                },
                CacheEntry::Present(Arc::clone(&direct)),
            );
            return Some(direct);
        }
        let combo = combination_of(element);
        for annotation in combo.get_annotations() {
            let aggregate = GenericSynthesizedAggregateAnnotation::new(annotation);
            if let Some(syn) = aggregate.synthesize(annotation_type) {
                l2_cache().write().insert(
                    AnnotationLookupKey {
                        element,
                        annotation_type,
                    },
                    CacheEntry::Present(Arc::clone(&syn)),
                );
                return Some(syn);
            }
        }
        None
    }

    /// 清空测试缓存。
    pub fn clear_caches_for_test() {
        l1_cache().write().clear();
        l2_cache().write().clear();
    }
}

/// AnnotationScanner 扩展。
pub trait AnnotationScannerExt: AnnotationScanner {
    /// 扫描注解类型并返回列表。
    fn get_annotations_for_type(
        &self,
        annotation_type: AnnotationTypeName,
    ) -> Vec<Arc<AnnotationMirror>> {
        MetaAnnotationScanner::new(true).get_meta_annotations(annotation_type)
    }
}

impl<T: AnnotationScanner + ?Sized> AnnotationScannerExt for T {}

/// 字符串属性读取 helper。
pub fn mirror_string(mirror: &AnnotationMirror, attr: &str, default: &str) -> String {
    mirror
        .get_raw(attr)
        .and_then(|v| v.as_str())
        .unwrap_or(default)
        .to_string()
}

/// 读取 class 属性为字符串。
pub fn mirror_class_name(mirror: &AnnotationMirror, attr: &str, default: &str) -> String {
    match mirror.get_raw(attr) {
        Some(AnnotationValue::Class(c)) => c.clone(),
        _ => default.to_string(),
    }
}

/// ValueKind 映射 helper。
pub fn value_kind_of_name(name: &str) -> ValueKind {
    match name {
        "int" | "integer" => ValueKind::I32,
        "long" => ValueKind::I64,
        "boolean" => ValueKind::Bool,
        "class" => ValueKind::Class,
        _ => ValueKind::String,
    }
}
