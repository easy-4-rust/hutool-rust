//! GenericSynthesizedAggregateAnnotationTest 夹具注册。

use std::collections::HashMap;
use std::sync::Arc;

use super::alias_annotation_post_processor::ALIAS_TYPE;
use super::element::{AnnotationRegistry, TypeBuilder};
use super::link::LINK_TYPE;
use super::mirror::{AnnotationMirror, AnnotationSchema, AnnotationValue, AttributeDef};
use super::relation_type::RelationType;

const P: &str = "cn.hutool.core.annotation.GenericSynthesizedAggregateAnnotationTest";

/// GenericSynthesizedAggregateAnnotationTest 测试类型名。
pub mod types {
    /// GrandParent 注解全限定名。
    pub const GRAND_PARENT: &str =
        "cn.hutool.core.annotation.GenericSynthesizedAggregateAnnotationTest.GrandParentAnnotation";
    /// Parent 注解全限定名。
    pub const PARENT: &str =
        "cn.hutool.core.annotation.GenericSynthesizedAggregateAnnotationTest.ParentAnnotation";
    /// Child 注解全限定名。
    pub const CHILD: &str =
        "cn.hutool.core.annotation.GenericSynthesizedAggregateAnnotationTest.ChildAnnotation";
    /// 镜像测试注解全限定名。
    pub const MIRROR: &str = "cn.hutool.core.annotation.GenericSynthesizedAggregateAnnotationTest.AnnotationForMirrorTest";
    /// AliasFor 测试注解全限定名。
    pub const ALIAS_FOR: &str = "cn.hutool.core.annotation.GenericSynthesizedAggregateAnnotationTest.AnnotationForAliasForTest";
    /// MetaAliasFor 测试注解全限定名。
    pub const META_ALIAS_FOR: &str = "cn.hutool.core.annotation.GenericSynthesizedAggregateAnnotationTest.MetaAnnotationForAliasForTest";
    /// ForceAliasFor 测试注解全限定名。
    pub const FORCE_ALIAS: &str = "cn.hutool.core.annotation.GenericSynthesizedAggregateAnnotationTest.AnnotationForceForAliasForTest";
    /// MetaForceAliasFor 测试注解全限定名。
    pub const META_FORCE_ALIAS: &str = "cn.hutool.core.annotation.GenericSynthesizedAggregateAnnotationTest.MetaAnnotationForForceAliasForTest";
    /// Link 测试注解全限定名。
    pub const LINK_TEST: &str =
        "cn.hutool.core.annotation.GenericSynthesizedAggregateAnnotationTest.AnnotationForLinkTest";
    /// MirrorThenAliasFor 测试注解全限定名。
    pub const MIRROR_THEN_ALIAS: &str = "cn.hutool.core.annotation.GenericSynthesizedAggregateAnnotationTest.AnnotationForMirrorThenAliasForTest";
    /// MetaMirrorThenAliasFor 测试注解全限定名。
    pub const META_MIRROR_THEN_ALIAS: &str = "cn.hutool.core.annotation.GenericSynthesizedAggregateAnnotationTest.MetaAnnotationForMirrorThenAliasForTest";
    /// MultiAliasFor 测试注解全限定名。
    pub const MULTI_ALIAS: &str = "cn.hutool.core.annotation.GenericSynthesizedAggregateAnnotationTest.AnnotationForMultiAliasForTest";
    /// MetaMultiAliasFor1 测试注解全限定名。
    pub const META_MULTI1: &str = "cn.hutool.core.annotation.GenericSynthesizedAggregateAnnotationTest.MetaAnnotationForMultiAliasForTest1";
    /// MetaMultiAliasFor2 测试注解全限定名。
    pub const META_MULTI2: &str = "cn.hutool.core.annotation.GenericSynthesizedAggregateAnnotationTest.MetaAnnotationForMultiAliasForTest2";
    /// ImplicitAlias 测试注解全限定名。
    pub const IMPLICIT: &str = "cn.hutool.core.annotation.GenericSynthesizedAggregateAnnotationTest.AnnotationForImplicitAliasTest";
    /// MetaImplicitAlias 测试注解全限定名。
    pub const META_IMPLICIT: &str = "cn.hutool.core.annotation.GenericSynthesizedAggregateAnnotationTest.MetaAnnotationForImplicitAliasTest";
}

fn link(
    reg: &AnnotationRegistry,
    annotation: &str,
    attribute: &str,
    relation: RelationType,
) -> Arc<AnnotationMirror> {
    let rel = match relation {
        RelationType::MirrorFor => "MIRROR_FOR",
        RelationType::ForceAliasFor => "FORCE_ALIAS_FOR",
        _ => "ALIAS_FOR",
    };
    reg.annotation(
        LINK_TYPE,
        HashMap::from([
            (
                "annotation".to_string(),
                AnnotationValue::String(annotation.to_string()),
            ),
            (
                "attribute".to_string(),
                AnnotationValue::String(attribute.to_string()),
            ),
            ("type".to_string(), AnnotationValue::String(rel.into())),
        ]),
    )
}

