//! 对齐: `cn.hutool.core.annotation.SynthesizedAggregateAnnotation`

use std::collections::HashMap;
use std::sync::Arc;

use super::annotation_synthesizer::AnnotationSynthesizer;
use super::mirror::{AnnotationMirror, AnnotationTypeName, AnnotationValue, ValueKind};

/// 对齐 Java interface: `cn.hutool.core.annotation.SynthesizedAggregateAnnotation`
pub trait SynthesizedAggregateAnnotation: AnnotationSynthesizer + Send + Sync {
    /// 获取根对象。
    fn get_root(&self) -> Arc<AnnotationMirror>;

    /// 获取指定注解。
    fn get_annotation(&self, annotation_type: AnnotationTypeName) -> Option<Arc<AnnotationMirror>>;

    /// 是否包含指定注解。
    fn is_annotation_present(&self, annotation_type: AnnotationTypeName) -> bool;

    /// 获取全部注解。
    fn get_annotations(&self) -> Vec<Arc<AnnotationMirror>>;

    /// 按类型合成视图。
    fn synthesize_view(&self, annotation_type: AnnotationTypeName)
    -> Option<Arc<AnnotationMirror>>;
}

/// 聚合注解便捷 trait 别名。
pub trait AggregateAnnotationExt: SynthesizedAggregateAnnotation {
    /// 获取属性值（默认实现委托 attribute processor）。
    fn aggregate_attribute_value(
        &self,
        attribute_name: &str,
        attribute_type: ValueKind,
    ) -> Option<AnnotationValue> {
        self.get_attribute_value(attribute_name, attribute_type)
    }
}

impl<T: SynthesizedAggregateAnnotation> AggregateAnnotationExt for T {}

/// 合成注解映射视图。
pub type SynthesizedAnnotationMap =
    HashMap<AnnotationTypeName, Arc<dyn super::synthesized_annotation::SynthesizedAnnotation>>;
