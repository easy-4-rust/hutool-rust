//! 对齐: `cn.hutool.core.annotation.PropIgnore`
//! 来源: hutool-core/src/main/java/cn/hutool/core/annotation/PropIgnore.java
//!
//! Java `@PropIgnore` 在 Rust 侧以元数据标记 + 字段名约定表达（无反射代理）。

/// `@PropIgnore` 元注解类型名。
pub const TYPE_NAME: &str = "cn.hutool.core.annotation.PropIgnore";

/// 对齐 Java 类: `cn.hutool.core.annotation.PropIgnore`
///
/// 作为字段忽略标记的元数据描述符；实际忽略逻辑由序列化/拷贝调用方解释。
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct PropIgnore;

impl PropIgnore {
    /// 返回注解类型全名。
    pub fn type_name() -> &'static str {
        TYPE_NAME
    }

    /// 判断属性名是否应按约定忽略（以 `_` 开头或显式列表）。
    pub fn should_ignore(field_name: &str, explicit: &[&str]) -> bool {
        field_name.starts_with('_') || explicit.contains(&field_name)
    }
}
