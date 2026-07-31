//! 测试夹具注册，镜像 Java 测试内嵌类。

use std::collections::HashMap;
use std::sync::Arc;

use super::element::{
    global_registry, AnnotationRegistry, ElementHandle, FieldBuilder, MethodBuilder, TypeBuilder,
};
use super::mirror::{
    AnnotationMirror, AnnotationSchema, AnnotationValue, AttributeDef, ValueKind,
};

/// 常用测试类型名。
pub mod types {
    /// `AnnotationForTest` 全限定名。
    pub const ANNOTATION_FOR_TEST: &str = "cn.hutool.core.annotation.AnnotationForTest";
    /// `RepeatAnnotationForTest` 全限定名。
    pub const REPEAT_ANNOTATION_FOR_TEST: &str = "cn.hutool.core.annotation.RepeatAnnotationForTest";
    /// `Alias` 全限定名。
    pub const ALIAS: &str = "cn.hutool.core.annotation.Alias";
    /// `AnnotationForScannerTest` 全限定名。
    pub const ANNOTATION_FOR_SCANNER_TEST: &str =
        "cn.hutool.core.annotation.scanner.AnnotationForScannerTest";
}

/// 初始化 parity 测试公共 schema。
pub fn init_base_schemas(registry: &mut AnnotationRegistry) {
    registry.register_schema(AnnotationSchema {
        type_name: types::ALIAS,
        attributes: vec![AttributeDef::string("value", "")],
        meta: vec![],
        inherited: false,
    });

    let mut value_attr = AttributeDef::string("value", "");
    value_attr = value_attr.with_meta(Arc::new(AnnotationMirror::new(
        types::ALIAS,
        HashMap::from([("value".to_string(), AnnotationValue::String("value".into()))]),
    )));
    let mut retry_attr = AttributeDef::string("retry", "");
    retry_attr = retry_attr.with_meta(Arc::new(AnnotationMirror::new(
        types::ALIAS,
        HashMap::from([(
            "value".to_string(),
            AnnotationValue::String("value".into()),
        )]),
    )));
    registry.register_schema(AnnotationSchema {
        type_name: types::ANNOTATION_FOR_TEST,
        attributes: vec![
            value_attr,
            retry_attr,
            AttributeDef {
                name: "names",
                value_kind: ValueKind::Array,
                default_value: AnnotationValue::Array(vec![]),
                meta: vec![],
            },
        ],
        meta: vec![],
        inherited: false,
    });

    registry.register_schema(AnnotationSchema {
        type_name: types::REPEAT_ANNOTATION_FOR_TEST,
        attributes: vec![],
        meta: vec![registry.annotation(
            types::ANNOTATION_FOR_TEST,
            HashMap::from([(
                "value".to_string(),
                AnnotationValue::String("repeat-annotation".into()),
            )]),
        )],
        inherited: false,
    });

    registry.register_schema(AnnotationSchema {
        type_name: types::ANNOTATION_FOR_SCANNER_TEST,
        attributes: vec![AttributeDef::string("value", "")],
        meta: vec![],
        inherited: false,
    });
}

/// 构建 `ClassWithAnnotation` 测试类。
pub fn class_with_annotation(registry: &mut AnnotationRegistry) -> ElementHandle {
    init_base_schemas(registry);
    let a1 = registry.annotation(
        types::ANNOTATION_FOR_TEST,
        HashMap::from([
            ("value".to_string(), AnnotationValue::String("测试".into())),
            (
                "names".to_string(),
                AnnotationValue::Array(vec![
                    AnnotationValue::String("测试1".into()),
                    AnnotationValue::String("测试2".into()),
                ]),
            ),
        ]),
    );
    let a2 = registry.annotation(types::REPEAT_ANNOTATION_FOR_TEST, HashMap::new());
    TypeBuilder::begin(registry, "ClassWithAnnotation")
        .annotate(a1)
        .annotate(a2)
        .build()
}

/// 重置全局 registry（仅测试）。
pub fn reset_registry() {
    *global_registry().write() = AnnotationRegistry::new();
}

