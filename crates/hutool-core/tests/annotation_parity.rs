//! `cn.hutool.core.annotation` 对比验证测试（已覆盖部分）
//! 来源: hutool-core/src/test/java/cn/hutool/core/annotation/AnnotationUtilTest.java

use std::sync::Arc;

use hutool_annotation::{AnnotationUtil, ElementHandle, fixtures, global_registry};

mod annotation_common;
use annotation_common::*;

/// 对齐 Java: `AnnotationUtilTest.getCombinationAnnotationsTest()`
#[test]
fn annotation_util_get_combination_annotations_test() {
    let _guard = reset_all();
    let class = class_with_annotation();
    let annotations = AnnotationUtil::get_annotations(class, true);
    assert_eq!(2, annotations.len());
}

/// 对齐 Java: `AnnotationUtilTest.getAnnotationValueTest()`
#[test]
fn annotation_util_get_annotation_value_test() {
    let _guard = reset_all();
    let class = class_with_annotation();
    let value = AnnotationUtil::get_annotation_value(class, ANNOTATION_FOR_TEST).unwrap();
    let s = value.as_str().unwrap();
    assert!(s == "测试" || s == "repeat-annotation");
}

/// 对齐 Java: `AnnotationUtilTest.scanMetaAnnotationTest()`
#[test]
fn annotation_util_scan_meta_annotation_test() {
    let _guard = reset_all();
    register_meta_annotation_chain();
    let annotations = AnnotationUtil::scan_meta_annotation(ROOT_ANNOTATION);
    assert_eq!(4, annotations.len());
    assert_eq!(ROOT_META3, annotations.last().unwrap().annotation_type());
}

/// 对齐 Java: `AnnotationUtilTest.scanClassTest()`
#[test]
fn annotation_util_scan_class_test() {
    let _guard = reset_all();
    let target = register_target_class_hierarchy();
    let annotations = AnnotationUtil::scan_class(target);
    assert_eq!(5, annotations.len());
    assert_eq!(
        "TargetClass",
        annotation_value_str(&annotations[0], "value")
    );
    assert_eq!(
        "TargetSuperClass",
        annotation_value_str(&annotations[1], "value")
    );
    assert_eq!(
        "TargetSuperInterface",
        annotation_value_str(&annotations[2], "value")
    );
    assert_eq!(
        "SuperInterface",
        annotation_value_str(&annotations[3], "value")
    );
    assert_eq!(
        "SuperTargetSuperInterface",
        annotation_value_str(&annotations[4], "value")
    );
}

/// 对齐 Java: `AnnotationUtilTest.scanMethodTest()`
#[test]
fn annotation_util_scan_method_test() {
    let _guard = reset_all();
    let (target, method) = register_target_class_with_method();
    let annotations = AnnotationUtil::scan_method(method);
    assert_eq!(3, annotations.len());
    assert_eq!(
        "TargetClass",
        annotation_value_str(&annotations[0], "value")
    );
    assert_eq!(
        "TargetSuperClass",
        annotation_value_str(&annotations[1], "value")
    );
    assert_eq!(
        "TargetSuperInterface",
        annotation_value_str(&annotations[2], "value")
    );
    let _ = target;
}

/// 对齐 Java: `AnnotationUtilTest.AnnotationL1CacheTest()`
#[test]
fn annotation_util_annotation_l1_cache_test() {
    let _guard = reset_all();
    let class = class_with_annotation();
    let a1 = AnnotationUtil::get_annotation(class, ANNOTATION_FOR_TEST).unwrap();
    let a2 = AnnotationUtil::get_annotation(class, ANNOTATION_FOR_TEST).unwrap();
    assert!(Arc::ptr_eq(&a1, &a2), "L1 cache miss on direct annotation");

    let target = register_target_class_hierarchy();
    let h1 = AnnotationUtil::get_annotation(target, ANNOTATION_FOR_TEST).unwrap();
    let h2 = AnnotationUtil::get_annotation(target, ANNOTATION_FOR_TEST).unwrap();
    assert!(Arc::ptr_eq(&h1, &h2), "L1 cache miss on hierarchy scan");

    assert!(AnnotationUtil::get_annotation(class, DEPRECATED).is_none());
}

/// 对齐 Java: `AnnotationUtilTest.AnnotationAliasL2CacheTest()`
#[test]
fn annotation_util_annotation_alias_l2_cache_test() {
    let _guard = reset_all();
    let class = class_with_annotation();
    let a1 = AnnotationUtil::get_annotation_alias(class, ANNOTATION_FOR_TEST).unwrap();
    let a2 = AnnotationUtil::get_annotation_alias(class, ANNOTATION_FOR_TEST).unwrap();
    assert!(Arc::ptr_eq(&a1, &a2), "L2 cache miss on alias");
    let retry = AnnotationUtil::get_annotation_value_named(class, ANNOTATION_FOR_TEST, "retry")
        .unwrap()
        .as_str()
        .unwrap()
        .to_string();
    assert!(retry == "测试" || retry == "repeat-annotation");
}

/// 对齐 Java: `AnnotationUtilTest.SynthesizedAnnotationL2CacheTest()`
#[test]
fn annotation_util_synthesized_annotation_l2_cache_test() {
    let _guard = reset_all();
    register_meta_annotation_chain();
    let root = root_annotation_element();
    let a1 = AnnotationUtil::get_synthesized_annotation(root, ROOT_META1).unwrap();
    let a2 = AnnotationUtil::get_synthesized_annotation(root, ROOT_META1).unwrap();
    assert!(
        Arc::ptr_eq(&a1, &a2),
        "L2 cache miss on synthesized annotation"
    );
}

/// 对齐 Java: `AnnotationUtilTest.HasAnnotationWithCacheTest()`
#[test]
fn annotation_util_has_annotation_with_cache_test() {
    let _guard = reset_all();
    let class = class_with_annotation();
    let target = register_target_class_hierarchy();
    assert!(AnnotationUtil::has_annotation(class, ANNOTATION_FOR_TEST));
    assert!(AnnotationUtil::has_annotation(target, ANNOTATION_FOR_TEST));
    assert!(!AnnotationUtil::has_annotation(class, DEPRECATED));
}

fn class_with_annotation() -> ElementHandle {
    let mut reg = global_registry().write();
    fixtures::class_with_annotation(&mut reg)
}
