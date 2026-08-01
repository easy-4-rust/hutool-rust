//! type_util parity tests
//! 对齐: hutool-core TypeUtilTest

use hutool_core::TypeUtil;

// ── 类型名称 ──

#[test]
fn type_name_i32() {
    assert_eq!(TypeUtil::type_name::<i32>(), "i32");
}

#[test]
fn type_name_i64() {
    assert_eq!(TypeUtil::type_name::<i64>(), "i64");
}

#[test]
fn type_name_f64() {
    assert_eq!(TypeUtil::type_name::<f64>(), "f64");
}

#[test]
fn type_name_bool() {
    assert_eq!(TypeUtil::type_name::<bool>(), "bool");
}

#[test]
fn type_name_string() {
    assert_eq!(TypeUtil::type_name::<String>(), "alloc::string::String");
}

#[test]
fn type_name_vec() {
    let name = TypeUtil::type_name::<Vec<i32>>();
    assert!(name.contains("Vec"));
}

#[test]
fn type_name_of_value() {
    let val = 42i32;
    assert_eq!(TypeUtil::type_name_of(&val), "i32");
}

// ── 类型比较 ──

#[test]
fn is_assignable_from_same() {
    assert!(TypeUtil::is_assignable_from::<i32, i32>());
}

#[test]
fn is_assignable_from_different() {
    assert!(!TypeUtil::is_assignable_from::<i32, f64>());
}

#[test]
fn is_instance_of_i32() {
    let val = 42i32;
    assert!(TypeUtil::is_instance_of::<i32>(&val));
    assert!(!TypeUtil::is_instance_of::<f64>(&val));
}

// ── 基础类型判断 ──

#[test]
fn is_basic_type_integers() {
    assert!(TypeUtil::is_basic_type::<i8>());
    assert!(TypeUtil::is_basic_type::<i16>());
    assert!(TypeUtil::is_basic_type::<i32>());
    assert!(TypeUtil::is_basic_type::<i64>());
    assert!(TypeUtil::is_basic_type::<u8>());
    assert!(TypeUtil::is_basic_type::<u16>());
    assert!(TypeUtil::is_basic_type::<u32>());
    assert!(TypeUtil::is_basic_type::<u64>());
}

#[test]
fn is_basic_type_floats() {
    assert!(TypeUtil::is_basic_type::<f32>());
    assert!(TypeUtil::is_basic_type::<f64>());
}

#[test]
fn is_basic_type_bool_char() {
    assert!(TypeUtil::is_basic_type::<bool>());
    assert!(TypeUtil::is_basic_type::<char>());
}

#[test]
fn is_basic_type_not_string() {
    assert!(!TypeUtil::is_basic_type::<String>());
}

#[test]
fn is_basic_type_not_vec() {
    assert!(!TypeUtil::is_basic_type::<Vec<i32>>());
}

#[test]
fn is_simple_type_includes_string() {
    assert!(TypeUtil::is_simple_type::<String>());
    assert!(TypeUtil::is_simple_type::<i32>());
    assert!(!TypeUtil::is_simple_type::<Vec<i32>>());
}

// ── 数值类型判断 ──

#[test]
fn is_number_integers() {
    assert!(TypeUtil::is_number::<i32>());
    assert!(TypeUtil::is_number::<i64>());
    assert!(TypeUtil::is_number::<u32>());
    assert!(TypeUtil::is_number::<u64>());
}

#[test]
fn is_number_floats() {
    assert!(TypeUtil::is_number::<f32>());
    assert!(TypeUtil::is_number::<f64>());
}

#[test]
fn is_number_not_bool() {
    assert!(!TypeUtil::is_number::<bool>());
}

#[test]
fn is_number_not_string() {
    assert!(!TypeUtil::is_number::<String>());
}

#[test]
fn is_integer_i32() {
    assert!(TypeUtil::is_integer::<i32>());
    assert!(TypeUtil::is_integer::<i64>());
    assert!(TypeUtil::is_integer::<u32>());
}

#[test]
fn is_integer_not_float() {
    assert!(!TypeUtil::is_integer::<f32>());
    assert!(!TypeUtil::is_integer::<f64>());
}

