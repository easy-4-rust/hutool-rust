//! 对齐: `cn.hutool.core.annotation.AliasAnnotationPostProcessor`

use std::collections::HashMap;
use std::sync::Arc;

use super::force_aliased_annotation_attribute::ForceAliasedAnnotationAttribute;
use super::mirror::AnnotationTypeName;
use super::synthesized_annotation::SynthesizedAnnotation;
use super::synthesized_annotation_post_processor::SynthesizedAnnotationPostProcessor;

/// `@Alias` 元注解类型名。
pub const ALIAS_TYPE: AnnotationTypeName = "cn.hutool.core.annotation.Alias";

/// 对齐 Java 类: `cn.hutool.core.annotation.AliasAnnotationPostProcessor`
#[derive(Debug, Default)]
pub struct AliasAnnotationPostProcessor;

impl SynthesizedAnnotationPostProcessor for AliasAnnotationPostProcessor {
    fn order(&self) -> i32 {
        i32::MIN
    }

    fn process(
        &self,
        synthesized_annotation: Arc<dyn SynthesizedAnnotation>,
        _synthesizer: &dyn super::annotation_synthesizer::AnnotationSynthesizer,
    ) {
        let mut attribute_map = synthesized_annotation.get_attributes();
        let alias_links: HashMap<String, String> = attribute_map
            .iter()
            .filter_map(|(name, attr)| {
                attr.get_meta_annotation(ALIAS_TYPE).and_then(|m| {
                    m.get_raw("value")
                        .and_then(|v| v.as_str())
                        .map(|alias| (name.clone(), alias.to_string()))
                })
            })
            .collect();

        for (attribute_name, attribute) in attribute_map.clone() {
            let Some(alias_name) = alias_links.get(&attribute_name) else {
                continue;
            };
            let Some(resolved_name) = resolve_root_alias(&alias_links, alias_name) else {
                continue;
            };
            let Some(resolved) = attribute_map.get(&resolved_name).cloned() else {
                continue;
            };
            if !std::ptr::eq(attribute.as_ref(), resolved.as_ref()) {
                attribute_map.insert(
                    attribute_name,
                    ForceAliasedAnnotationAttribute::new(attribute, resolved),
                );
            }
        }
        for (k, v) in attribute_map {
            synthesized_annotation.set_attribute(&k, v);
        }
    }
}

fn resolve_root_alias(links: &HashMap<String, String>, start: &str) -> Option<String> {
    let mut current = start.to_string();
    for _ in 0..links.len() + 1 {
        if let Some(next) = links.get(&current) {
            current = next.clone();
        } else {
            return Some(current);
        }
    }
    None
}
