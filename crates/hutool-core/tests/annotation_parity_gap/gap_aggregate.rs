//! GenericSynthesizedAggregateAnnotation parity 测试。

use std::collections::HashMap;
use std::sync::Arc;

use hutool_annotation::{
    AnnotationSynthesizer, CacheableSynthesizedAnnotationAttributeProcessor,
    GenericSynthesizedAggregateAnnotation, SynthesizedAggregateAnnotation, ValueKind,
    fixtures_aggregate::{self, types as agg},
    global_registry, mirror_string,
};

use crate::annotation_common::reset_all;

/// 对齐 Java: `GenericSynthesizedAggregateAnnotationTest.baseSynthesisAnnotationWorkTest()`
#[test]
fn generic_synthesized_aggregate_annotation_base_synthesis_annotation_work_test() {
    let _guard = reset_all();
    let mut reg = global_registry().write();
    let child = fixtures_aggregate::annotated_class_child(&mut reg);
    let aggregate = GenericSynthesizedAggregateAnnotation::new(Arc::clone(&child));

    assert!(aggregate.is_annotation_present(agg::GRAND_PARENT));
    assert!(aggregate.is_annotation_present(agg::PARENT));
    assert!(aggregate.is_annotation_present(agg::CHILD));
    assert_eq!(3, aggregate.get_all_synthesized_annotation().len());
    assert_eq!(3, aggregate.get_annotation_post_processors().len());
    assert!(aggregate.get_synthesized_annotation(agg::CHILD).is_some());
}

/// 对齐 Java: `GenericSynthesizedAggregateAnnotationTest.synthesisAnnotationAttributeTest()`
#[test]
fn generic_synthesized_aggregate_annotation_synthesis_annotation_attribute_test() {
    let _guard = reset_all();
    let mut reg = global_registry().write();
    let child = fixtures_aggregate::annotated_class_child(&mut reg);
    let aggregate = GenericSynthesizedAggregateAnnotation::new(Arc::clone(&child));

    assert_eq!(1, aggregate.get_source().len());
    assert_eq!(3, aggregate.get_annotations().len());
    assert_eq!(
        "Child!",
        mirror_string(
            &aggregate.synthesize(agg::CHILD).expect("child proxy"),
            "childValue",
            ""
        )
    );
    assert_eq!(
        "Child!",
        aggregate
            .get_attribute_value("childValueAlias", ValueKind::String)
            .map(|v| v.as_str().unwrap_or("").to_string())
            .unwrap()
    );
    assert_eq!(
        "Child's Parent!",
        aggregate
            .get_attribute_value("parentValue", ValueKind::String)
            .map(|v| v.as_str().unwrap_or("").to_string())
            .unwrap()
    );
    assert_eq!(
        "Child's GrandParent!",
        aggregate
            .get_attribute_value("grandParentValue", ValueKind::String)
            .map(|v| v.as_str().unwrap_or("").to_string())
            .unwrap()
    );
}

/// 对齐 Java: `GenericSynthesizedAggregateAnnotationTest.syntheticAnnotationTest()`
#[test]
fn generic_synthesized_aggregate_annotation_synthetic_annotation_test() {
    let _guard = reset_all();
    let mut reg = global_registry().write();
    let child = fixtures_aggregate::annotated_class_child(&mut reg);
    let aggregate = GenericSynthesizedAggregateAnnotation::new(Arc::clone(&child));

    let child_syn = aggregate.synthesize(agg::CHILD).unwrap();
    assert_eq!(
        "Child!",
        child_syn
            .get_raw("childValue")
            .and_then(|v| v.as_str())
            .unwrap()
    );
    assert_eq!(
        "java.lang.Integer",
        child_syn
            .get_raw("grandParentType")
            .map(|v| match v {
                hutool_annotation::AnnotationValue::Class(c) => c.clone(),
                _ => String::new(),
            })
            .unwrap()
    );

    let parent_syn = aggregate.synthesize(agg::PARENT).unwrap();
    assert_eq!(
        "Child's Parent!",
        parent_syn
            .get_raw("parentValue")
            .and_then(|v| v.as_str())
            .unwrap()
    );

    let gp_syn = aggregate.synthesize(agg::GRAND_PARENT).unwrap();
    assert_eq!(
        "Child's GrandParent!",
        gp_syn
            .get_raw("grandParentValue")
            .and_then(|v| v.as_str())
            .unwrap()
    );
}

