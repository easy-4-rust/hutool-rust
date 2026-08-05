//! 对齐: `cn.hutool.core.annotation.CacheableSynthesizedAnnotationAttributeProcessor`

use std::collections::HashMap;
use std::sync::Arc;

use crate::mirror::{AnnotationValue, ValueKind};
use crate::synthesized_annotation::SynthesizedAnnotation;

/// 测试用合成注解（带值 map）。
pub struct TestValueSynthesizedAnnotation {
    vertical_distance: i32,
    horizontal_distance: i32,
    values: HashMap<String, AnnotationValue>,
}

impl TestValueSynthesizedAnnotation {
    /// 创建测试合成注解。
    pub fn new(
        vertical_distance: i32,
        horizontal_distance: i32,
        values: HashMap<String, AnnotationValue>,
    ) -> Arc<Self> {
        Arc::new(Self {
            vertical_distance,
            horizontal_distance,
            values,
        })
    }
}

impl crate::hierarchical::Hierarchical for TestValueSynthesizedAnnotation {
    fn get_root(&self) -> Option<&dyn std::any::Any> {
        None
    }
    fn get_vertical_distance(&self) -> i32 {
        self.vertical_distance
    }
    fn get_horizontal_distance(&self) -> i32 {
        self.horizontal_distance
    }
}

impl SynthesizedAnnotation for TestValueSynthesizedAnnotation {
    fn get_annotation(&self) -> Arc<crate::mirror::AnnotationMirror> {
        Arc::new(crate::mirror::AnnotationMirror::new(
            "test.Test",
            Default::default(),
        ))
    }
    fn has_attribute(&self, attribute_name: &str, return_type: ValueKind) -> bool {
        self.values
            .get(attribute_name)
            .map(|v| crate::mirror::is_assignable(return_type, v))
            .unwrap_or(false)
    }
    fn get_attributes(
        &self,
    ) -> HashMap<String, Arc<dyn crate::annotation_attribute::AnnotationAttribute>> {
        Default::default()
    }
    fn set_attribute(
        &self,
        _attribute_name: &str,
        _attribute: Arc<dyn crate::annotation_attribute::AnnotationAttribute>,
    ) {
    }
    fn replace_attribute(
        &self,
        _attribute_name: &str,
        _operator: Box<
            dyn Fn(
                    Arc<dyn crate::annotation_attribute::AnnotationAttribute>,
                ) -> Arc<dyn crate::annotation_attribute::AnnotationAttribute>
                + Send
                + Sync,
        >,
    ) {
    }
    fn get_attribute_value(&self, attribute_name: &str) -> Option<AnnotationValue> {
        self.values.get(attribute_name).cloned()
    }
    fn annotation_type(&self) -> crate::mirror::AnnotationTypeName {
        "test.Test"
    }
}