/// AbstractWrappedAnnotationAttributeTest 夹具。
pub fn wrapped_attribute_setup(registry: &mut AnnotationRegistry) -> Arc<AnnotationMirror> {
    let t1 = "AbstractWrappedAnnotationAttributeTest.AnnotationForTest1";
    let t2 = "AbstractWrappedAnnotationAttributeTest.AnnotationForTest2";
    let t3 = "AbstractWrappedAnnotationAttributeTest.AnnotationForTest3";
    registry.register_schema(AnnotationSchema {
        type_name: t1,
        attributes: vec![
            AttributeDef::string("value1", ""),
            AttributeDef::string("name1", ""),
        ],
        meta: vec![],
        inherited: false,
    });
    registry.register_schema(AnnotationSchema {
        type_name: t2,
        attributes: vec![AttributeDef::string("value2", "")],
        meta: vec![],
        inherited: false,
    });
    registry.register_schema(AnnotationSchema {
        type_name: t3,
        attributes: vec![AttributeDef::string("value3", "")],
        meta: vec![],
        inherited: false,
    });
    let a1 = registry.annotation(
        t1,
        HashMap::from([
            ("value1".to_string(), AnnotationValue::String("value1".into())),
            ("name1".to_string(), AnnotationValue::String("name1".into())),
        ]),
    );
    let a2 = registry.annotation(
        t2,
        HashMap::from([(
            "value2".to_string(),
            AnnotationValue::String("value2".into()),
        )]),
    );
    let a3 = registry.annotation(
        t3,
        HashMap::from([(
            "value3".to_string(),
            AnnotationValue::String("value3".into()),
        )]),
    );
    TypeBuilder::begin(registry, "ClassForTest1")
        .annotate(Arc::clone(&a1))
        .annotate(a2)
        .annotate(a3)
        .build();
    a1
}

/// AliasedAnnotationAttributeTest / CacheableAnnotationAttributeTest schema。
pub fn register_attribute_test_schema(registry: &mut AnnotationRegistry, alias_target: &str) {
    let alias = registry.annotation(
        types::ALIAS,
        HashMap::from([(
            "value".to_string(),
            AnnotationValue::String(alias_target.into()),
        )]),
    );
    let ty = "AttributeTest.AnnotationForTest";
    registry.register_schema(AnnotationSchema {
        type_name: ty,
        attributes: vec![
            AttributeDef::string("value", "").with_meta(Arc::clone(&alias)),
            AttributeDef::string("name", ""),
        ],
        meta: vec![],
        inherited: false,
    });
}

/// AliasAnnotationPostProcessorTest schema（value 别名 name）。
pub fn alias_post_processor_annotation(registry: &mut AnnotationRegistry) -> Arc<AnnotationMirror> {
    register_attribute_test_schema(registry, "name");
    let ty = "AttributeTest.AnnotationForTest";
    registry.annotation(ty, HashMap::new())
}

/// 带 name/value 的 Aliased 测试注解。
pub fn aliased_test_annotation(
    registry: &mut AnnotationRegistry,
    name: Option<&str>,
    value: Option<&str>,
) -> Arc<AnnotationMirror> {
    register_attribute_test_schema(registry, "name");
    let ty = "AttributeTest.AnnotationForTest";
    let mut attrs = HashMap::new();
    if let Some(n) = name {
        attrs.insert("name".to_string(), AnnotationValue::String(n.into()));
    }
    if let Some(v) = value {
        attrs.insert("value".to_string(), AnnotationValue::String(v.into()));
    }
    registry.annotation(ty, attrs)
}

/// Cacheable 默认值/非默认值测试注解。
pub fn cacheable_test_annotation(registry: &mut AnnotationRegistry, value: &str) -> Arc<AnnotationMirror> {
    let ty = "CacheableAnnotationAttributeTest.AnnotationForTest";
    registry.register_schema(AnnotationSchema {
        type_name: ty,
        attributes: vec![AttributeDef::string("value", "")],
        meta: vec![],
        inherited: false,
    });
    registry.annotation(
        ty,
        HashMap::from([("value".to_string(), AnnotationValue::String(value.into()))]),
    )
}