fn alias_meta(reg: &AnnotationRegistry, target: &str) -> Arc<AnnotationMirror> {
    reg.annotation(
        ALIAS_TYPE,
        HashMap::from([("value".to_string(), AnnotationValue::String(target.into()))]),
    )
}

/// 注册 GenericSynthesizedAggregateAnnotationTest 全部 schema。
pub fn register_schemas(reg: &mut AnnotationRegistry) {
    reg.register_schema(AnnotationSchema {
        type_name: types::GRAND_PARENT,
        attributes: vec![
            AttributeDef::string("grandParentValue", ""),
            AttributeDef::class_type("grandParentType", "java.lang.Void"),
        ],
        meta: vec![],
        inherited: false,
    });

    let parent_gp = reg.annotation(
        types::GRAND_PARENT,
        HashMap::from([(
            "grandParentValue".to_string(),
            AnnotationValue::String("Parent's GrandParent!".into()),
        )]),
    );
    reg.register_schema(AnnotationSchema {
        type_name: types::PARENT,
        attributes: vec![
            AttributeDef::string("parentValue", ""),
            AttributeDef::string("grandParentType", "java.lang.Void"),
        ],
        meta: vec![parent_gp],
        inherited: false,
    });

    let child_gp = reg.annotation(
        types::GRAND_PARENT,
        HashMap::from([(
            "grandParentValue".to_string(),
            AnnotationValue::String("Child's GrandParent!".into()),
        )]),
    );
    let child_parent = reg.annotation(
        types::PARENT,
        HashMap::from([(
            "parentValue".to_string(),
            AnnotationValue::String("Child's Parent!".into()),
        )]),
    );
    let child_value =
        AttributeDef::string("childValue", "").with_meta(alias_meta(reg, "childValueAlias"));
    reg.register_schema(AnnotationSchema {
        type_name: types::CHILD,
        attributes: vec![
            AttributeDef::string("childValueAlias", ""),
            child_value,
            AttributeDef::class_type("grandParentType", "java.lang.Void"),
        ],
        meta: vec![Arc::clone(&child_gp), child_parent],
        inherited: false,
    });

    let mirror_value = AttributeDef::string("value", "").with_meta(link(
        reg,
        "java.lang.annotation.Annotation",
        "name",
        RelationType::MirrorFor,
    ));
    let mirror_name = AttributeDef::string("name", "").with_meta(link(
        reg,
        "java.lang.annotation.Annotation",
        "value",
        RelationType::MirrorFor,
    ));
    reg.register_schema(AnnotationSchema {
        type_name: types::MIRROR,
        attributes: vec![mirror_value, mirror_name],
        meta: vec![],
        inherited: false,
    });

    reg.register_schema(AnnotationSchema {
        type_name: types::META_ALIAS_FOR,
        attributes: vec![AttributeDef::string("name", "")],
        meta: vec![reg.annotation(
            types::META_ALIAS_FOR,
            HashMap::from([("name".to_string(), AnnotationValue::String("Meta".into()))]),
        )],
        inherited: false,
    });
    let af_value = AttributeDef::string("value", "").with_meta(link(
        reg,
        types::META_ALIAS_FOR,
        "name",
        RelationType::AliasFor,
    ));
    reg.register_schema(AnnotationSchema {
        type_name: types::ALIAS_FOR,
        attributes: vec![af_value],
        meta: vec![reg.annotation(types::META_ALIAS_FOR, HashMap::new())],
        inherited: false,
    });

    reg.register_schema(AnnotationSchema {
        type_name: types::META_FORCE_ALIAS,
        attributes: vec![AttributeDef::string("name", "")],
        meta: vec![reg.annotation(
            types::META_FORCE_ALIAS,
            HashMap::from([("name".to_string(), AnnotationValue::String("Meta".into()))]),
        )],
        inherited: false,
    });
    let force_value = AttributeDef::string("value", "").with_meta(link(
        reg,
        types::META_FORCE_ALIAS,
        "name",
        RelationType::ForceAliasFor,
    ));
    reg.register_schema(AnnotationSchema {
        type_name: types::FORCE_ALIAS,
        attributes: vec![force_value],
        meta: vec![reg.annotation(types::META_FORCE_ALIAS, HashMap::new())],
        inherited: false,
    });

    reg.register_schema(AnnotationSchema {
        type_name: types::LINK_TEST,
        attributes: vec![
            AttributeDef::string("value", "value").with_meta(link(
                reg,
                types::LINK_TEST,
                "name",
                RelationType::AliasFor,
            )),
            AttributeDef::string("name", "name"),
        ],
        meta: vec![],
        inherited: false,
    });

    let mta_name = AttributeDef::string("name", "").with_meta(link(
        reg,
        "java.lang.annotation.Annotation",
        "value",
        RelationType::MirrorFor,
    ));
    let mta_value = AttributeDef::string("value", "").with_meta(link(
        reg,
        "java.lang.annotation.Annotation",
        "name",
        RelationType::MirrorFor,
    ));
    reg.register_schema(AnnotationSchema {
        type_name: types::META_MIRROR_THEN_ALIAS,
        attributes: vec![mta_name, mta_value],
        meta: vec![reg.annotation(
            types::META_MIRROR_THEN_ALIAS,
            HashMap::from([("name".to_string(), AnnotationValue::String("Meta".into()))]),
        )],
        inherited: false,
    });
    let mta_child = AttributeDef::string("childValue", "value").with_meta(link(
        reg,
        types::META_MIRROR_THEN_ALIAS,
        "name",
        RelationType::AliasFor,
    ));
    reg.register_schema(AnnotationSchema {
        type_name: types::MIRROR_THEN_ALIAS,
        attributes: vec![mta_child],
        meta: vec![reg.annotation(types::META_MIRROR_THEN_ALIAS, HashMap::new())],
        inherited: false,
    });

    let m1_name = AttributeDef::string("name", "").with_meta(link(
        reg,
        "java.lang.annotation.Annotation",
        "value1",
        RelationType::MirrorFor,
    ));
    let m1_v1 = AttributeDef::string("value1", "").with_meta(link(
        reg,
        "java.lang.annotation.Annotation",
        "name",
        RelationType::MirrorFor,
    ));
    reg.register_schema(AnnotationSchema {
        type_name: types::META_MULTI1,
        attributes: vec![m1_name, m1_v1],
        meta: vec![],
        inherited: false,
    });
    let m2_v2 = AttributeDef::string("value2", "").with_meta(link(
        reg,
        types::META_MULTI1,
        "name",
        RelationType::AliasFor,
    ));
    reg.register_schema(AnnotationSchema {
        type_name: types::META_MULTI2,
        attributes: vec![m2_v2],
        meta: vec![reg.annotation(types::META_MULTI1, HashMap::new())],
        inherited: false,
    });
    let multi_v3 = AttributeDef::string("value3", "value").with_meta(link(
        reg,
        types::META_MULTI2,
        "value2",
        RelationType::AliasFor,
    ));
    reg.register_schema(AnnotationSchema {
        type_name: types::MULTI_ALIAS,
        attributes: vec![multi_v3],
        meta: vec![reg.annotation(types::META_MULTI2, HashMap::new())],
        inherited: false,
    });

    let imp_name = AttributeDef::string("name", "").with_meta(link(
        reg,
        "java.lang.annotation.Annotation",
        "value",
        RelationType::MirrorFor,
    ));
    let imp_value = AttributeDef::string("value", "").with_meta(link(
        reg,
        "java.lang.annotation.Annotation",
        "name",
        RelationType::MirrorFor,
    ));
    reg.register_schema(AnnotationSchema {
        type_name: types::META_IMPLICIT,
        attributes: vec![imp_name, imp_value],
        meta: vec![reg.annotation(
            types::META_IMPLICIT,
            HashMap::from([("name".to_string(), AnnotationValue::String("Meta".into()))]),
        )],
        inherited: false,
    });
    reg.register_schema(AnnotationSchema {
        type_name: types::IMPLICIT,
        attributes: vec![AttributeDef::string("value", "")],
        meta: vec![reg.annotation(types::META_IMPLICIT, HashMap::new())],
        inherited: false,
    });

    let _ = P;
}

/// AnnotatedClass 上的 ChildAnnotation 实例。
pub fn annotated_class_child(reg: &mut AnnotationRegistry) -> Arc<AnnotationMirror> {
    register_schemas(reg);
    let child = reg.annotation(
        types::CHILD,
        HashMap::from([
            (
                "childValueAlias".to_string(),
                AnnotationValue::String("Child!".into()),
            ),
            (
                "grandParentType".to_string(),
                AnnotationValue::Class("java.lang.Integer".into()),
            ),
        ]),
    );
    TypeBuilder::begin(reg, "AnnotatedClass")
        .annotate(Arc::clone(&child))
        .build();
    child
}

/// 镜像测试注解实例。
pub fn mirror_annotation(
    reg: &mut AnnotationRegistry,
    value: &str,
    name: Option<&str>,
) -> Arc<AnnotationMirror> {
    register_schemas(reg);
    let mut attrs = HashMap::new();
    if !value.is_empty() {
        attrs.insert("value".to_string(), AnnotationValue::String(value.into()));
    }
    if let Some(n) = name {
        attrs.insert("name".to_string(), AnnotationValue::String(n.into()));
    }
    reg.annotation(types::MIRROR, attrs)
}