#[test]
fn is_float_f32() {
    assert!(TypeUtil::is_float::<f32>());
    assert!(TypeUtil::is_float::<f64>());
}

#[test]
fn is_float_not_int() {
    assert!(!TypeUtil::is_float::<i32>());
    assert!(!TypeUtil::is_float::<u64>());
}

// ── 容器类型判断 ──

#[test]
fn is_collection_vec() {
    assert!(TypeUtil::is_collection::<Vec<i32>>());
}

#[test]
fn is_collection_not_i32() {
    assert!(!TypeUtil::is_collection::<i32>());
}

#[test]
fn is_map_hashmap() {
    assert!(TypeUtil::is_map::<std::collections::HashMap<String, i32>>());
}

#[test]
fn is_map_not_i32() {
    assert!(!TypeUtil::is_map::<i32>());
}

// ── Hutool TEST parity gap wave ──
// ── Hutool TypeUtilTest remaining gaps ──

struct Level1<T> {
    #[allow(dead_code)] // 对齐 Java 测试字段，仅用于类型推断，预留
    id: T,
}

struct Level2<E> {
    #[allow(dead_code)] // 对齐 Java 测试字段，仅用于类型推断，预留
    id: i64,
    _marker: std::marker::PhantomData<E>,
}

struct Level3 {
    #[allow(dead_code)] // 对齐 Java 测试字段，仅用于类型推断，预留
    id: i64,
}

struct GenericArrayEle {
    #[allow(dead_code)] // 对齐 Java 测试字段，仅用于类型推断，预留
    uid: i64,
}

/// 对齐 Java: `TypeUtilTest.getEleTypeTest()`
#[test]
fn get_ele_type_test() {
    assert!(TypeUtil::type_name::<Vec<String>>().contains("Vec"));
    assert_eq!(TypeUtil::ele_type_name::<String>(), TypeUtil::type_name::<String>());
}

/// 对齐 Java: `TypeUtilTest.getParamTypeTest()`
#[test]
fn get_param_type_test() {
    assert_eq!(TypeUtil::param_type_name::<i32>(), TypeUtil::type_name::<i32>());
    assert_eq!(TypeUtil::class_type_name::<i32>(), TypeUtil::type_name::<i32>());
}

/// 对齐 Java: `TypeUtilTest.getClasses()`
#[test]
fn get_classes() {
    assert!(TypeUtil::class_type_name::<Level1<i64>>().contains("Level1"));
    assert_eq!(TypeUtil::class_type_name::<Level3>(), TypeUtil::type_name::<Level3>());
}

/// 对齐 Java: `TypeUtilTest.getClassForGenericArrayTypeTest()`
#[test]
fn get_class_for_generic_array_type_test() {
    let component = TypeUtil::generic_array_component_name();
    assert!(component.contains("Any"));
}

/// 对齐 Java: `TypeUtilTest.getClassForParameterizedArrayTypeTest()`
#[test]
fn get_class_for_parameterized_array_type_test() {
    let component = TypeUtil::parameterized_array_component_name::<Vec<String>>();
    assert!(component.contains("Vec"));
    assert!(TypeUtil::is_array_type::<[Vec<String>; 1]>());
}

/// 对齐 Java: `TypeUtilTest.getTypeArgumentTest()`
#[test]
fn get_type_argument_test() {
    assert_eq!(TypeUtil::type_argument_name::<String>(), TypeUtil::type_name::<String>());
}

/// 对齐 Java: `TypeUtilTest.getActualTypesTest()`
#[test]
fn get_actual_types_test() {
    assert_eq!(TypeUtil::actual_type_name::<i64>(), TypeUtil::type_name::<i64>());
    let _level: Level2<Level3> = Level2 {
        id: 1,
        _marker: std::marker::PhantomData,
    };
    assert!(TypeUtil::class_type_name::<Level2<Level3>>().contains("Level2"));
}

/// 对齐 Java: `TypeUtilTest.getActualTypeForGenericArrayTest()`
#[test]
fn get_actual_type_for_generic_array_test() {
    let array_type = TypeUtil::actual_array_type_name::<GenericArrayEle>();
    assert_eq!(array_type, format!("[{}]", TypeUtil::short_type_name(TypeUtil::type_name::<GenericArrayEle>())));
}
