//! 对齐: `cn.hutool.core.bean.copier.ValueProvider`
//! 来源: hutool-core/src/main/java/cn/hutool/core/bean/copier/ValueProvider.java
//!
//! 中文说明: 值提供者,用于提供 Bean 注入时参数对应值的抽象接口。
//! 继承或匿名实例化此接口,在 Bean 注入过程中,Bean 获得字段名,
//! 通过外部方式根据这个字段名查找相应的字段值,然后注入 Bean。

#![allow(dead_code, unused_variables, clippy::new_without_default)]

/// 对齐 Java interface: `cn.hutool.core.bean.copier.ValueProvider<T>`
///
/// 中文说明: KEY 类型,一般情况下为 [`String`]。本实现固定为 [`String`]
/// (Java 侧 hutool-core 几乎所有调用方都把 `T = String`)。
pub trait ValueProvider: Send + Sync {
    /// 对齐 Java: `Object value(T key, Type valueType)`
    ///
    /// 中文说明: 获取值,返回值一般需要匹配被注入类型,
    /// 如果不匹配会调用默认转换 `Convert#convert(Type, Object)` 实现转换。
    ///
    /// - `key`: Bean 对象中参数名
    /// - `value_type`: 被注入的值的类型,Rust 中以字符串表示(Java `Type` 的对等)
    fn value(&self, key: &str, value_type: &str) -> Option<ValueKind>;

    /// 对齐 Java: `boolean containsKey(T key)`
    ///
    /// 中文说明: 是否包含指定 KEY,如果不包含则忽略注入。
    /// 此接口方法单独需要实现的意义在于:有些值提供者(比如 Map)
    /// key 是存在的,但是 value 为 null,此时如果需要注入这个 null,
    /// 需要根据此方法判断。
    fn contains_key(&self, key: &str) -> bool;

    /// 对齐 Java 派生:返回所有可枚举的 KEY,默认实现遍历 `contains_key` 不可能,
    /// 因此 trait 不强制要求,提供默认空实现。
    fn keys(&self) -> Vec<String> {
        Vec::new()
    }
}

/// 对齐 Java `Object` 在 Rust 中的轻量枚举。
///
/// 真实 `BeanCopier` 路径只关心 `ValueKind::Null / Bool / Int / String`
/// 这几种基本形态,所有非 None 返回都打包为 [`ValueKind::Other`]。
#[derive(Debug, Clone, PartialEq)]
pub enum ValueKind {
    /// 对齐 Java: `null`
    Null,
    /// 对齐 Java: `Boolean`
    Bool(bool),
    /// 对齐 Java: `Long` / `Integer` / `Number` 整数部分
    Int(i64),
    /// 对齐 Java: `Double` / `Float`
    Float(f64),
    /// 对齐 Java: `String`
    String(String),
    /// 对齐 Java: 其它任意 `Object` —— 装箱为字符串表示。
    Other(String),
}

impl ValueKind {
    /// 还原到 `serde_json::Value`,供 `CopyOptions` 转换器使用。
    #[cfg(feature = "default")]
    pub fn to_json(&self) -> serde_json::Value {
        use serde_json::Value;
        match self {
            ValueKind::Null => Value::Null,
            ValueKind::Bool(b) => Value::Bool(*b),
            ValueKind::Int(i) => Value::Number((*i).into()),
            ValueKind::Float(f) => serde_json::Number::from_f64(*f)
                .map(Value::Number)
                .unwrap_or(Value::Null),
            ValueKind::String(s) => Value::String(s.clone()),
            ValueKind::Other(s) => Value::String(s.clone()),
        }
    }

    /// 对齐 Java `Object == null` 判定。
    pub fn is_null(&self) -> bool {
        matches!(self, ValueKind::Null)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn value_kind_null_is_null() {
        assert!(ValueKind::Null.is_null());
        assert!(!ValueKind::Int(0).is_null());
    }

    #[test]
    fn value_provider_default_keys_is_empty() {
        struct Empty;
        impl ValueProvider for Empty {
            fn value(&self, _k: &str, _t: &str) -> Option<ValueKind> {
                None
            }
            fn contains_key(&self, _k: &str) -> bool {
                false
            }
        }
        let p = Empty;
        assert!(p.keys().is_empty());
        assert!(!p.contains_key("x"));
    }
}