//! 属性包装/缓存 parity 测试。

use std::sync::Arc;

use hutool_annotation::{
    ALIAS_TYPE, AbstractWrappedAnnotationAttribute, AliasedAnnotationAttribute,
    AnnotationAttribute, ForceAliasedAnnotationAttribute, MirroredAnnotationAttribute, ValueKind,
    fixtures, global_registry,
};

use crate::annotation_common::*;

const T1: &str = "AbstractWrappedAnnotationAttributeTest.AnnotationForTest1";
const T2: &str = "AbstractWrappedAnnotationAttributeTest.AnnotationForTest2";
const T3: &str = "AbstractWrappedAnnotationAttributeTest.AnnotationForTest3";
const ATTR_TEST: &str = "AttributeTest.AnnotationForTest";
const CACHEABLE_TEST: &str = "CacheableAnnotationAttributeTest.AnnotationForTest";

/// 对齐 Java: `AbstractWrappedAnnotationAttributeTest.workTest()`
#[test]
fn abstract_wrapped_annotation_attribute_work_test() {
    let _guard = reset_all();
    let mut reg = global_registry().write();
    let annotation = fixtures::wrapped_attribute_setup(&mut reg);
    let value_attr = cacheable_attr(Arc::clone(&annotation), T1, "value1");
    let name_attr = cacheable_attr(Arc::clone(&annotation), T1, "name1");
    let wrapper = AbstractWrappedAnnotationAttribute::test_wrapper(name_attr, value_attr);

    assert!(Arc::ptr_eq(&wrapper.get_annotation(), &annotation));
    assert_eq!(T1, wrapper.get_annotation_type());
    assert_eq!("name1", wrapper.get_attribute_name());
    assert_eq!(ValueKind::String, wrapper.get_attribute_type());
    assert!(wrapper.is_wrapped());
    assert_eq!("value1", wrapper.get_value().as_str().unwrap());
}

/// 对齐 Java: `AbstractWrappedAnnotationAttributeTest.multiWrapperTest()`
#[test]
fn abstract_wrapped_annotation_attribute_multi_wrapper_test() {
    let _guard = reset_all();
    let mut reg = global_registry().write();
    let a1 = fixtures::wrapped_attribute_setup(&mut reg);
    let a2 = reg.annotation(T2, Default::default());
    let a3 = reg.annotation(T3, Default::default());
    let v1 = cacheable_attr(Arc::clone(&a1), T1, "value1");
    let n1 = cacheable_attr(Arc::clone(&a1), T1, "name1");
    let w1 = AbstractWrappedAnnotationAttribute::test_wrapper(n1.clone(), v1.clone());
    let v2 = cacheable_attr(a2, T2, "value2");
    let w2 = AbstractWrappedAnnotationAttribute::test_wrapper(w1.clone(), v2.clone());
    let v3 = cacheable_attr(a3, T3, "value3");
    let w3 = AbstractWrappedAnnotationAttribute::test_wrapper(v3.clone(), w2.clone());

    assert_eq!(
        n1.impl_type_name(),
        w1.get_non_wrapped_original().impl_type_name()
    );
    let leaves1 = w1.get_all_linked_non_wrapped_attributes();
    assert_eq!(2, leaves1.len());
    assert_eq!(
        n1.impl_type_name(),
        w2.get_non_wrapped_original().impl_type_name()
    );
    assert_eq!(3, w2.get_all_linked_non_wrapped_attributes().len());
    assert_eq!(
        v3.impl_type_name(),
        w3.get_non_wrapped_original().impl_type_name()
    );
    assert_eq!(4, w3.get_all_linked_non_wrapped_attributes().len());
}

/// 对齐 Java: `AliasedAnnotationAttributeTest.baseInfoTest()`
#[test]
fn aliased_annotation_attribute_base_info_test() {
    let _guard = reset_all();
    let mut reg = global_registry().write();
    let annotation = fixtures::aliased_test_annotation(&mut reg, Some("name"), Some("value"));
    let value_attr = cacheable_attr(Arc::clone(&annotation), ATTR_TEST, "value");
    let name_attr = cacheable_attr(Arc::clone(&annotation), ATTR_TEST, "name");
    let aliased = AliasedAnnotationAttribute::new(value_attr.clone(), name_attr.clone());

    assert!(Arc::ptr_eq(&aliased.get_annotation(), &annotation));
    assert_eq!(ATTR_TEST, aliased.get_annotation_type());
    assert!(aliased.get_meta_annotation(ALIAS_TYPE).is_some());
    assert_eq!("value", aliased.get_attribute_name());
    assert_eq!(ValueKind::String, aliased.get_attribute_type());
}

