//! enum_util parity tests
//! 对齐: `cn.hutool.core.util.EnumUtilTest`

use hutool_core::EnumUtil;

#[derive(Debug, Clone, PartialEq)]
enum Color {
    Red,
    Green,
    Blue,
}

#[derive(Debug, Clone, PartialEq)]
enum Size {
    Small,
    Medium,
    Large,
}

// ── 枚举名称 ──

#[test]
fn name_of_color() {
    assert_eq!(EnumUtil::name_of(&Color::Red), "Red");
    assert_eq!(EnumUtil::name_of(&Color::Green), "Green");
    assert_eq!(EnumUtil::name_of(&Color::Blue), "Blue");
}

#[test]
fn name_of_size() {
    assert_eq!(EnumUtil::name_of(&Size::Small), "Small");
    assert_eq!(EnumUtil::name_of(&Size::Large), "Large");
}

// ── 枚举查找 ──

#[test]
fn from_names_found() {
    let variants = vec![Color::Red, Color::Green, Color::Blue];
    assert_eq!(EnumUtil::from_names(&variants, "Red"), Some(Color::Red));
    assert_eq!(EnumUtil::from_names(&variants, "Green"), Some(Color::Green));
}

#[test]
fn from_names_not_found() {
    let variants = vec![Color::Red, Color::Green, Color::Blue];
    assert_eq!(EnumUtil::from_names(&variants, "Yellow"), None);
}

// ── 枚举列表 ──

#[test]
fn names_color() {
    let variants = vec![Color::Red, Color::Green, Color::Blue];
    let names = EnumUtil::names(&variants);
    assert_eq!(names, vec!["Red", "Green", "Blue"]);
}

#[test]
fn names_size() {
    let variants = vec![Size::Small, Size::Medium, Size::Large];
    let names = EnumUtil::names(&variants);
    assert_eq!(names, vec!["Small", "Medium", "Large"]);
}

#[test]
fn count_color() {
    let variants = vec![Color::Red, Color::Green, Color::Blue];
    assert_eq!(EnumUtil::count(&variants), 3);
}

// ── 枚举映射 ──

#[test]
fn name_map_color() {
    let variants = vec![Color::Red, Color::Green, Color::Blue];
    let map = EnumUtil::name_map(&variants);
    assert_eq!(map.len(), 3);
    assert_eq!(map.get("Red"), Some(&Color::Red));
    assert_eq!(map.get("Green"), Some(&Color::Green));
    assert_eq!(map.get("Blue"), Some(&Color::Blue));
}

// ── 枚举验证 ──

#[test]
fn contains_name_found() {
    let variants = vec![Color::Red, Color::Green, Color::Blue];
    assert!(EnumUtil::contains_name(&variants, "Red"));
    assert!(EnumUtil::contains_name(&variants, "Green"));
    assert!(EnumUtil::contains_name(&variants, "Blue"));
}

#[test]
fn contains_name_not_found() {
    let variants = vec![Color::Red, Color::Green, Color::Blue];
    assert!(!EnumUtil::contains_name(&variants, "Yellow"));
    assert!(!EnumUtil::contains_name(&variants, ""));
}

// ── 对齐 Hutool EnumUtilTest ──

#[derive(Debug, Clone, PartialEq)]
enum TestEnum {
    TEST1,
    TEST2,
    TEST3,
}

/// 对齐 Java: `EnumUtilTest.getNamesTest()`
#[test]
fn get_names_test() {
    let names = EnumUtil::names(&[TestEnum::TEST1, TestEnum::TEST2, TestEnum::TEST3]);
    assert_eq!(
        vec![
            "TEST1".to_string(),
            "TEST2".to_string(),
            "TEST3".to_string()
        ],
        names
    );
}

/// 对齐 Java: `EnumUtilTest.getEnumMapTest()`
#[test]
fn get_enum_map_test() {
    let enum_map = EnumUtil::name_map(&[TestEnum::TEST1, TestEnum::TEST2, TestEnum::TEST3]);
    assert_eq!(Some(&TestEnum::TEST1), enum_map.get("TEST1"));
}

