//! 对齐: `cn.hutool.core.annotation.CacheableSynthesizedAnnotationAttributeProcessor`

use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use crate::hierarchical::default_hierarchical_cmp;
use crate::mirror::{AnnotationValue, ValueKind};
use crate::synthesized_annotation::SynthesizedAnnotation;
use crate::synthesized_annotation_attribute_processor::SynthesizedAnnotationAttributeProcessor;

/// 对齐 Java 类: `cn.hutool.core.annotation.CacheableSynthesizedAnnotationAttributeProcessor`
pub struct CacheableSynthesizedAnnotationAttributeProcessor {
    cache: RwLock<HashMap<(String, ValueKind), AnnotationValue>>,
}

impl Default for CacheableSynthesizedAnnotationAttributeProcessor {
    fn default() -> Self {
        Self::new()
    }
}

impl CacheableSynthesizedAnnotationAttributeProcessor {
    /// 创建带缓存的处理器。
    pub fn new() -> Self {
        Self {
            cache: RwLock::new(HashMap::new()),
        }
    }
}

impl SynthesizedAnnotationAttributeProcessor for CacheableSynthesizedAnnotationAttributeProcessor {
    fn get_attribute_value(
        &self,
        attribute_name: &str,
        attribute_type: ValueKind,
        synthesized_annotations: &[Arc<dyn SynthesizedAnnotation>],
    ) -> Option<AnnotationValue> {
        let key = (attribute_name.to_string(), attribute_type);
        if let Some(v) = self.cache.read().unwrap().get(&key) {
            return Some(v.clone());
        }
        let mut candidates: Vec<&Arc<dyn SynthesizedAnnotation>> = synthesized_annotations
            .iter()
            .filter(|ma| ma.has_attribute(attribute_name, attribute_type))
            .collect();
        candidates.sort_by(|a, b| default_hierarchical_cmp(a.as_ref(), b.as_ref()));
        let value = candidates
            .first()
            .and_then(|ma| ma.get_attribute_value(attribute_name));
        if let Some(v) = value.clone() {
            self.cache.write().unwrap().insert(key, v);
        }
        value
    }
}