/// 对齐 Java: `AliasedAnnotationAttributeTest.workWhenValueDefaultTest()`
#[test]
fn aliased_annotation_attribute_work_when_value_default_test() {
    let _guard = reset_all();
    let mut reg = global_registry().write();
    let annotation = fixtures::aliased_test_annotation(&mut reg, Some("name"), Some("value"));
    let value_attr = cacheable_attr(Arc::clone(&annotation), ATTR_TEST, "value");
    let name_attr = cacheable_attr(Arc::clone(&annotation), ATTR_TEST, "name");
    let aliased = AliasedAnnotationAttribute::new(value_attr, name_attr);

    assert_eq!("name", aliased.get_value().as_str().unwrap());
    assert!(!aliased.is_value_equivalent_to_default_value());
    assert!(aliased.is_wrapped());
}

/// 对齐 Java: `AliasedAnnotationAttributeTest.workWhenValueNonDefaultTest()`
#[test]
fn aliased_annotation_attribute_work_when_value_non_default_test() {
    let _guard = reset_all();
    let mut reg = global_registry().write();
    let annotation = fixtures::aliased_test_annotation(&mut reg, None, Some("value"));
    let value_attr = cacheable_attr(Arc::clone(&annotation), ATTR_TEST, "value");
    let name_attr = cacheable_attr(Arc::clone(&annotation), ATTR_TEST, "name");
    let aliased = AliasedAnnotationAttribute::new(value_attr, name_attr);

    assert_eq!("value", aliased.get_value().as_str().unwrap());
    assert!(!aliased.is_value_equivalent_to_default_value());
    assert!(aliased.is_wrapped());
}

/// 对齐 Java: `CacheableAnnotationAttributeTest.baseInfoTest()`
#[test]
fn cacheable_annotation_attribute_base_info_test() {
    let _guard = reset_all();
    let mut reg = global_registry().write();
    let annotation = fixtures::cacheable_test_annotation(&mut reg, "");
    let attr = cacheable_attr(annotation.clone(), CACHEABLE_TEST, "value");

    assert!(Arc::ptr_eq(&attr.get_annotation(), &annotation));
    assert_eq!(CACHEABLE_TEST, attr.get_annotation_type());
    assert_eq!("value", attr.get_attribute_name());
    assert_eq!(ValueKind::String, attr.get_attribute_type());
}

/// 对齐 Java: `CacheableAnnotationAttributeTest.workWhenValueDefaultTest()`
#[test]
fn cacheable_annotation_attribute_work_when_value_default_test() {
    let _guard = reset_all();
    let mut reg = global_registry().write();
    let annotation = fixtures::cacheable_test_annotation(&mut reg, "");
    let attr = cacheable_attr(annotation, CACHEABLE_TEST, "value");

    assert_eq!("", attr.get_value().as_str().unwrap());
    assert!(attr.is_value_equivalent_to_default_value());
    assert!(!attr.is_wrapped());
}

/// 对齐 Java: `CacheableAnnotationAttributeTest.workWhenValueNonDefaultTest()`
#[test]
fn cacheable_annotation_attribute_work_when_value_non_default_test() {
    let _guard = reset_all();
    let mut reg = global_registry().write();
    let annotation = fixtures::cacheable_test_annotation(&mut reg, "test");
    let attr = cacheable_attr(annotation, CACHEABLE_TEST, "value");

    assert_eq!("test", attr.get_value().as_str().unwrap());
    assert!(!attr.is_value_equivalent_to_default_value());
    assert!(!attr.is_wrapped());
}

/// 对齐 Java: `ForceAliasedAnnotationAttributeTest.baseInfoTest()`
#[test]
fn force_aliased_annotation_attribute_base_info_test() {
    let _guard = reset_all();
    let mut reg = global_registry().write();
    let annotation = fixtures::aliased_test_annotation(&mut reg, Some("name"), Some("value"));
    let value_attr = cacheable_attr(Arc::clone(&annotation), ATTR_TEST, "value");
    let name_attr = cacheable_attr(Arc::clone(&annotation), ATTR_TEST, "name");
    let forced = ForceAliasedAnnotationAttribute::new(value_attr.clone(), name_attr.clone());

    assert!(Arc::ptr_eq(&forced.get_annotation(), &annotation));
    assert_eq!(ATTR_TEST, forced.get_annotation_type());
    assert_eq!("value", forced.get_attribute_name());
    assert_eq!(ValueKind::String, forced.get_attribute_type());
}

