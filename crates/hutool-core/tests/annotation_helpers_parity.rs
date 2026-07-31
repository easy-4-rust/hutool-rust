//! Wave-2 非反射 annotation helper 冒烟测试
//! 对齐: Alias / AliasFor / ForceAliasFor / MirrorFor / PropIgnore / RelationType

use hutool_annotation::{
    Alias, AliasFor, ForceAliasFor, MirrorFor, PropIgnore, RelationType, ALIAS_TYPE_NAME,
};

/// 对齐 Java: 元注解类型名与非反射描述符
#[test]
fn annotation_non_reflective_helpers() {
    assert_eq!(Alias::type_name(), ALIAS_TYPE_NAME);
    let alias = Alias::new("value");
    assert_eq!(alias.value, "value");

    let af = AliasFor::new("cn.example.Ann", "name");
    assert_eq!(af.attribute, "name");
    assert_eq!(AliasFor::type_name(), "cn.hutool.core.annotation.AliasFor");

    let ff = ForceAliasFor::new("cn.example.Ann", "id");
    assert_eq!(ff.attribute, "id");

    let mf = MirrorFor::new("cn.example.Ann", "mirror");
    assert_eq!(mf.attribute, "mirror");

    assert!(PropIgnore::should_ignore("_secret", &[]));
    assert!(PropIgnore::should_ignore("password", &["password"]));
    assert!(!PropIgnore::should_ignore("name", &[]));

    assert_eq!(RelationType::AliasFor, RelationType::AliasFor);
    assert_ne!(RelationType::MirrorFor, RelationType::ForceAliasFor);
}
