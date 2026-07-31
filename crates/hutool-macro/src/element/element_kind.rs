//! 被注解元素注册表，对齐 Java `AnnotatedElement` 及其层级结构。

/// 元素种类。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ElementKind {
    /// 类型。
    Type,
    /// 方法。
    Method,
    /// 字段。
    Field,
}
