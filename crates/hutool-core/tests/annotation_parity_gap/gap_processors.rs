//! Post-processor parity 测试。

use std::sync::Arc;

use hutool_annotation::{
    fixtures, global_registry, AliasAnnotationPostProcessor,
    AnnotationSynthesizer, AnnotationTypeName,
    GenericSynthesizedAggregateAnnotation, GenericSynthesizedAnnotation,
    SynthesizedAnnotation, SynthesizedAnnotationPostProcessor,
};
use hutool_annotation::fixtures_aggregate::{self, types as agg};

use crate::annotation_common::*;

struct NoOpSynthesizer;

impl AnnotationSynthesizer for NoOpSynthesizer {
    fn get_source(&self) -> Vec<Arc<hutool_annotation::AnnotationMirror>> {
        vec![]
    }
    fn get_annotation_selector(
        &self,
    ) -> Arc<dyn hutool_annotation::SynthesizedAnnotationSelector> {
        nearest_selector()
    }
    fn get_annotation_attribute_processor(
        &self,
    ) -> Arc<dyn hutool_annotation::SynthesizedAnnotationAttributeProcessor> {
        Arc::new(processor_cache())
    }
    fn get_annotation_post_processors(
        &self,
    ) -> Vec<Arc<dyn SynthesizedAnnotationPostProcessor>> {
        vec![]
    }
    fn get_synthesized_annotation(
        &self,
        _annotation_type: AnnotationTypeName,
    ) -> Option<Arc<dyn SynthesizedAnnotation>> {
        None
    }
    fn get_all_synthesized_annotation(
        &self,
    ) -> std::collections::HashMap<AnnotationTypeName, Arc<dyn SynthesizedAnnotation>> {
        Default::default()
    }
    fn synthesize(
        &self,
        _annotation_type: AnnotationTypeName,
    ) -> Option<Arc<hutool_annotation::AnnotationMirror>> {
        None
    }
    fn get_attribute_value(
        &self,
        _attribute_name: &str,
        _attribute_type: hutool_annotation::ValueKind,
    ) -> Option<hutool_annotation::AnnotationValue> {
        None
    }
}

/// 对齐 Java: `AliasAnnotationPostProcessorTest.processTest()`
#[test]
fn alias_annotation_post_processor_process_test() {
    let _guard = reset_all();
    let mut reg = global_registry().write();
    let annotation = fixtures::alias_post_processor_annotation(&mut reg);
    let syn: Arc<dyn SynthesizedAnnotation> = GenericSynthesizedAnnotation::new(
        Arc::clone(&annotation),
        Arc::clone(&annotation),
        0,
        0,
    );
    let processor = AliasAnnotationPostProcessor;
    processor.process(Arc::clone(&syn), &NoOpSynthesizer);
    let attrs = syn.get_attributes();
    let value_attr = attrs.get("value").unwrap();
    assert_eq!("value", value_attr.get_attribute_name());
    assert!(value_attr.is_wrapped());
    assert_eq!(
        "ForceAliasedAnnotationAttribute",
        value_attr.impl_type_name()
    );
    let name_attr = attrs.get("name").unwrap();
    assert!(!name_attr.is_wrapped());
    assert_eq!("CacheableAnnotationAttribute", name_attr.impl_type_name());
    let wrapped = value_attr.as_wrapped().unwrap();
    assert!(Arc::ptr_eq(&wrapped.get_linked(), name_attr));
}
/// 对齐 Java: `MirrorLinkAnnotationPostProcessorTest.processTest()`
#[test]
fn mirror_link_annotation_post_processor_process_test() {
    let _guard = reset_all();
    let mut reg = global_registry().write();
    fixtures_aggregate::register_schemas(&mut reg);
    let annotation = fixtures_aggregate::mirror_annotation(&mut reg, "Foo", None);
    let aggregate = GenericSynthesizedAggregateAnnotation::new(Arc::clone(&annotation));
    let syn = aggregate
        .get_synthesized_annotation(agg::MIRROR)
        .expect("mirror synthesized");
    let attrs = syn.get_attributes();
    assert!(attrs.get("value").unwrap().is_wrapped());
    assert!(attrs.get("name").unwrap().is_wrapped());
}

/// 对齐 Java: `AliasLinkAnnotationPostProcessorTest.processAliasForTest()`
#[test]
fn alias_link_annotation_post_processor_process_alias_for_test() {
    let _guard = reset_all();
    let mut reg = global_registry().write();
    fixtures_aggregate::register_schemas(&mut reg);
    let annotation = reg.annotation(agg::ALIAS_FOR, Default::default());
    let aggregate = GenericSynthesizedAggregateAnnotation::new(Arc::clone(&annotation));
    let meta = aggregate.synthesize(agg::META_ALIAS_FOR).unwrap();
    assert_eq!("Meta", meta.get_raw("name").and_then(|v| v.as_str()).unwrap());
    let child = aggregate.synthesize(agg::ALIAS_FOR).unwrap();
    assert_eq!("", child.get_raw("value").and_then(|v| v.as_str()).unwrap());
}

/// 对齐 Java: `AliasLinkAnnotationPostProcessorTest.processForceAliasForTest()`
#[test]
fn alias_link_annotation_post_processor_process_force_alias_for_test() {
    let _guard = reset_all();
    let mut reg = global_registry().write();
    fixtures_aggregate::register_schemas(&mut reg);
    let annotation = reg.annotation(agg::FORCE_ALIAS, Default::default());
    let aggregate = GenericSynthesizedAggregateAnnotation::new(Arc::clone(&annotation));
    let meta = aggregate.synthesize(agg::META_FORCE_ALIAS).unwrap();
    assert_eq!("", meta.get_raw("name").and_then(|v| v.as_str()).unwrap());
    let child = aggregate.synthesize(agg::FORCE_ALIAS).unwrap();
    assert_eq!("", child.get_raw("value").and_then(|v| v.as_str()).unwrap());
}
