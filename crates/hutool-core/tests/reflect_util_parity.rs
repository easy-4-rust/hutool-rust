//! reflect_util parity tests
//! 对齐: hutool-core ReflectUtilTest

use hutool_core::ReflectUtil;
use std::any::TypeId;
use std::collections::HashMap;

// ── 类型判断 ──

#[test]
fn is_basic_type_i32() {
    assert!(ReflectUtil::is_basic_type::<i32>());
}

#[test]
fn is_basic_type_f64() {
    assert!(ReflectUtil::is_basic_type::<f64>());
}

#[test]
fn is_basic_type_bool() {
    assert!(ReflectUtil::is_basic_type::<bool>());
}

#[test]
fn is_basic_type_string() {
    assert!(!ReflectUtil::is_basic_type::<String>());
}

#[test]
fn is_basic_type_vec() {
    assert!(!ReflectUtil::is_basic_type::<Vec<i32>>());
}

#[test]
fn is_basic_type_dyn() {
    let val = 42i32;
    assert!(ReflectUtil::is_basic_type_dyn(&val));
    let val = "hello";
    assert!(!ReflectUtil::is_basic_type_dyn(&val));
}

#[test]
fn is_simple_type_string() {
    assert!(ReflectUtil::is_simple_type::<String>());
}

#[test]
fn is_simple_type_i32() {
    assert!(ReflectUtil::is_simple_type::<i32>());
}

#[test]
fn is_simple_type_vec() {
    assert!(!ReflectUtil::is_simple_type::<Vec<i32>>());
}

// ── 类型名称 ──

#[test]
fn type_name_i32() {
    assert_eq!(ReflectUtil::type_name::<i32>(), "i32");
}

#[test]
fn type_name_string() {
    assert_eq!(ReflectUtil::type_name::<String>(), "alloc::string::String");
}

#[test]
fn type_name_vec() {
    let name = ReflectUtil::type_name::<Vec<i32>>();
    assert!(name.contains("Vec"));
}

// ── 类型比较 ──

#[test]
fn type_eq_same() {
    assert!(ReflectUtil::type_eq::<i32, i32>());
}

#[test]
fn type_eq_different() {
    assert!(!ReflectUtil::type_eq::<i32, f64>());
}

#[test]
fn type_eq_dyn_same() {
    let a = 42i32;
    let b = 100i32;
    assert!(ReflectUtil::type_eq_dyn(&a, &b));
}

#[test]
fn type_eq_dyn_different() {
    let a = 42i32;
    let b = 3.14f64;
    assert!(!ReflectUtil::type_eq_dyn(&a, &b));
}

// ── 类型转换 ──

#[test]
fn cast_downcast_success() {
    let value: Box<dyn std::any::Any> = Box::new(42i32);
    let result = ReflectUtil::cast_downcast::<i32>(value);
    assert!(result.is_ok());
    assert_eq!(*result.unwrap(), 42);
}

#[test]
fn cast_downcast_failure() {
    let value: Box<dyn std::any::Any> = Box::new(42i32);
    let result = ReflectUtil::cast_downcast::<f64>(value);
    assert!(result.is_err());
}

#[test]
fn cast_ref_success() {
    let value = 42i32;
    let result = ReflectUtil::cast_ref::<i32>(&value);
    assert_eq!(result, Some(&42));
}

#[test]
fn cast_ref_failure() {
    let value = 42i32;
    let result = ReflectUtil::cast_ref::<f64>(&value);
    assert!(result.is_none());
}

// ── TypeId ──

#[test]
fn type_id_i32() {
    assert_eq!(ReflectUtil::type_id::<i32>(), TypeId::of::<i32>());
}

#[test]
fn type_id_different() {
    assert_ne!(ReflectUtil::type_id::<i32>(), ReflectUtil::type_id::<f64>());
}

// ── new_instance ──

#[test]
fn new_instance_i32() {
    let val: i32 = ReflectUtil::new_instance();
    assert_eq!(val, 0);
}

#[test]
fn new_instance_string() {
    let val: String = ReflectUtil::new_instance();
    assert!(val.is_empty());
}

#[test]
fn new_instance_vec() {
    let val: Vec<i32> = ReflectUtil::new_instance();
    assert!(val.is_empty());
}

