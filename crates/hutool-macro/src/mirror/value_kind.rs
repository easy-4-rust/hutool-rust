//! 结构化注解模型核心类型，对齐 Java `Annotation` / `AnnotatedElement` 语义。
//!
//! Rust 无 JVM 运行时注解，通过 [`AnnotationMirror`] + [`ElementHandle`] 表达注解实例与被注解元素。

/// 属性值类型种类，对齐 Java 注解属性返回类型。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ValueKind {
    /// void。
    Void,
    /// boolean。
    Bool,
    /// int。
    I32,
    /// long。
    I64,
    /// double。
    F64,
    /// String。
    String,
    /// Class。
    Class,
    /// 注解类型。
    Annotation,
    /// 数组。
    Array,
}
