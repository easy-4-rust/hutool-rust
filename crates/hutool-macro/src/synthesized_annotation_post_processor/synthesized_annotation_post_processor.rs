//! 对齐: `cn.hutool.core.annotation.SynthesizedAnnotationPostProcessor`

use std::sync::Arc;

use crate::annotation_synthesizer::AnnotationSynthesizer;
use crate::synthesized_annotation::SynthesizedAnnotation;

/// 对齐 Java interface: `cn.hutool.core.annotation.SynthesizedAnnotationPostProcessor`
pub trait SynthesizedAnnotationPostProcessor: Send + Sync {
    /// 处理顺序。
    fn order(&self) -> i32;

    /// 处理合成注解。
    fn process(
        &self,
        synthesized_annotation: Arc<dyn SynthesizedAnnotation>,
        synthesizer: &dyn AnnotationSynthesizer,
    );
}