/// 对齐 Java: `ForceAliasedAnnotationAttributeTest.workWhenValueDefaultTest()`
#[test]
fn force_aliased_annotation_attribute_work_when_value_default_test() {
    let _guard = reset_all();
    let mut reg = global_registry().write();
    let annotation = fixtures::aliased_test_annotation(&mut reg, Some("name"), None);
    let value_attr = cacheable_attr(Arc::clone(&annotation), ATTR_TEST, "value");
    let name_attr = cacheable_attr(Arc::clone(&annotation), ATTR_TEST, "name");
    let forced = ForceAliasedAnnotationAttribute::new(value_attr, name_attr);

    assert_eq!("name", forced.get_value().as_str().unwrap());
    assert!(!forced.is_value_equivalent_to_default_value());
    assert!(forced.is_wrapped());
}

/// 对齐 Java: `ForceAliasedAnnotationAttributeTest.workWhenValueNonDefaultTest()`
#[test]
fn force_aliased_annotation_attribute_work_when_value_non_default_test() {
    let _guard = reset_all();
    let mut reg = global_registry().write();
    let annotation = fixtures::aliased_test_annotation(&mut reg, None, Some("value"));
    let value_attr = cacheable_attr(Arc::clone(&annotation), ATTR_TEST, "value");
    let name_attr = cacheable_attr(Arc::clone(&annotation), ATTR_TEST, "name");
    let forced = ForceAliasedAnnotationAttribute::new(value_attr, name_attr);

    assert_eq!("value", forced.get_value().as_str().unwrap());
    assert!(!forced.is_value_equivalent_to_default_value());
    assert!(forced.is_wrapped());
}

/// 对齐 Java: `MirroredAnnotationAttributeTest.baseInfoTest()`
#[test]
fn mirrored_annotation_attribute_base_info_test() {
    let _guard = reset_all();
    let mut reg = global_registry().write();
    let annotation = fixtures::cacheable_test_annotation(&mut reg, "a");
    let v = cacheable_attr(Arc::clone(&annotation), CACHEABLE_TEST, "value");
    let n = cacheable_attr(Arc::clone(&annotation), CACHEABLE_TEST, "value");
    let mirrored = MirroredAnnotationAttribute::new(v.clone(), n.clone());

    assert!(Arc::ptr_eq(&mirrored.get_annotation(), &annotation));
    assert_eq!(CACHEABLE_TEST, mirrored.get_annotation_type());
    assert_eq!("value", mirrored.get_attribute_name());
    assert_eq!(ValueKind::String, mirrored.get_attribute_type());
}

/// 对齐 Java: `MirroredAnnotationAttributeTest.workWhenValueDefaultTest()`
#[test]
fn mirrored_annotation_attribute_work_when_value_default_test() {
    let _guard = reset_all();
    let mut reg = global_registry().write();
    let annotation = fixtures::cacheable_test_annotation(&mut reg, "");
    let v = cacheable_attr(Arc::clone(&annotation), CACHEABLE_TEST, "value");
    let n = cacheable_attr(Arc::clone(&annotation), CACHEABLE_TEST, "value");
    let mirrored = MirroredAnnotationAttribute::new(v, n);

    assert_eq!("", mirrored.get_value().as_str().unwrap());
    assert!(mirrored.is_value_equivalent_to_default_value());
    assert!(mirrored.is_wrapped());
}

/// 对齐 Java: `MirroredAnnotationAttributeTest.workWhenValueNonDefaultTest()`
#[test]
fn mirrored_annotation_attribute_work_when_value_non_default_test() {
    let _guard = reset_all();
    let mut reg = global_registry().write();
    let annotation = fixtures::cacheable_test_annotation(&mut reg, "same");
    let v = cacheable_attr(Arc::clone(&annotation), CACHEABLE_TEST, "value");
    let n = cacheable_attr(Arc::clone(&annotation), CACHEABLE_TEST, "value");
    let mirrored = MirroredAnnotationAttribute::new(v, n);

    assert_eq!("same", mirrored.get_value().as_str().unwrap());
    assert!(!mirrored.is_value_equivalent_to_default_value());
    assert!(mirrored.is_wrapped());
}
