//! 对齐: `cn.hutool.core.annotation.MirrorLinkAnnotationPostProcessor`

use std::sync::Arc;

use super::abstract_link_annotation_post_processor::AbstractLinkAnnotationPostProcessor;
use super::mirrored_annotation_attribute::MirroredAnnotationAttribute;
use super::relation_type::RelationType;
use super::synthesized_annotation::SynthesizedAnnotation;
use super::synthesized_annotation_post_processor::SynthesizedAnnotationPostProcessor;

/// 对齐 Java 类: `cn.hutool.core.annotation.MirrorLinkAnnotationPostProcessor`
#[derive(Debug, Default)]
pub struct MirrorLinkAnnotationPostProcessor;

impl SynthesizedAnnotationPostProcessor for MirrorLinkAnnotationPostProcessor {
    fn order(&self) -> i32 {
        i32::MIN + 1
    }

    fn process(
        &self,
        synthesized_annotation: Arc<dyn SynthesizedAnnotation>,
        synthesizer: &dyn super::annotation_synthesizer::AnnotationSynthesizer,
    ) {
        AbstractLinkAnnotationPostProcessor::process_types(
            self,
            synthesized_annotation,
            synthesizer,
            &[RelationType::MirrorFor],
            |original_annotation, original_attribute, linked_annotation, linked_attribute| {
                if original_attribute.is_wrapped() || linked_attribute.is_wrapped() {
                    return;
                }
                let mirrored_original = MirroredAnnotationAttribute::new(
                    Arc::clone(&original_attribute),
                    Arc::clone(&linked_attribute),
                );
                original_annotation
                    .set_attribute(&original_attribute.get_attribute_name(), mirrored_original);
                let mirrored_linked = MirroredAnnotationAttribute::new(
                    Arc::clone(&linked_attribute),
                    Arc::clone(&original_attribute),
                );
                linked_annotation
                    .set_attribute(&linked_attribute.get_attribute_name(), mirrored_linked);
            },
        );
    }
}
