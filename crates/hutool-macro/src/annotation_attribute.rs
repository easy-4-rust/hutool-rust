//! 对齐: `cn.hutool.core.annotation.AnnotationAttribute`

use std::sync::Arc;

use super::mirror::{
    AnnotationMirror, AnnotationTypeName, AnnotationValue, AttributeRef, ValueKind,
};

/// 对齐 Java interface: `cn.hutool.core.annotation.AnnotationAttribute`
pub trait AnnotationAttribute: Send + Sync {
    /// 实现类型名，用于 parity 测试中的 `getClass()` 断言。
    fn impl_type_name(&self) -> &'static str;

    /// 获取注解对象。
    fn get_annotation(&self) -> Arc<AnnotationMirror>;

    /// 获取属性引用（对齐 Java `Method getAttribute()`）。
    fn get_attribute(&self) -> AttributeRef;

    /// 获取声明属性的注解类型。
    fn get_annotation_type(&self) -> AnnotationTypeName {
        self.get_attribute().annotation_type
    }

    /// 获取属性名称。
    fn get_attribute_name(&self) -> String {
        self.get_attribute().name.clone()
    }

    /// 获取属性值。
    fn get_value(&self) -> AnnotationValue;

    /// 该注解属性的值是否等于默认值。
    fn is_value_equivalent_to_default_value(&self) -> bool;

    /// 获取属性类型。
    fn get_attribute_type(&self) -> ValueKind;

    /// 获取属性上的元注解。
    fn get_meta_annotation(&self, type_name: AnnotationTypeName) -> Option<Arc<AnnotationMirror>>;

    /// 当前注解属性是否已经被包装。
    fn is_wrapped(&self) -> bool {
        false
    }

    /// 尝试转为 WrappedAnnotationAttribute。
    fn as_wrapped(
        &self,
    ) -> Option<&dyn super::wrapped_annotation_attribute::WrappedAnnotationAttribute> {
        None
    }
}

/// 从 trait 对象读取字符串属性值。
pub fn attribute_string(attr: &dyn AnnotationAttribute) -> Option<String> {
    match attr.get_value() {
        AnnotationValue::String(s) => Some(s),
        _ => None,
    }
}
