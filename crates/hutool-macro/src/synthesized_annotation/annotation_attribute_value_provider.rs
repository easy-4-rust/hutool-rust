//! 对齐: `cn.hutool.core.annotation.SynthesizedAnnotation`

use crate::mirror::{AnnotationValue, ValueKind};

use crate::synthesized_annotation::SynthesizedAnnotation;

/// 属性值提供者。
pub trait AnnotationAttributeValueProvider {
    /// 按名称与类型获取属性值。
    fn get_attribute_value_typed(
        &self,
        attribute_name: &str,
        attribute_type: ValueKind,
    ) -> Option<AnnotationValue>;
}

impl<T: SynthesizedAnnotation> AnnotationAttributeValueProvider for T {
    fn get_attribute_value_typed(
        &self,
        attribute_name: &str,
        attribute_type: ValueKind,
    ) -> Option<AnnotationValue> {
        if self.has_attribute(attribute_name, attribute_type) {
            self.get_attribute_value(attribute_name)
        } else {
            None
        }
    }
}
