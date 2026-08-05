//! annotation parity 测试公共辅助。

#![allow(dead_code, missing_docs)] // 测试公共常量按需使用、无需文档，预留

use std::collections::HashMap;
use std::sync::{Arc, Mutex, MutexGuard};

use hutool_annotation::{
    AnnotationMirror, AnnotationSchema, AnnotationUtil, AnnotationValue, AttributeRef,
    CacheableAnnotationAttribute, CacheableSynthesizedAnnotationAttributeProcessor, ElementHandle,
    MethodBuilder, PostProcessors, Selectors, SynthesizedAnnotationPostProcessor,
    SynthesizedAnnotationSelector, TypeBuilder, clear_combination_cache_for_test, fixtures,
    global_registry,
};

static TEST_SERIAL: Mutex<()> = Mutex::new(());

/// 测试串行锁，避免并行测试争用全局 registry。
pub struct TestGuard<'a> {
    _lock: MutexGuard<'a, ()>,
}

pub const ANNOTATION_FOR_TEST: &str = "cn.hutool.core.annotation.AnnotationForTest";
pub const ALIAS: &str = "cn.hutool.core.annotation.Alias";
pub const DEPRECATED: &str = "java.lang.Deprecated";
pub const ROOT_ANNOTATION: &str = "cn.hutool.core.annotation.AnnotationUtilTest.RootAnnotation";
pub const ROOT_META1: &str = "cn.hutool.core.annotation.AnnotationUtilTest.RootMetaAnnotation1";
pub const ROOT_META2: &str = "cn.hutool.core.annotation.AnnotationUtilTest.RootMetaAnnotation2";
pub const ROOT_META3: &str = "cn.hutool.core.annotation.AnnotationUtilTest.RootMetaAnnotation3";

pub fn annotation_value_str(mirror: &AnnotationMirror, attr: &str) -> String {
    mirror
        .get_raw(attr)
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string()
}

pub fn register_meta_annotation_chain() {
    let mut reg = global_registry().write();
    register_meta_annotation_chain_inner(&mut reg);
}

fn register_meta_annotation_chain_inner(reg: &mut hutool_annotation::AnnotationRegistry) {
    reg.register_schema(AnnotationSchema {
        type_name: ROOT_META3,
        attributes: vec![],
        meta: vec![],
        inherited: false,
    });
    let m3 = reg.annotation(ROOT_META3, HashMap::new());
    reg.register_schema(AnnotationSchema {
        type_name: ROOT_META2,
        attributes: vec![],
        meta: vec![m3],
        inherited: false,
    });
    let m2 = reg.annotation(ROOT_META2, HashMap::new());
    reg.register_schema(AnnotationSchema {
        type_name: ROOT_META1,
        attributes: vec![],
        meta: vec![m2],
        inherited: false,
    });
    let m1 = reg.annotation(ROOT_META1, HashMap::new());
    let m3b = reg.annotation(ROOT_META3, HashMap::new());
    reg.register_schema(AnnotationSchema {
        type_name: ROOT_ANNOTATION,
        attributes: vec![],
        meta: vec![m3b, m1],
        inherited: false,
    });
}

/// 注册带 `@RootAnnotation` 的测试类元素。
pub fn root_annotation_element() -> ElementHandle {
    let mut reg = global_registry().write();
    register_meta_annotation_chain_inner(&mut reg);
    let root = reg.annotation(ROOT_ANNOTATION, HashMap::new());
    TypeBuilder::begin(&mut reg, "RootAnnotationHost")
        .annotate(root)
        .build()
}

fn mk_test_annotation(
    reg: &mut hutool_annotation::AnnotationRegistry,
    value: &str,
) -> Arc<AnnotationMirror> {
    reg.annotation(
        ANNOTATION_FOR_TEST,
        HashMap::from([("value".to_string(), AnnotationValue::String(value.into()))]),
    )
}

pub fn register_target_class_hierarchy() -> ElementHandle {
    let mut reg = global_registry().write();
    fixtures::init_base_schemas(&mut reg);
    let a_super_iface = mk_test_annotation(&mut reg, "SuperInterface");
    let super_iface = TypeBuilder::begin(&mut reg, "SuperInterface")
        .annotate(a_super_iface)
        .build();
    let a_super_target = mk_test_annotation(&mut reg, "SuperTargetSuperInterface");
    let super_target = TypeBuilder::begin(&mut reg, "SuperTargetSuperInterface")
        .annotate(a_super_target)
        .interface(super_iface)
        .build();
    let a_target_iface = mk_test_annotation(&mut reg, "TargetSuperInterface");
    let target_iface = TypeBuilder::begin(&mut reg, "TargetSuperInterface")
        .annotate(a_target_iface)
        .interface(super_target)
        .build();
    let a_super_class = mk_test_annotation(&mut reg, "TargetSuperClass");
    let super_class = TypeBuilder::begin(&mut reg, "TargetSuperClass")
        .annotate(a_super_class)
        .interface(super_iface)
        .build();
    let a_target = mk_test_annotation(&mut reg, "TargetClass");
    let target = TypeBuilder::begin(&mut reg, "TargetClass")
        .annotate(a_target)
        .super_type(super_class)
        .interface(target_iface)
        .build();
    let a_method_super = mk_test_annotation(&mut reg, "TargetSuperClass");
    let _ = MethodBuilder::begin(&mut reg, super_class, "testMethod")
        .annotate(a_method_super)
        .build();
    let a_method_iface = mk_test_annotation(&mut reg, "TargetSuperInterface");
    let _ = MethodBuilder::begin(&mut reg, target_iface, "testMethod")
        .annotate(a_method_iface)
        .build();
    target
}

pub fn register_target_class_with_method() -> (ElementHandle, ElementHandle) {
    let target = register_target_class_hierarchy();
    let mut reg = global_registry().write();
    let ann = mk_test_annotation(&mut reg, "TargetClass");
    let method = MethodBuilder::begin(&mut reg, target, "testMethod")
        .annotate(ann)
        .build();
    (target, method)
}

pub fn cacheable_attr(
    annotation: Arc<AnnotationMirror>,
    type_name: &'static str,
    attr: &'static str,
) -> Arc<dyn hutool_annotation::AnnotationAttribute> {
    Arc::new(CacheableAnnotationAttribute::new(
        annotation,
        AttributeRef::new(type_name, attr),
    ))
}

pub fn nearest_selector() -> Arc<dyn SynthesizedAnnotationSelector> {
    Selectors::nearest_and_oldest_priority()
}

pub fn processor_cache() -> CacheableSynthesizedAnnotationAttributeProcessor {
    CacheableSynthesizedAnnotationAttributeProcessor::new()
}

pub fn post_processor_alias() -> Arc<dyn SynthesizedAnnotationPostProcessor> {
    PostProcessors::alias_annotation_post_processor()
}

pub fn reset_all() -> TestGuard<'static> {
    let lock = TEST_SERIAL
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    fixtures::reset_registry();
    clear_combination_cache_for_test();
    AnnotationUtil::clear_caches_for_test();
    TestGuard { _lock: lock }
}
