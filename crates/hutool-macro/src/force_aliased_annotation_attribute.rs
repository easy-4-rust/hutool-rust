//! 对齐: `cn.hutool.core.annotation.ForceAliasedAnnotationAttribute`

use std::sync::Arc;

use super::abstract_wrapped_annotation_attribute::AbstractWrappedAnnotationAttribute;
use super::annotation_attribute::AnnotationAttribute;
use super::mirror::AnnotationValue;
use super::wrapped_annotation_attribute::WrappedAnnotationAttribute;

/// 对齐 Java 类: `cn.hutool.core.annotation.ForceAliasedAnnotationAttribute`
pub struct ForceAliasedAnnotationAttribute {
    inner: Arc<AbstractWrappedAnnotationAttribute>,
}

impl ForceAliasedAnnotationAttribute {
    /// 构造强制别名属性。
    pub fn new(
        original: Arc<dyn AnnotationAttribute>,
        linked: Arc<dyn AnnotationAttribute>,
    ) -> Arc<Self> {
        Arc::new(Self {
            inner: AbstractWrappedAnnotationAttribute::new(
                original,
                linked,
                |_, linked| linked.get_value(),
                |_, linked| linked.is_value_equivalent_to_default_value(),
            ),
        })
    }
}

impl AnnotationAttribute for ForceAliasedAnnotationAttribute {
    fn impl_type_name(&self) -> &'static str {
        "ForceAliasedAnnotationAttribute"
    }
    fn get_annotation(&self) -> Arc<super::mirror::AnnotationMirror> {
        self.inner.get_annotation()
    }
    fn get_attribute(&self) -> super::mirror::AttributeRef {
        self.inner.get_attribute()
    }
    fn get_value(&self) -> AnnotationValue {
        self.inner.get_value()
    }
    fn is_value_equivalent_to_default_value(&self) -> bool {
        self.inner.is_value_equivalent_to_default_value()
    }
    fn get_attribute_type(&self) -> super::mirror::ValueKind {
        self.inner.get_linked().get_attribute_type()
    }
    fn get_meta_annotation(
        &self,
        type_name: super::mirror::AnnotationTypeName,
    ) -> Option<Arc<super::mirror::AnnotationMirror>> {
        self.inner.get_meta_annotation(type_name)
    }
    fn is_wrapped(&self) -> bool {
        true
    }

    fn as_wrapped(&self) -> Option<&dyn WrappedAnnotationAttribute> {
        Some(self)
    }
}

impl WrappedAnnotationAttribute for ForceAliasedAnnotationAttribute {
    fn get_original(&self) -> Arc<dyn AnnotationAttribute> {
        self.inner.get_original()
    }
    fn get_linked(&self) -> Arc<dyn AnnotationAttribute> {
        self.inner.get_linked()
    }
}
