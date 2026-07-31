//! 结构化注解模型核心类型，对齐 Java `Annotation` / `AnnotatedElement` 语义。
//!
//! 该模块类型与 hutool-macro 的 `mirror` 模块完全一致（迁移期统一类型），
//! 直接 re-export 避免双份类型导致 `AnnotationMirror`/`AnnotationValue` 歧义。

pub use hutool_macro::mirror::*;