/// 注册 scanner 测试 Example 类（type, field, method）。
pub fn scanner_example(registry: &mut AnnotationRegistry) -> (ElementHandle, ElementHandle, ElementHandle) {
    init_base_schemas(registry);
    let scanner_anno = registry.annotation(types::ANNOTATION_FOR_SCANNER_TEST, HashMap::new());
    let ty = TypeBuilder::begin(registry, "scanner.Example").build();
    let field = FieldBuilder::begin(registry, ty, "id")
        .annotate(Arc::clone(&scanner_anno))
        .build();
    let method = MethodBuilder::begin(registry, ty, "getId").build();
    (ty, field, method)
}

/// MetaAnnotationScanner 元注解链。
pub fn meta_scanner_chain(registry: &mut AnnotationRegistry) -> &'static str {
    init_base_schemas(registry);
    let base = types::ANNOTATION_FOR_SCANNER_TEST;
    let t1 = "cn.hutool.core.annotation.scanner.AnnotationForScannerTest1";
    let t2 = "cn.hutool.core.annotation.scanner.AnnotationForScannerTest2";
    let t3 = "cn.hutool.core.annotation.scanner.AnnotationForScannerTest3";
    registry.register_schema(AnnotationSchema {
        type_name: t1,
        attributes: vec![],
        meta: vec![registry.annotation(base, HashMap::new())],
        inherited: false,
    });
    registry.register_schema(AnnotationSchema {
        type_name: t2,
        attributes: vec![],
        meta: vec![registry.annotation(t1, HashMap::new())],
        inherited: false,
    });
    registry.register_schema(AnnotationSchema {
        type_name: t3,
        attributes: vec![],
        meta: vec![registry.annotation(t2, HashMap::new())],
        inherited: false,
    });
    t3
}

/// TypeAnnotationScanner 测试层级。
pub fn type_scanner_hierarchy(registry: &mut AnnotationRegistry) -> ElementHandle {
    init_base_schemas(registry);
    let a_iface = registry.annotation(
        types::ANNOTATION_FOR_SCANNER_TEST,
        HashMap::from([(
            "value".to_string(),
            AnnotationValue::String("ExampleInterface".into()),
        )]),
    );
    let iface = TypeBuilder::begin(registry, "scanner.ExampleInterface")
        .annotate(a_iface)
        .build();
    let a_sup = registry.annotation(
        types::ANNOTATION_FOR_SCANNER_TEST,
        HashMap::from([(
            "value".to_string(),
            AnnotationValue::String("ExampleSupplerClass".into()),
        )]),
    );
    let sup = TypeBuilder::begin(registry, "scanner.ExampleSupplerClass")
        .annotate(a_sup)
        .interface(iface)
        .build();
    let a_example = registry.annotation(
        types::ANNOTATION_FOR_SCANNER_TEST,
        HashMap::from([(
            "value".to_string(),
            AnnotationValue::String("Example".into()),
        )]),
    );
    TypeBuilder::begin(registry, "scanner.Example")
        .annotate(a_example)
        .super_type(sup)
        .build()
}

/// Generic scanner 测试类层级。
pub fn generic_scanner_hierarchy(registry: &mut AnnotationRegistry) -> ElementHandle {
    let meta = "cn.hutool.core.annotation.scanner.MetaAnnotationForTest";
    let anno = "cn.hutool.core.annotation.scanner.AnnotationForTest";
    registry.register_schema(AnnotationSchema {
        type_name: meta,
        attributes: vec![],
        meta: vec![],
        inherited: false,
    });
    registry.register_schema(AnnotationSchema {
        type_name: anno,
        attributes: vec![AttributeDef::string("value", "")],
        meta: vec![registry.annotation(meta, HashMap::new())],
        inherited: false,
    });
    let marker = registry.annotation(anno, HashMap::new());
    let sup = TypeBuilder::begin(registry, "SupperForTest")
        .annotate(Arc::clone(&marker))
        .build();
    let iface = TypeBuilder::begin(registry, "InterfaceForTest")
        .annotate(Arc::clone(&marker))
        .build();
    TypeBuilder::begin(registry, "ClassForTest")
        .annotate(marker)
        .super_type(sup)
        .interface(iface)
        .build()
}
