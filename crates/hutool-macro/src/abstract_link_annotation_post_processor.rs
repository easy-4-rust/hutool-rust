//! 对齐: `cn.hutool.core.annotation.AbstractLinkAnnotationPostProcessor`

use std::sync::Arc;

use super::annotation_synthesizer::AnnotationSynthesizer;
use super::link::{LINK_TYPE, Link};
use super::mirror::AnnotationTypeName;
use super::relation_type::RelationType;
use super::synthesized_annotation::SynthesizedAnnotation;

/// 对齐 Java 抽象类: `cn.hutool.core.annotation.AbstractLinkAnnotationPostProcessor`
pub struct AbstractLinkAnnotationPostProcessor;

impl AbstractLinkAnnotationPostProcessor {
    /// 处理指定 Link 类型。
    pub fn process_types(
        _processor: &dyn super::synthesized_annotation_post_processor::SynthesizedAnnotationPostProcessor,
        synthesized_annotation: Arc<dyn SynthesizedAnnotation>,
        synthesizer: &dyn AnnotationSynthesizer,
        relation_types: &[RelationType],
        mut handler: impl FnMut(
            Arc<dyn SynthesizedAnnotation>,
            Arc<dyn super::annotation_attribute::AnnotationAttribute>,
            Arc<dyn SynthesizedAnnotation>,
            Arc<dyn super::annotation_attribute::AnnotationAttribute>,
        ),
    ) {
        for (_original_attribute_name, original_attribute) in
            synthesized_annotation.get_attributes()
        {
            let Some(link) = get_link_annotation(original_attribute.as_ref(), relation_types)
            else {
                continue;
            };
            let target_type =
                get_linked_annotation_type(&link, synthesized_annotation.annotation_type());
            let Some(linked_annotation) = synthesizer.get_synthesized_annotation(target_type)
            else {
                continue;
            };
            let Some(linked_attribute) = linked_annotation
                .get_attributes()
                .get(link.attribute())
                .cloned()
            else {
                continue;
            };
            handler(
                Arc::clone(&synthesized_annotation),
                original_attribute,
                linked_annotation,
                linked_attribute,
            );
        }
    }
}

fn get_link_annotation(
    attribute: &dyn super::annotation_attribute::AnnotationAttribute,
    relation_types: &[RelationType],
) -> Option<Link> {
    attribute
        .get_meta_annotation(LINK_TYPE)
        .map(Link::from_mirror)
        .filter(|l| relation_types.contains(&l.relation_type()))
}

fn get_linked_annotation_type(link: &Link, default_type: AnnotationTypeName) -> AnnotationTypeName {
    if link.annotation_type() == "java.lang.annotation.Annotation" {
        default_type
    } else {
        super::element::global_registry()
            .read()
            .resolve_type_name(link.annotation_type())
            .unwrap_or(default_type)
    }
}
