//! 对齐: `cn.hutool.core.annotation.SynthesizedAnnotationPostProcessor`

use std::sync::Arc;

use crate::synthesized_annotation_post_processor::SynthesizedAnnotationPostProcessor;

/// 预置后置处理器工厂。
pub struct PostProcessors;

impl PostProcessors {
    /// Alias 处理器。
    pub fn alias_annotation_post_processor() -> Arc<dyn SynthesizedAnnotationPostProcessor> {
        Arc::new(crate::alias_annotation_post_processor::AliasAnnotationPostProcessor)
    }

    /// MirrorLink 处理器。
    pub fn mirror_link_annotation_post_processor() -> Arc<dyn SynthesizedAnnotationPostProcessor> {
        Arc::new(crate::mirror_link_annotation_post_processor::MirrorLinkAnnotationPostProcessor)
    }

    /// AliasLink 处理器。
    pub fn alias_link_annotation_post_processor() -> Arc<dyn SynthesizedAnnotationPostProcessor> {
        Arc::new(crate::alias_link_annotation_post_processor::AliasLinkAnnotationPostProcessor)
    }
}