// ── Map 操作 ──

#[test]
fn get_string_from_map() {
    let mut map: HashMap<String, Box<dyn std::any::Any>> = HashMap::new();
    map.insert("name".to_string(), Box::new("Alice".to_string()));
    assert_eq!(
        ReflectUtil::get_string(&map, "name"),
        Some("Alice".to_string())
    );
}

#[test]
fn get_string_missing() {
    let map: HashMap<String, Box<dyn std::any::Any>> = HashMap::new();
    assert_eq!(ReflectUtil::get_string(&map, "name"), None);
}

#[test]
fn get_i64_from_map() {
    let mut map: HashMap<String, Box<dyn std::any::Any>> = HashMap::new();
    map.insert("age".to_string(), Box::new(30i64));
    assert_eq!(ReflectUtil::get_i64(&map, "age"), Some(30));
}

#[test]
fn get_i64_from_i32() {
    let mut map: HashMap<String, Box<dyn std::any::Any>> = HashMap::new();
    map.insert("age".to_string(), Box::new(30i32));
    assert_eq!(ReflectUtil::get_i64(&map, "age"), Some(30));
}

#[test]
fn get_f64_from_map() {
    let mut map: HashMap<String, Box<dyn std::any::Any>> = HashMap::new();
    map.insert("score".to_string(), Box::new(95.5f64));
    assert_eq!(ReflectUtil::get_f64(&map, "score"), Some(95.5));
}

#[test]
fn get_bool_from_map() {
    let mut map: HashMap<String, Box<dyn std::any::Any>> = HashMap::new();
    map.insert("active".to_string(), Box::new(true));
    assert_eq!(ReflectUtil::get_bool(&map, "active"), Some(true));
}

// ── 扩展测试 ──

#[test]
fn is_wrapper_type_i32() {
    assert!(ReflectUtil::is_wrapper_type::<i32>());
}

#[test]
fn is_wrapper_type_string() {
    assert!(!ReflectUtil::is_wrapper_type::<String>());
}

#[test]
fn convert_to_string_i32() {
    let val = 42i32;
    assert_eq!(ReflectUtil::convert_to_string(&val), Some("42".to_string()));
}

#[test]
fn convert_to_string_string() {
    let val = "hello".to_string();
    assert_eq!(
        ReflectUtil::convert_to_string(&val),
        Some("hello".to_string())
    );
}

#[test]
fn convert_to_string_bool() {
    let val = true;
    assert_eq!(
        ReflectUtil::convert_to_string(&val),
        Some("true".to_string())
    );
}

#[test]
fn set_and_get_field() {
    let mut map: HashMap<String, Box<dyn std::any::Any>> = HashMap::new();
    ReflectUtil::set_field_in_map(&mut map, "name", Box::new("Alice".to_string()));
    let field = ReflectUtil::get_field_from_map(&map, "name");
    assert!(field.is_some());
}

#[test]
fn is_option_true() {
    assert!(ReflectUtil::is_option::<Option<i32>>());
}

#[test]
fn is_option_false() {
    assert!(!ReflectUtil::is_option::<i32>());
}

#[test]
fn is_result_true() {
    assert!(ReflectUtil::is_result::<Result<i32, String>>());
}

#[test]
fn is_result_false() {
    assert!(!ReflectUtil::is_result::<i32>());
}

#[test]
fn default_values() {
    assert_eq!(ReflectUtil::default_value_i32(), 0);
    assert_eq!(ReflectUtil::default_value_i64(), 0);
    assert_eq!(ReflectUtil::default_value_f64(), 0.0);
    assert_eq!(ReflectUtil::default_value_bool(), false);
    assert_eq!(ReflectUtil::default_value_string(), "");
}

// ── Hutool TEST parity gap wave ──
// ── Hutool ReflectUtilTest remaining gaps ──

/// 对齐 Java: `ReflectUtilTest.getMethodsTest()`
#[test]
#[ignore = "planned: ReflectUtil 搁置（Rust 无对等运行时反射）"]
fn get_methods_test() {}

/// 对齐 Java: `ReflectUtilTest.getMethodTest()`
#[test]
#[ignore = "planned: ReflectUtil 搁置（Rust 无对等运行时反射）"]
fn get_method_test() {}

