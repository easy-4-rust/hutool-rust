//! 对齐: `cn.hutool.core.annotation.AliasLinkAnnotationPostProcessor`

use std::sync::Arc;

use super::abstract_link_annotation_post_processor::AbstractLinkAnnotationPostProcessor;
use super::aliased_annotation_attribute::AliasedAnnotationAttribute;
use super::force_aliased_annotation_attribute::ForceAliasedAnnotationAttribute;
use super::relation_type::RelationType;
use super::synthesized_annotation::SynthesizedAnnotation;
use super::synthesized_annotation_post_processor::SynthesizedAnnotationPostProcessor;

/// 对齐 Java 类: `cn.hutool.core.annotation.AliasLinkAnnotationPostProcessor`
#[derive(Debug, Default)]
pub struct AliasLinkAnnotationPostProcessor;

impl SynthesizedAnnotationPostProcessor for AliasLinkAnnotationPostProcessor {
    fn order(&self) -> i32 {
        i32::MIN + 2
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
            &[RelationType::AliasFor, RelationType::ForceAliasFor],
            |original_annotation, original_attribute, linked_annotation, linked_attribute| {
                let link_type = original_attribute
                    .get_meta_annotation(super::link::LINK_TYPE)
                    .map(super::link::Link::from_mirror)
                    .map(|l| l.relation_type())
                    .unwrap_or(RelationType::AliasFor);
                let original = Arc::clone(&original_attribute);
                let attr_name = linked_attribute.get_attribute_name().to_string();
                if link_type == RelationType::AliasFor {
                    linked_annotation.replace_attribute(
                        &attr_name,
                        Box::new(move |old| {
                            AliasedAnnotationAttribute::new(Arc::clone(&original), old)
                        }),
                    );
                } else {
                    linked_annotation.replace_attribute(
                        &attr_name,
                        Box::new(move |old| {
                            ForceAliasedAnnotationAttribute::new(Arc::clone(&original), old)
                        }),
                    );
                }
                let _ = original_annotation;
            },
        );
    }
}
