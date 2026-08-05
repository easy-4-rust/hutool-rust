//! 对齐: `cn.hutool.core.bean.copier.IJSONTypeConverter`
//! 来源: hutool-core/src/main/java/cn/hutool/core/bean/copier/IJSONTypeConverter.java
//!
//! 中文说明: JSON 自定义转换扩展接口,因 core 模块无法直接调用 json 模块而创建,
//! 使用此接口避免使用反射调用 toBean 方法而性能太差。

#![allow(dead_code, unused_variables, clippy::new_without_default)]

use std::any::Any;

/// 对齐 Java interface: `cn.hutool.core.bean.copier.IJSONTypeConverter`
///
/// 中文说明: Java 接口在 Rust 中通过 trait dispatch 表达。
/// Rust 版对外暴露的 key 是 `ValueType` (`&'static str`),与 Java `Type`
/// 不等价;实现者负责自行把字符串映射到具体目标类型并构造对象。
pub trait IJSONTypeConverter: Any + Send + Sync {
    /// 对齐 Java: `<T> T toBean(Type type)`
    ///
    /// 参数 `value_type` 是 Java `Type` 在 Rust 中的字符串表示(实现约定)。
    /// 返回值装箱为 [`Box<dyn Any>`],调用方负责 downcast 到具体类型。
    fn to_beans(&self, value_type: &str) -> Box<dyn Any>;

    /// 对齐 Java: `Object value` — 即 `IJSONTypeConverter` 自身实例可被复制转换。
    /// Rust 版提供辅助:对同一 type 再做一次 `toBean` 调用。
    fn convert(&self, value_type: &str) -> Box<dyn Any> {
        self.to_beans(value_type)
    }
}

/// 类型擦除的 JSON 转换器:把任意 `to_beans` 调用转写到内嵌 Box<dyn Any>。
pub struct BoxedJSONTypeConverter {
    inner: Box<dyn IJSONTypeConverter>,
}

impl BoxedJSONTypeConverter {
    /// 包装任意 [`IJSONTypeConverter`] 实现。
    pub fn new(inner: Box<dyn IJSONTypeConverter>) -> Self {
        Self { inner }
    }

    /// 委托给内部实现做转换。
    pub fn to_beans(&self, value_type: &str) -> Box<dyn Any> {
        self.inner.to_beans(value_type)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct StringConverter;
    impl IJSONTypeConverter for StringConverter {
        fn to_beans(&self, _value_type: &str) -> Box<dyn Any> {
            Box::new("hello".to_string())
        }
    }

    #[test]
    fn ijson_type_converter_basic_dispatch() {
        let boxed = BoxedJSONTypeConverter::new(Box::new(StringConverter));
        let any = boxed.to_beans("java.lang.String");
        let downcast = any.downcast_ref::<String>().expect("must be String");
        assert_eq!(downcast, "hello");
    }

    #[test]
    fn ijson_type_converter_default_convert() {
        struct IntConverter;
        impl IJSONTypeConverter for IntConverter {
            fn to_beans(&self, _vt: &str) -> Box<dyn Any> {
                Box::new(42i32)
            }
        }
        let c = IntConverter;
        let v = IJSONTypeConverter::convert(&c, "int");
        assert_eq!(*v.downcast_ref::<i32>().unwrap(), 42);
    }
}
