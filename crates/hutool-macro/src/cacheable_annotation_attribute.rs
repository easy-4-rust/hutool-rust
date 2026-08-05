//! 对齐: `cn.hutool.core.annotation.CacheableAnnotationAttribute`

use std::sync::{Arc, OnceLock};

use super::annotation_attribute::AnnotationAttribute;
use super::element::global_registry;
use super::mirror::{
    AnnotationMirror, AnnotationTypeName, AnnotationValue, AttributeRef, ValueKind,
};

/// 对齐 Java 类: `cn.hutool.core.annotation.CacheableAnnotationAttribute`
pub struct CacheableAnnotationAttribute {
    annotation: Arc<AnnotationMirror>,
    attribute: AttributeRef,
    value_cache: OnceLock<AnnotationValue>,
    default_checked: OnceLock<bool>,
}

impl CacheableAnnotationAttribute {
    /// 构造可缓存注解属性。
    pub fn new(annotation: Arc<AnnotationMirror>, attribute: AttributeRef) -> Self {
        Self {
            annotation,
            attribute,
            value_cache: OnceLock::new(),
            default_checked: OnceLock::new(),
        }
    }

    fn default_value(&self) -> AnnotationValue {
        global_registry()
            .read()
            .schema(self.attribute.annotation_type)
            .and_then(|s| s.attribute(&self.attribute.name))
            .map(|a| a.default_value.clone())
            .unwrap_or(AnnotationValue::Unit)
    }
}

impl AnnotationAttribute for CacheableAnnotationAttribute {
    fn impl_type_name(&self) -> &'static str {
        "CacheableAnnotationAttribute"
    }

    fn get_annotation(&self) -> Arc<AnnotationMirror> {
        Arc::clone(&self.annotation)
    }

    fn get_attribute(&self) -> AttributeRef {
        self.attribute.clone()
    }

    fn get_value(&self) -> AnnotationValue {
        self.value_cache
            .get_or_init(|| {
                let registry = global_registry().read();
                registry
                    .schema(self.attribute.annotation_type)
                    .map(|schema| self.annotation.resolve_value(schema, &self.attribute.name))
                    .unwrap_or(AnnotationValue::Unit)
            })
            .clone()
    }

    fn is_value_equivalent_to_default_value(&self) -> bool {
        *self
            .default_checked
            .get_or_init(|| self.get_value() == self.default_value())
    }

    fn get_attribute_type(&self) -> ValueKind {
        global_registry()
            .read()
            .schema(self.attribute.annotation_type)
            .and_then(|s| s.attribute(&self.attribute.name))
            .map(|a| a.value_kind)
            .unwrap_or(ValueKind::Void)
    }

    fn get_meta_annotation(&self, type_name: AnnotationTypeName) -> Option<Arc<AnnotationMirror>> {
        global_registry()
            .read()
            .schema(self.attribute.annotation_type)
            .and_then(|s| s.attribute(&self.attribute.name))
            .and_then(|a| {
                a.meta
                    .iter()
                    .find(|m| m.annotation_type() == type_name)
                    .cloned()
            })
    }
}
