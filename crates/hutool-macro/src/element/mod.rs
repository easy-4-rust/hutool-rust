//! 被注解元素注册表，对齐 Java `AnnotatedElement` 及其层级结构。

use parking_lot::RwLock;

pub use crate::mirror::ElementHandle;

mod element_kind;
mod type_element;
mod method_element;
mod field_element;
mod annotated_element;
mod annotation_registry;
mod type_builder;
mod method_builder;
mod field_builder;

pub use element_kind::ElementKind;
pub use type_element::TypeElement;
pub use method_element::MethodElement;
pub use field_element::FieldElement;
pub use annotated_element::AnnotatedElement;
pub use annotation_registry::AnnotationRegistry;
pub use type_builder::TypeBuilder;
pub use method_builder::MethodBuilder;
pub use field_builder::FieldBuilder;

static GLOBAL_REGISTRY: std::sync::OnceLock<RwLock<AnnotationRegistry>> = std::sync::OnceLock::new();

/// 获取全局注册表（惰性初始化）。
pub fn global_registry() -> &'static RwLock<AnnotationRegistry> {
    GLOBAL_REGISTRY.get_or_init(|| RwLock::new(AnnotationRegistry::new()))
}
