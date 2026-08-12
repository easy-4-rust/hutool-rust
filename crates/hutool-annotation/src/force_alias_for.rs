//! 对齐: `cn.hutool.core.annotation.ForceAliasFor`
//! 来源: hutool-core/src/main/java/cn/hutool/core/annotation/ForceAliasFor.java

/// `@ForceAliasFor` 元注解类型名。
pub const TYPE_NAME: &str = "cn.hutool.core.annotation.ForceAliasFor";

/// 对齐 Java 注解: 强制别名关系描述（无 JVM 反射）。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ForceAliasFor {
    /// 注解类型名。
    pub annotation: String,
    /// 属性名。
    pub attribute: String,
}

impl ForceAliasFor {
    /// 创建强制别名。
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
