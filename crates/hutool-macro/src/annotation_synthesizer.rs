//! 对齐: `cn.hutool.core.annotation.AnnotationSynthesizer`

use std::collections::HashMap;
use std::sync::Arc;

use super::mirror::{AnnotationTypeName, AnnotationValue, ValueKind};
use super::synthesized_annotation::SynthesizedAnnotation;
use super::synthesized_annotation_attribute_processor::SynthesizedAnnotationAttributeProcessor;
use super::synthesized_annotation_post_processor::SynthesizedAnnotationPostProcessor;
use super::synthesized_annotation_selector::SynthesizedAnnotationSelector;

/// 对齐 Java interface: `cn.hutool.core.annotation.AnnotationSynthesizer`
pub trait AnnotationSynthesizer: Send + Sync {
    /// 获取源。
    fn get_source(&self) -> Vec<Arc<super::mirror::AnnotationMirror>>;

    /// 获取选择器。
    fn get_annotation_selector(&self) -> Arc<dyn SynthesizedAnnotationSelector>;

    /// 获取属性处理器。
    fn get_annotation_attribute_processor(
        &self,
    ) -> Arc<dyn SynthesizedAnnotationAttributeProcessor>;

    /// 获取后置处理器列表。
    fn get_annotation_post_processors(&self) -> Vec<Arc<dyn SynthesizedAnnotationPostProcessor>>;

    /// 获取指定合成注解。
    fn get_synthesized_annotation(
        &self,
        annotation_type: AnnotationTypeName,
    ) -> Option<Arc<dyn SynthesizedAnnotation>>;

    /// 获取全部合成注解。
    fn get_all_synthesized_annotation(
        &self,
    ) -> HashMap<AnnotationTypeName, Arc<dyn SynthesizedAnnotation>>;

    /// 合成指定类型注解视图。
    fn synthesize(
        &self,
        annotation_type: AnnotationTypeName,
    ) -> Option<Arc<super::mirror::AnnotationMirror>>;

    /// 获取属性值。
    fn get_attribute_value(
        &self,
        attribute_name: &str,
        attribute_type: ValueKind,
    ) -> Option<AnnotationValue>;
}
