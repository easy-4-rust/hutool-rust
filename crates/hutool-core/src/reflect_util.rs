//! 对齐: `cn.hutool.core.util.ReflectUtil`
//! 来源: hutool-core/src/main/java/cn/hutool/core/util/ReflectUtil.java
//!
//! Rust 版本提供类型反射和运行时类型信息的 idiomatic 实现。

use std::any::{Any, TypeId};
use std::collections::HashMap;

/// 对齐 Java: `cn.hutool.core.util.ReflectUtil`
#[derive(Debug, Clone, Copy, Default)]
pub struct ReflectUtil;

impl ReflectUtil {
    // ── 类型判断 ──

    /// 对齐 Java: `ReflectUtil.isBasicType(Class)`
    pub fn is_basic_type<T: 'static>() -> bool {
        let type_id = TypeId::of::<T>();
        type_id == TypeId::of::<bool>()
            || type_id == TypeId::of::<i8>()
            || type_id == TypeId::of::<i16>()
            || type_id == TypeId::of::<i32>()
            || type_id == TypeId::of::<i64>()
            || type_id == TypeId::of::<i128>()
            || type_id == TypeId::of::<u8>()
            || type_id == TypeId::of::<u16>()
            || type_id == TypeId::of::<u32>()
            || type_id == TypeId::of::<u64>()
            || type_id == TypeId::of::<u128>()
            || type_id == TypeId::of::<f32>()
            || type_id == TypeId::of::<f64>()
            || type_id == TypeId::of::<char>()
    }

    /// 对齐 Java: `ReflectUtil.isBasicType(Object)`
    pub fn is_basic_type_val<T: Any>(value: &T) -> bool {
        Self::is_basic_type_dyn(value as &dyn Any)
    }

    /// 动态版本的类型判断
    pub fn is_basic_type_dyn(value: &dyn Any) -> bool {
        value.is::<bool>()
            || value.is::<i8>()
            || value.is::<i16>()
            || value.is::<i32>()
            || value.is::<i64>()
            || value.is::<i128>()
            || value.is::<u8>()
            || value.is::<u16>()
            || value.is::<u32>()
            || value.is::<u64>()
            || value.is::<u128>()
            || value.is::<f32>()
            || value.is::<f64>()
            || value.is::<char>()
    }

    /// 对齐 Java: `ReflectUtil.isSimpleValueType(Class)`
    pub fn is_simple_type<T: 'static>() -> bool {
        Self::is_basic_type::<T>()
            || TypeId::of::<T>() == TypeId::of::<String>()
            || TypeId::of::<T>() == TypeId::of::<&str>()
    }

    // ── 类型名称 ──

    /// 对齐 Java: `ReflectUtil.getClassName(Class)`
    pub fn type_name<T: 'static>() -> &'static str {
        std::any::type_name::<T>()
    }

    /// 对齐 Java: `ReflectUtil.getClassName(Object)`
    pub fn type_name_of<T: Any>(value: &T) -> &'static str {
        Self::type_name_dyn(value as &dyn Any)
    }

    /// 动态版本的类型名称
    pub fn type_name_dyn(_value: &dyn Any) -> &'static str {
        // dyn Any doesn't expose type_name directly in stable Rust
        // This is a limitation compared to Java reflection
        "dyn Any"
    }

    // ── 类型比较 ──

    /// 对齐 Java: `ReflectUtil.isAssignableFrom(Class, Class)`
    pub fn type_eq<A: 'static, B: 'static>() -> bool {
        TypeId::of::<A>() == TypeId::of::<B>()
    }

    /// 比较两个值的运行时类型是否相同
    pub fn type_eq_dyn(a: &dyn Any, b: &dyn Any) -> bool {
        a.type_id() == b.type_id()
    }

    // ── 安全类型转换 ──

    /// 对齐 Java: `ReflectUtil.cast(Object, Class)`
    pub fn cast_downcast<T: 'static>(value: Box<dyn Any>) -> Result<Box<T>, Box<dyn Any>> {
        value.downcast::<T>()
    }

    /// 对齐 Java: `ReflectUtil.cast(Object, Class)` (引用版本)
    pub fn cast_ref<T: 'static>(value: &dyn Any) -> Option<&T> {
        value.downcast_ref::<T>()
    }

    // ── 字段/方法信息（简化版） ──

    /// 对齐 Java: `ReflectUtil.getFields(Class)`
    /// Rust 没有运行时反射获取字段列表，返回类型 ID 作为标识
    pub fn type_id<T: 'static>() -> TypeId {
        TypeId::of::<T>()
    }

    /// 对齐 Java: `ReflectUtil.newInstance(Class)`
    /// Rust 中需要 Default trait 来创建实例
    pub fn new_instance<T: Default>() -> T {
        T::default()
    }

    // ── Map 到结构体转换（简化版） ──

    /// 从 HashMap 获取值并尝试转换类型
    pub fn get_as<T: 'static + Clone>(map: &HashMap<String, Box<dyn Any>>, key: &str) -> Option<T> {
        map.get(key)?.downcast_ref::<T>().cloned()
    }

    /// 从 HashMap 获取字符串值
    pub fn get_string(map: &HashMap<String, Box<dyn Any>>, key: &str) -> Option<String> {
        if let Some(s) = map.get(key)?.downcast_ref::<String>() {
            return Some(s.clone());
        }
        if let Some(s) = map.get(key)?.downcast_ref::<&str>() {
            return Some(s.to_string());
        }
        None
    }

    /// 从 HashMap 获取 i64 值
    pub fn get_i64(map: &HashMap<String, Box<dyn Any>>, key: &str) -> Option<i64> {
        if let Some(v) = map.get(key)?.downcast_ref::<i64>() {
            return Some(*v);
        }
        if let Some(v) = map.get(key)?.downcast_ref::<i32>() {
            return Some(*v as i64);
        }
        None
    }

    /// 从 HashMap 获取 f64 值
    pub fn get_f64(map: &HashMap<String, Box<dyn Any>>, key: &str) -> Option<f64> {
        if let Some(v) = map.get(key)?.downcast_ref::<f64>() {
            return Some(*v);
        }
        if let Some(v) = map.get(key)?.downcast_ref::<f32>() {
            return Some(*v as f64);
        }
        None
    }

    /// 从 HashMap 获取 bool 值
    pub fn get_bool(map: &HashMap<String, Box<dyn Any>>, key: &str) -> Option<bool> {
        map.get(key)?.downcast_ref::<bool>().copied()
    }
}

