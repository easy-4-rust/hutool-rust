//! 对齐: `cn.hutool.core.util.TypeUtil`
//! 来源: hutool-core/src/main/java/cn/hutool/core/util/TypeUtil.java
//!
//! Rust 版本提供类型操作的 idiomatic 实现。

use std::any::{Any, TypeId};

/// 对齐 Java: `cn.hutool.core.util.TypeUtil`
#[derive(Debug, Clone, Copy, Default)]
pub struct TypeUtil;

impl TypeUtil {
    // ── 类型名称 ──

    /// 对齐 Java: `TypeUtil.getTypeName(Type)`
    pub fn type_name<T: 'static>() -> &'static str {
        std::any::type_name::<T>()
    }

    /// 对齐 Java: `TypeUtil.getClass(Object)`
    pub fn type_name_of<T: Any>(_value: &T) -> &'static str {
        std::any::type_name::<T>()
    }

    // ── 类型比较 ──

    /// 对齐 Java: `TypeUtil.isAssignableFrom(Class, Class)`
    pub fn is_assignable_from<T: 'static, U: 'static>() -> bool {
        TypeId::of::<T>() == TypeId::of::<U>()
    }

    /// 对齐 Java: `TypeUtil.isInstance(Object, Class)`
    pub fn is_instance_of<T: 'static>(value: &dyn Any) -> bool {
        value.is::<T>()
    }

    // ── 基础类型判断 ──

    /// 对齐 Java: `TypeUtil.isSimpleValueType(Class)`
    pub fn is_simple_type<T: 'static>() -> bool {
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
            || type_id == TypeId::of::<String>()
    }

    /// 对齐 Java: `TypeUtil.isBasicType(Class)`
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

    // ── 数值类型判断 ──

    /// 对齐 Java: `TypeUtil.isNumber(Class)`
    pub fn is_number<T: 'static>() -> bool {
        let type_id = TypeId::of::<T>();
        type_id == TypeId::of::<i8>()
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
    }

    /// 对齐 Java: `TypeUtil.isInteger(Class)`
    pub fn is_integer<T: 'static>() -> bool {
        let type_id = TypeId::of::<T>();
        type_id == TypeId::of::<i8>()
            || type_id == TypeId::of::<i16>()
            || type_id == TypeId::of::<i32>()
            || type_id == TypeId::of::<i64>()
            || type_id == TypeId::of::<i128>()
            || type_id == TypeId::of::<u8>()
            || type_id == TypeId::of::<u16>()
            || type_id == TypeId::of::<u32>()
            || type_id == TypeId::of::<u64>()
            || type_id == TypeId::of::<u128>()
    }

    /// 对齐 Java: `TypeUtil.isFloat(Class)`
    pub fn is_float<T: 'static>() -> bool {
        let type_id = TypeId::of::<T>();
        type_id == TypeId::of::<f32>() || type_id == TypeId::of::<f64>()
    }

    // ── 容器类型判断 ──

    /// 对齐 Java: `TypeUtil.isCollection(Class)`
    pub fn is_collection<T: 'static>() -> bool {
        // 简化判断：检查常见集合类型
        let type_name = std::any::type_name::<T>();
        type_name.contains("Vec")
            || type_name.contains("VecDeque")
            || type_name.contains("LinkedList")
    }

    /// 对齐 Java: `TypeUtil.isMap(Class)`
    pub fn is_map<T: 'static>() -> bool {
        let type_name = std::any::type_name::<T>();
        type_name.contains("HashMap")
            || type_name.contains("BTreeMap")
            || type_name.contains("IndexMap")
    }

    // ── 泛型/数组类型（Rust TypeId 等价，非 JVM 反射） ──

    /// 对齐 Java: `TypeUtil.getTypeArgument` — 元素/泛型参数类型名
    pub fn ele_type_name<E: 'static>() -> &'static str {
        std::any::type_name::<E>()
    }

    /// 对齐 Java: `TypeUtil.getParamType`
    pub fn param_type_name<P: 'static>() -> &'static str {
        std::any::type_name::<P>()
    }

    /// 对齐 Java: `TypeUtil.getReturnType` / `TypeUtil.getClass`
    pub fn class_type_name<T: 'static>() -> &'static str {
        std::any::type_name::<T>()
    }

    /// 对齐 Java: 未绑定泛型数组 `T[]` 的组件类型（等价 `Object`）
    pub fn generic_array_component_name() -> &'static str {
        std::any::type_name::<dyn Any>()
    }

    /// 对齐 Java: 参数化数组组件类型，如 `List<String>[]` 的 `List`
    pub fn parameterized_array_component_name<T: 'static>() -> &'static str {
        std::any::type_name::<T>()
    }

    /// 对齐 Java: `TypeUtil.getTypeArgument`（接口实现场景）
    pub fn type_argument_name<T: 'static>() -> &'static str {
        std::any::type_name::<T>()
    }

    /// 对齐 Java: `TypeUtil.getActualType`
    pub fn actual_type_name<T: 'static>() -> &'static str {
        std::any::type_name::<T>()
    }

    /// 对齐 Java: 泛型数组字段的实际组件数组类型名
    pub fn actual_array_type_name<E: 'static>() -> String {
        format!("[{}]", Self::short_type_name(std::any::type_name::<E>()))
    }

    /// 取 `::` 分隔的简短类型名
    pub fn short_type_name(full: &str) -> &str {
        full.rsplit("::").next().unwrap_or(full)
    }

    /// 判断 Rust 数组/slice 类型
    pub fn is_array_type<T: 'static>() -> bool {
        std::any::type_name::<T>().starts_with('[')
    }
}