// ── Hutool TEST parity gap wave ──
// ── Hutool EnumUtilTest remaining gaps ──

#[derive(Debug, Clone, PartialEq)]
enum TestEnumTyped {
    TEST1,
    TEST2,
    TEST3,
}

impl TestEnumTyped {
    /// 返回 Hutool TestEnum 的 type 字段值
    fn type_str(&self) -> &'static str {
        match self {
            Self::TEST1 => "type1",
            Self::TEST2 => "type2",
            Self::TEST3 => "type3",
        }
    }
}

fn test_enum_typed_variants() -> Vec<TestEnumTyped> {
    vec![
        TestEnumTyped::TEST1,
        TestEnumTyped::TEST2,
        TestEnumTyped::TEST3,
    ]
}

#[derive(Debug, Clone, PartialEq)]
enum SelfRefEnum {
    A,
    B,
    C,
}

impl SelfRefEnum {
    fn label(&self) -> &'static str {
        match self {
            Self::A => "labelA",
            Self::B => "labelB",
            Self::C => "labelC",
        }
    }
}

fn self_ref_enum_variants() -> Vec<SelfRefEnum> {
    vec![SelfRefEnum::A, SelfRefEnum::B, SelfRefEnum::C]
}

/// 对齐 Java: `EnumUtilTest.getFieldValuesTest()`
#[test]
fn get_field_values_test() {
    let all = test_enum_typed_variants();
    let types = EnumUtil::get_field_values(&all, |e| e.type_str().to_string());
    assert_eq!(types, vec!["type1", "type2", "type3"]);
}

/// 对齐 Java: `EnumUtilTest.getFieldNamesTest()`
#[test]
fn get_field_names_test() {
    let names = EnumUtil::get_field_names(&["type", "name"]);
    assert!(names.contains(&"type".to_string()));
    assert!(names.contains(&"name".to_string()));
}

/// 对齐 Java: `EnumUtilTest.getByTest()`
#[test]
fn get_by_test() {
    let all = test_enum_typed_variants();
    let found = EnumUtil::get_by_field(&all, |e| all.iter().position(|v| v == e).unwrap(), &1usize);
    assert_eq!(found, Some(TestEnumTyped::TEST2));
}

/// 对齐 Java: `EnumUtilTest.getFieldByTest()`
#[test]
fn get_field_by_test() {
    let all = test_enum_typed_variants();
    let ty = EnumUtil::get_field_by(
        &all,
        |e| e.type_str().to_string(),
        |e| all.iter().position(|v| v == e).unwrap(),
        &1usize,
    );
    assert_eq!(ty, Some("type2".to_string()));
    let ordinal = EnumUtil::get_field_by(
        &all,
        |e| all.iter().position(|v| v == e).unwrap(),
        |e| all.iter().position(|v| v == e).unwrap(),
        &1usize,
    );
    assert_eq!(ordinal, Some(1usize));
}

/// 对齐 Java: `EnumUtilTest.likeValueOfTest()`
#[test]
fn like_value_of_test() {
    let all = test_enum_typed_variants();
    let found = EnumUtil::like_value_of(
        &all,
        "type2",
        &[|e: &TestEnumTyped| e.type_str().to_string()],
    );
    assert_eq!(found, Some(TestEnumTyped::TEST2));
}

/// 对齐 Java: `EnumUtilTest.getNameFieldMapTest()`
#[test]
fn get_name_field_map_test() {
    let all = test_enum_typed_variants();
    let enum_map = EnumUtil::get_name_field_map(&all, |e| e.type_str().to_string());
    assert_eq!(enum_map.get("TEST1"), Some(&"type1".to_string()));
}

/// 对齐 Java: `EnumUtilTest.getFieldValuesRecursiveTest()`
#[test]
fn get_field_values_recursive_test() {
    let all = self_ref_enum_variants();
    let values = EnumUtil::get_field_values_recursive(&all, |e| e.label().to_string());
    assert_eq!(values.len(), 3);
    assert_eq!(values, vec!["labelA", "labelB", "labelC"]);
}