/// 对齐 Java: `GenericSynthesizedAggregateAnnotationTest.linkTest()`
#[test]
fn generic_synthesized_aggregate_annotation_link_test() {
    let _guard = reset_all();
    let mut reg = global_registry().write();
    fixtures_aggregate::register_schemas(&mut reg);
    let link_ann = reg.annotation(agg::LINK_TEST, HashMap::new());
    let aggregate = GenericSynthesizedAggregateAnnotation::new(link_ann);
    let link = aggregate.synthesize(agg::LINK_TEST).unwrap();
    assert_eq!(
        "name",
        link.get_raw("value")
            .and_then(|v| v.as_str())
            .unwrap_or("name")
    );
}

/// 对齐 Java: `GenericSynthesizedAggregateAnnotationTest.mirrorAttributeTest()`
#[test]
fn generic_synthesized_aggregate_annotation_mirror_attribute_test() {
    let _guard = reset_all();
    let mut reg = global_registry().write();
    let ann = fixtures_aggregate::mirror_annotation(&mut reg, "Foo", None);
    let syn = GenericSynthesizedAggregateAnnotation::new(ann)
        .synthesize(agg::MIRROR)
        .unwrap();
    assert_eq!("Foo", syn.get_raw("name").and_then(|v| v.as_str()).unwrap());
    assert_eq!(
        "Foo",
        syn.get_raw("value").and_then(|v| v.as_str()).unwrap()
    );

    let ann2 = fixtures_aggregate::mirror_annotation(&mut reg, "", Some("Foo"));
    let syn2 = GenericSynthesizedAggregateAnnotation::new(ann2)
        .synthesize(agg::MIRROR)
        .unwrap();
    assert_eq!(
        "Foo",
        syn2.get_raw("name").and_then(|v| v.as_str()).unwrap()
    );

    let ann3 = fixtures_aggregate::mirror_annotation(&mut reg, "Aoo", Some("Foo"));
    let result = std::panic::catch_unwind(|| {
        GenericSynthesizedAggregateAnnotation::new(ann3).synthesize(agg::MIRROR)
    });
    assert!(result.is_err());
}

/// 对齐 Java: `GenericSynthesizedAggregateAnnotationTest.aliasForTest()`
#[test]
fn generic_synthesized_aggregate_annotation_alias_for_test() {
    let _guard = reset_all();
    let mut reg = global_registry().write();
    fixtures_aggregate::register_schemas(&mut reg);
    let ann = reg.annotation(agg::ALIAS_FOR, HashMap::new());
    let aggregate = GenericSynthesizedAggregateAnnotation::new(ann);
    assert_eq!(
        "Meta",
        aggregate
            .synthesize(agg::META_ALIAS_FOR)
            .unwrap()
            .get_raw("name")
            .and_then(|v| v.as_str())
            .unwrap()
    );
    assert_eq!(
        "",
        aggregate
            .synthesize(agg::ALIAS_FOR)
            .unwrap()
            .get_raw("value")
            .and_then(|v| v.as_str())
            .unwrap()
    );

    let ann2 = reg.annotation(
        agg::ALIAS_FOR,
        HashMap::from([(
            "value".to_string(),
            hutool_annotation::AnnotationValue::String("Foo".into()),
        )]),
    );
    let aggregate2 = GenericSynthesizedAggregateAnnotation::new(ann2);
    assert_eq!(
        "Foo",
        aggregate2
            .synthesize(agg::META_ALIAS_FOR)
            .unwrap()
            .get_raw("name")
            .and_then(|v| v.as_str())
            .unwrap()
    );
}

/// 对齐 Java: `GenericSynthesizedAggregateAnnotationTest.forceAliasForTest()`
#[test]
fn generic_synthesized_aggregate_annotation_force_alias_for_test() {
    let _guard = reset_all();
    let mut reg = global_registry().write();
    fixtures_aggregate::register_schemas(&mut reg);
    let ann = reg.annotation(agg::FORCE_ALIAS, HashMap::new());
    let aggregate = GenericSynthesizedAggregateAnnotation::new(ann);
    assert_eq!(
        "",
        aggregate
            .synthesize(agg::META_FORCE_ALIAS)
            .unwrap()
            .get_raw("name")
            .and_then(|v| v.as_str())
            .unwrap()
    );

    let ann2 = reg.annotation(
        agg::FORCE_ALIAS,
        HashMap::from([(
            "value".to_string(),
            hutool_annotation::AnnotationValue::String("Foo".into()),
        )]),
    );
    let aggregate2 = GenericSynthesizedAggregateAnnotation::new(ann2);
    assert_eq!(
        "Foo",
        aggregate2
            .synthesize(agg::META_FORCE_ALIAS)
            .unwrap()
            .get_raw("name")
            .and_then(|v| v.as_str())
            .unwrap()
    );
}