// ── 扩展方法 ──
impl ReflectUtil {
    // ── 包装类型判断 ──

    /// 对齐 Java: `ReflectUtil.isWrapperType(Class)`
    pub fn is_wrapper_type<T: 'static>() -> bool {
        let type_id = TypeId::of::<T>();
        type_id == TypeId::of::<bool>()
            || type_id == TypeId::of::<i8>()
            || type_id == TypeId::of::<i16>()
            || type_id == TypeId::of::<i32>()
            || type_id == TypeId::of::<i64>()
            || type_id == TypeId::of::<u8>()
            || type_id == TypeId::of::<u16>()
            || type_id == TypeId::of::<u32>()
            || type_id == TypeId::of::<u64>()
            || type_id == TypeId::of::<f32>()
            || type_id == TypeId::of::<f64>()
            || type_id == TypeId::of::<char>()
    }

    // ── 数组/集合类型判断 ──

    /// 对齐 Java: `ReflectUtil.isArray(Class)`
    pub fn is_array<T: 'static>() -> bool {
        TypeId::of::<T>() == TypeId::of::<Vec<u8>>()  // 简化判断
    }

    /// 对齐 Java: `ReflectUtil.isCollection(Class)`
    pub fn is_collection<T: 'static>() -> bool {
        // 简化判断：检查常见集合类型
        let type_id = TypeId::of::<T>();
        type_id == TypeId::of::<Vec<u8>>()
            || type_id == TypeId::of::<Vec<i32>>()
            || type_id == TypeId::of::<Vec<String>>()
    }

    // ── 类型转换工具 ──

    /// 对齐 Java: `ReflectUtil.convert(Type, Object)`
    pub fn convert_to_string(value: &dyn Any) -> Option<String> {
        if let Some(s) = value.downcast_ref::<String>() {
            return Some(s.clone());
        }
        if let Some(s) = value.downcast_ref::<&str>() {
            return Some(s.to_string());
        }
        if let Some(i) = value.downcast_ref::<i32>() {
            return Some(i.to_string());
        }
        if let Some(i) = value.downcast_ref::<i64>() {
            return Some(i.to_string());
        }
        if let Some(f) = value.downcast_ref::<f64>() {
            return Some(f.to_string());
        }
        if let Some(b) = value.downcast_ref::<bool>() {
            return Some(b.to_string());
        }
        None
    }

    /// 对齐 Java: `ReflectUtil.getFieldValue(Object, String)`
    pub fn get_field_from_map<'a>(map: &'a HashMap<String, Box<dyn Any>>, field: &str) -> Option<&'a Box<dyn Any>> {
        map.get(field)
    }

    /// 对齐 Java: `ReflectUtil.setFieldValue(Object, String, Object)`
    pub fn set_field_in_map(map: &mut HashMap<String, Box<dyn Any>>, field: &str, value: Box<dyn Any>) {
        map.insert(field.to_string(), value);
    }

    // ── 默认值 ──

    /// 对齐 Java: `ReflectUtil.getDefaultValue(Class)`
    pub fn default_value_i32() -> i32 { 0 }
    /// i64 默认值。
    pub fn default_value_i64() -> i64 { 0 }
    /// f64 默认值。
    pub fn default_value_f64() -> f64 { 0.0 }
    /// bool 默认值。
    pub fn default_value_bool() -> bool { false }
    /// String 默认值。
    pub fn default_value_string() -> String { String::new() }

    // ── 泛型工具 ──

    /// 对齐 Java: `ReflectUtil.getParameterizedTypes(Field)`
    pub fn is_option<T: 'static>() -> bool {
        // 检查是否为 Option 类型（简化判断）
        std::any::type_name::<T>().starts_with("core::option::Option")
    }

    /// 对齐 Java: `ReflectUtil.isResult(Type)`
    pub fn is_result<T: 'static>() -> bool {
        std::any::type_name::<T>().starts_with("core::result::Result")
    }
}
