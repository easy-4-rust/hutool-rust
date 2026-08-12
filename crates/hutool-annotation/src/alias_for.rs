//! 对齐: `cn.hutool.core.annotation.AliasFor`
//! 来源: hutool-core/src/main/java/cn/hutool/core/annotation/AliasFor.java

/// `@AliasFor` 元注解类型名。
pub const TYPE_NAME: &str = "cn.hutool.core.annotation.AliasFor";

/// 对齐 Java 注解: `AliasFor` — 显式别名关系描述（无 JVM 反射）。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AliasFor {
    /// 注解类型名。
    pub annotation: String,
    /// 属性名。
    pub attribute: String,
}

impl AliasFor {
    /// 创建别名关系。
    pub fn new(annotation: impl Into<String>, attribute: impl Into<String>) -> Self {
        Self {
            annotation: annotation.into(),
            attribute: attribute.into(),
        }
    }

    /// 返回注解类型全名。
    pub fn type_name() -> &'static str {
        TYPE_NAME
    }
}
