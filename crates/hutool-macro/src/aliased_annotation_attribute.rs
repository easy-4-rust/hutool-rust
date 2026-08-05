//! 对齐: `cn.hutool.core.annotation.AliasedAnnotationAttribute`

use std::sync::Arc;

use super::abstract_wrapped_annotation_attribute::AbstractWrappedAnnotationAttribute;
use super::annotation_attribute::AnnotationAttribute;
use super::mirror::AnnotationValue;
use super::wrapped_annotation_attribute::WrappedAnnotationAttribute;

/// 对齐 Java 类: `cn.hutool.core.annotation.AliasedAnnotationAttribute`
pub struct AliasedAnnotationAttribute {
    inner: Arc<AbstractWrappedAnnotationAttribute>,
}

impl AliasedAnnotationAttribute {
    /// 构造别名属性。
    pub fn new(
        original: Arc<dyn AnnotationAttribute>,
        linked: Arc<dyn AnnotationAttribute>,
    ) -> Arc<Self> {
        Arc::new(Self {
            inner: AbstractWrappedAnnotationAttribute::new(
                original,
                linked,
                |original, linked| {
                    if linked.is_value_equivalent_to_default_value() {
                        original.get_value()
                    } else {
                        linked.get_value()
                    }
                },
                |original, linked| {
                    original.is_value_equivalent_to_default_value()
                        && linked.is_value_equivalent_to_default_value()
                },
            ),
        })
    }
}

impl AnnotationAttribute for AliasedAnnotationAttribute {
    fn impl_type_name(&self) -> &'static str {
        "AliasedAnnotationAttribute"
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
        self.inner.get_attribute_type()
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

impl WrappedAnnotationAttribute for AliasedAnnotationAttribute {
    fn get_original(&self) -> Arc<dyn AnnotationAttribute> {
        self.inner.get_original()
    }
    fn get_linked(&self) -> Arc<dyn AnnotationAttribute> {
        self.inner.get_linked()
    }
}