/// 对齐 Java: `ReflectUtilTest.getMethodIgnoreCaseTest()`
#[test]
#[ignore = "planned: ReflectUtil 搁置（Rust 无对等运行时反射）"]
fn get_method_ignore_case_test() {}

/// 对齐 Java: `ReflectUtilTest.getFieldTest()`
#[test]
#[ignore = "planned: ReflectUtil 搁置（Rust 无对等运行时反射）"]
fn get_field_test() {}

/// 对齐 Java: `ReflectUtilTest.getFieldMapTest()`
#[test]
#[ignore = "planned: ReflectUtil 搁置（Rust 无对等运行时反射）"]
fn get_field_map_test() {}

/// 对齐 Java: `ReflectUtilTest.getFieldsTest()`
#[test]
#[ignore = "planned: ReflectUtil 搁置（Rust 无对等运行时反射）"]
fn get_fields_test() {}

/// 对齐 Java: `ReflectUtilTest.setFieldTest()`
#[test]
#[ignore = "planned: ReflectUtil 搁置（Rust 无对等运行时反射）"]
fn set_field_test() {}

/// 对齐 Java: `ReflectUtilTest.invokeTest()`
#[test]
#[ignore = "planned: ReflectUtil 搁置（Rust 无对等运行时反射）"]
fn invoke_test() {}

/// 对齐 Java: `ReflectUtilTest.invokeMethodTest()`
#[test]
#[ignore = "planned: ReflectUtil 搁置（Rust 无对等运行时反射）"]
fn invoke_method_test() {}

/// 对齐 Java: `ReflectUtilTest.invokeMethodWithParamConvertTest()`
#[test]
#[ignore = "planned: ReflectUtil 搁置（Rust 无对等运行时反射）"]
fn invoke_method_with_param_convert_test() {}

/// 对齐 Java: `ReflectUtilTest.invokeMethodWithParamConvertFailedTest()`
#[test]
#[ignore = "planned: ReflectUtil 搁置（Rust 无对等运行时反射）"]
fn invoke_method_with_param_convert_failed_test() {}

/// 对齐 Java: `ReflectUtilTest.noneStaticInnerClassTest()`
#[test]
#[ignore = "planned: ReflectUtil 搁置（Rust 无对等运行时反射）"]
fn none_static_inner_class_test() {}

/// 对齐 Java: `ReflectUtilTest.getMethodBenchTest()`
#[test]
#[ignore = "planned: ReflectUtil 搁置（Rust 无对等运行时反射）"]
fn get_method_bench_test() {}

/// 对齐 Java: `ReflectUtilTest.getMethodsFromClassExtends()`
#[test]
#[ignore = "planned: ReflectUtil 搁置（Rust 无对等运行时反射）"]
fn get_methods_from_class_extends() {}

/// 对齐 Java: `ReflectUtilTest.getMethodsFromInterfaceTest()`
#[test]
#[ignore = "planned: ReflectUtil 搁置（Rust 无对等运行时反射）"]
fn get_methods_from_interface_test() {}

/// 对齐 Java: `ReflectUtilTest.newInstanceIfPossibleTest()`
#[test]
fn new_instance_if_possible_test() {
    let v: i32 = ReflectUtil::new_instance();
    assert_eq!(v, 0);
}

/// 对齐 Java: `ReflectUtilTest.setFieldValueWithFinalTest()`
#[test]
#[ignore = "planned: ReflectUtil 搁置（Rust 无对等运行时反射）"]
fn set_field_value_with_final_test() {}

/// 对齐 Java: `ReflectUtilTest.issue2625Test()`
#[test]
#[ignore = "planned: ReflectUtil 搁置（Rust 无对等运行时反射）"]
fn issue2625_test() {}

/// 对齐 Java: `ReflectUtilTest.newInstanceIfPossibleTest2()`
#[test]
#[ignore = "planned: ReflectUtil 搁置（Rust 无对等运行时反射）"]
fn new_instance_if_possible_test2() {}

/// 对齐 Java: `ReflectUtilTest.testGetMethodWithCache()`
#[test]
#[ignore = "planned: ReflectUtil 搁置（Rust 无对等运行时反射）"]
fn test_get_method_with_cache() {}
