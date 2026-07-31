//! AnnotationUtil 缺口 parity 测试。

use hutool_annotation::{fixtures, global_registry, AnnotationUtil, AnnotationValue};

use crate::annotation_common::*;

/// 对齐 Java: `AnnotationUtilTest.getCombinationAnnotationsWithClassTest()`
#[test]
fn annotation_util_get_combination_annotations_with_class_test() {
    let _guard = reset_all();
    let mut reg = global_registry().write();
    let class = fixtures::class_with_annotation(&mut reg);
    let annotations = AnnotationUtil::get_combination_annotations(class, ANNOTATION_FOR_TEST);
    assert_eq!(1, annotations.len());
    let v = annotations[0].get_raw("value").and_then(|x| x.as_str()).unwrap();
    assert!(v == "测试" || v == "repeat-annotation");
}

/// 对齐 Java: `AnnotationUtilTest.getAnnotationValueTest2()`
#[test]
fn annotation_util_get_annotation_value_test2() {
    let _guard = reset_all();
    let mut reg = global_registry().write();
    let class = fixtures::class_with_annotation(&mut reg);
    let names = AnnotationUtil::get_annotation_value_named(class, ANNOTATION_FOR_TEST, "names")
        .unwrap();
    match names {
        AnnotationValue::Array(items) => {
            assert_eq!(2, items.len());
            assert_eq!("测试1", items[0].as_str().unwrap());
            assert_eq!("测试2", items[1].as_str().unwrap());
        }
        _ => panic!("expected string array"),
    }
}

/// 对齐 Java: `AnnotationUtilTest.getAnnotationSyncAlias()`
#[test]
fn annotation_util_get_annotation_sync_alias() {
    let _guard = reset_all();
    let mut reg = global_registry().write();
    let class = fixtures::class_with_annotation(&mut reg);
    let direct = AnnotationUtil::get_annotation(class, ANNOTATION_FOR_TEST).unwrap();
    assert_eq!("", direct.get_raw("retry").and_then(|v| v.as_str()).unwrap_or(""));

    let alias = AnnotationUtil::get_annotation_alias(class, ANNOTATION_FOR_TEST).unwrap();
    let retry = alias.get_raw("retry").and_then(|v| v.as_str()).unwrap();
    assert!(retry == "测试" || retry == "repeat-annotation");
    assert!(AnnotationUtil::is_synthesized_annotation(&alias));
}

/// 对齐 Java: `AnnotationUtilTest.getAnnotationSyncAliasWhenNotAnnotation()`
#[test]
fn annotation_util_get_annotation_sync_alias_when_not_annotation() {
    let _guard = reset_all();
    let mut reg = global_registry().write();
    let class = fixtures::class_with_annotation(&mut reg);
    annotation_util_get_annotation_sync_alias();
    assert!(AnnotationUtil::get_annotation_alias(class, ALIAS).is_none());
}
