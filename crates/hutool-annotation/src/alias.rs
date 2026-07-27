//! 对齐: `cn.hutool.core.annotation.Alias`
//! 来源: hutool-core/src/main/java/cn/hutool/core/annotation/Alias.java

/// `@Alias` 元注解类型名。
pub const TYPE_NAME: &str = "cn.hutool.core.annotation.Alias";

/// 对齐 Java 注解: `cn.hutool.core.annotation.Alias` — 非反射元数据描述符。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Alias {
    /// 别名目标属性名。
    pub value: String,
}

impl Alias {
    /// 创建别名标记。
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
        }
    }

    /// 返回注解类型全名。
    pub fn type_name() -> &'static str {
        TYPE_NAME
    }
}
