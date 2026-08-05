//! 对齐: `cn.hutool.core.annotation.AbstractWrappedAnnotationAttribute`

use std::sync::Arc;

use super::annotation_attribute::AnnotationAttribute;
use super::mirror::{
    AnnotationMirror, AnnotationTypeName, AnnotationValue, AttributeRef, ValueKind,
};
use super::wrapped_annotation_attribute::WrappedAnnotationAttribute;

/// 对齐 Java 抽象类: `cn.hutool.core.annotation.AbstractWrappedAnnotationAttribute`
pub struct AbstractWrappedAnnotationAttribute {
    original: Arc<dyn AnnotationAttribute>,
    linked: Arc<dyn AnnotationAttribute>,
    value_fn: fn(&Arc<dyn AnnotationAttribute>, &Arc<dyn AnnotationAttribute>) -> AnnotationValue,
    default_fn: fn(&Arc<dyn AnnotationAttribute>, &Arc<dyn AnnotationAttribute>) -> bool,
}

impl AbstractWrappedAnnotationAttribute {
    /// 构造包装属性。
    pub fn new(
        original: Arc<dyn AnnotationAttribute>,
        linked: Arc<dyn AnnotationAttribute>,
        value_fn: fn(
            &Arc<dyn AnnotationAttribute>,
            &Arc<dyn AnnotationAttribute>,
        ) -> AnnotationValue,
        default_fn: fn(&Arc<dyn AnnotationAttribute>, &Arc<dyn AnnotationAttribute>) -> bool,
    ) -> Arc<Self> {
        Arc::new(Self {
            original,
            linked,
            value_fn,
            default_fn,
        })
    }

    /// 测试用包装：linked 值作为结果。
    pub fn test_wrapper(
        original: Arc<dyn AnnotationAttribute>,
        linked: Arc<dyn AnnotationAttribute>,
    ) -> Arc<Self> {
        Self::new(
            original,
            linked,
            |_, linked| linked.get_value(),
            |original, linked| {
                original.is_value_equivalent_to_default_value()
                    && linked.is_value_equivalent_to_default_value()
            },
        )
    }

    /// 获取最内层 original 链末端。
    pub fn get_non_wrapped_original(&self) -> Arc<dyn AnnotationAttribute> {
        let mut current = Arc::clone(&self.original);
        while current.is_wrapped() {
            if let Some(w) = current.as_wrapped() {
                current = w.get_original();
            } else {
                break;
            }
        }
        current
    }

    /// 收集全部非包装叶子属性。
    pub fn get_all_linked_non_wrapped_attributes(&self) -> Vec<Arc<dyn AnnotationAttribute>> {
        let mut leaves = Vec::new();
        collect_leaves(Arc::clone(&self.original), &mut leaves);
        collect_leaves(Arc::clone(&self.linked), &mut leaves);
        leaves
    }
}

fn collect_leaves(
    current: Arc<dyn AnnotationAttribute>,
    out: &mut Vec<Arc<dyn AnnotationAttribute>>,
) {
    if !current.is_wrapped() {
        out.push(current);
        return;
    }
    if let Some(w) = current.as_wrapped() {
        collect_leaves(w.get_original(), out);
        collect_leaves(w.get_linked(), out);
    }
}

impl AnnotationAttribute for AbstractWrappedAnnotationAttribute {
    fn impl_type_name(&self) -> &'static str {
        "AbstractWrappedAnnotationAttribute"
    }

    fn get_annotation(&self) -> Arc<AnnotationMirror> {
        self.original.get_annotation()
    }

    fn get_attribute(&self) -> AttributeRef {
        self.original.get_attribute()
    }

    fn get_value(&self) -> AnnotationValue {
        (self.value_fn)(&self.original, &self.linked)
    }

    fn is_value_equivalent_to_default_value(&self) -> bool {
        (self.default_fn)(&self.original, &self.linked)
    }

    fn get_attribute_type(&self) -> ValueKind {
        self.original.get_attribute_type()
    }

    fn get_meta_annotation(&self, type_name: AnnotationTypeName) -> Option<Arc<AnnotationMirror>> {
        self.original.get_meta_annotation(type_name)
    }

    fn is_wrapped(&self) -> bool {
        true
    }

    fn as_wrapped(&self) -> Option<&dyn WrappedAnnotationAttribute> {
        Some(self)
    }
}

impl WrappedAnnotationAttribute for AbstractWrappedAnnotationAttribute {
    fn get_original(&self) -> Arc<dyn AnnotationAttribute> {
        Arc::clone(&self.original)
    }

    fn get_linked(&self) -> Arc<dyn AnnotationAttribute> {
        Arc::clone(&self.linked)
    }
}
