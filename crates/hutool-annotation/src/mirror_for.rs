//! 对齐: `cn.hutool.core.annotation.MirrorFor`
//! 来源: hutool-core/src/main/java/cn/hutool/core/annotation/MirrorFor.java

/// `@MirrorFor` 元注解类型名。
pub const TYPE_NAME: &str = "cn.hutool.core.annotation.MirrorFor";

/// 对齐 Java 注解: 镜像属性关系（非反射元数据）。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MirrorFor {
    /// 注解类型名。
    pub annotation: String,
    /// 属性名。
    pub attribute: String,
}

impl MirrorFor {
    /// 创建镜像关系。
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