/// 对齐 Java: `GenericSynthesizedAggregateAnnotationTest.aliasForAndMirrorTest()`
#[test]
fn generic_synthesized_aggregate_annotation_alias_for_and_mirror_test() {
    let _guard = reset_all();
    let mut reg = global_registry().write();
    fixtures_aggregate::register_schemas(&mut reg);
    let ann = reg.annotation(
        agg::MIRROR_THEN_ALIAS,
        HashMap::from([(
            "childValue".to_string(),
            hutool_annotation::AnnotationValue::String("test".into()),
        )]),
    );
    let aggregate = GenericSynthesizedAggregateAnnotation::new(ann);
    let meta = aggregate.synthesize(agg::META_MIRROR_THEN_ALIAS).unwrap();
    assert_eq!(
        "test",
        meta.get_raw("name").and_then(|v| v.as_str()).unwrap()
    );
    assert_eq!(
        "test",
        meta.get_raw("value").and_then(|v| v.as_str()).unwrap()
    );
}

/// 对齐 Java: `GenericSynthesizedAggregateAnnotationTest.multiAliasForTest()`
#[test]
fn generic_synthesized_aggregate_annotation_multi_alias_for_test() {
    let _guard = reset_all();
    let mut reg = global_registry().write();
    fixtures_aggregate::register_schemas(&mut reg);
    let ann = reg.annotation(
        agg::MULTI_ALIAS,
        HashMap::from([(
            "value3".to_string(),
            hutool_annotation::AnnotationValue::String("test".into()),
        )]),
    );
    let aggregate = GenericSynthesizedAggregateAnnotation::new(ann);
    assert_eq!(
        "test",
        aggregate
            .synthesize(agg::META_MULTI1)
            .unwrap()
            .get_raw("name")
            .and_then(|v| v.as_str())
            .unwrap()
    );
    assert_eq!(
        "test",
        aggregate
            .synthesize(agg::META_MULTI2)
            .unwrap()
            .get_raw("value2")
            .and_then(|v| v.as_str())
            .unwrap()
    );
}

/// 对齐 Java: `GenericSynthesizedAggregateAnnotationTest.implicitAliasTest()`
#[test]
fn generic_synthesized_aggregate_annotation_implicit_alias_test() {
    let _guard = reset_all();
    let mut reg = global_registry().write();
    fixtures_aggregate::register_schemas(&mut reg);
    let ann = reg.annotation(
        agg::IMPLICIT,
        HashMap::from([(
            "value".to_string(),
            hutool_annotation::AnnotationValue::String("Foo".into()),
        )]),
    );
    let aggregate = GenericSynthesizedAggregateAnnotation::new(ann);
    let meta = aggregate.synthesize(agg::META_IMPLICIT).unwrap();
    assert_eq!(
        "Meta",
        meta.get_raw("name").and_then(|v| v.as_str()).unwrap()
    );
    assert_eq!(
        "Foo",
        meta.get_raw("value").and_then(|v| v.as_str()).unwrap()
    );
}

/// 对齐 Java: `CacheableSynthesizedAnnotationAttributeProcessorTest.getAttributeValueTest()`
#[test]
fn cacheable_synthesized_annotation_attribute_processor_get_attribute_value_test() {
    use hutool_annotation::TestValueSynthesizedAnnotation;

    use hutool_annotation::SynthesizedAnnotationAttributeProcessor;

    let processor = CacheableSynthesizedAnnotationAttributeProcessor::new();
    let a1: Arc<dyn hutool_annotation::SynthesizedAnnotation> = TestValueSynthesizedAnnotation::new(
        1,
        0,
        HashMap::from([
            (
                "name".to_string(),
                hutool_annotation::AnnotationValue::String("name1".into()),
            ),
            (
                "value".to_string(),
                hutool_annotation::AnnotationValue::I32(111),
            ),
        ]),
    );
    let a2: Arc<dyn hutool_annotation::SynthesizedAnnotation> = TestValueSynthesizedAnnotation::new(
        0,
        0,
        HashMap::from([
            (
                "name".to_string(),
                hutool_annotation::AnnotationValue::String("name2".into()),
            ),
            (
                "value".to_string(),
                hutool_annotation::AnnotationValue::String("value2".into()),
            ),
        ]),
    );
    let list = vec![a1, a2];
    assert_eq!(
        "name2",
        processor
            .get_attribute_value("name", ValueKind::String, &list)
            .and_then(|v| v.as_str().map(str::to_string))
            .unwrap()
    );
    assert_eq!(
        111,
        processor
            .get_attribute_value("value", ValueKind::I32, &list)
            .and_then(|v| match v {
                hutool_annotation::AnnotationValue::I32(i) => Some(i),
                _ => None,
            })
            .unwrap()
    );
}
