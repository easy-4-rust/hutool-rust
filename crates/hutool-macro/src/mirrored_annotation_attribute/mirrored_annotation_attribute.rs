//! 对齐: `cn.hutool.core.annotation.MirroredAnnotationAttribute`

use std::sync::Arc;

use crate::abstract_wrapped_annotation_attribute::AbstractWrappedAnnotationAttribute;
use crate::annotation_attribute::AnnotationAttribute;
use crate::mirror::AnnotationValue;
use crate::wrapped_annotation_attribute::WrappedAnnotationAttribute;

use super::mirror_value_conflict_error::MirrorValueConflictError;

/// 对齐 Java 类: `cn.hutool.core.annotation.MirroredAnnotationAttribute`
pub struct MirroredAnnotationAttribute {
    inner: Arc<AbstractWrappedAnnotationAttribute>,
}

impl MirroredAnnotationAttribute {
    /// 构造镜像属性。
    pub fn new(original: Arc<dyn AnnotationAttribute>, linked: Arc<dyn AnnotationAttribute>) -> Arc<Self> {
        Arc::new(Self {
            inner: AbstractWrappedAnnotationAttribute::new(
                original,
                linked,
                mirror_value,
                |original, linked| {
                    original.is_value_equivalent_to_default_value()
                        && linked.is_value_equivalent_to_default_value()
                },
            ),
        })
    }

    /// 读取镜像属性值，冲突时返回 Err。
    pub fn try_get_value(&self) -> Result<AnnotationValue, MirrorValueConflictError> {
        mirror_value_result(&self.inner.get_original(), &self.inner.get_linked())
    }
}

impl AnnotationAttribute for MirroredAnnotationAttribute {
    fn impl_type_name(&self) -> &'static str {
        "MirroredAnnotationAttribute"
    }
    fn get_annotation(&self) -> Arc<crate::mirror::AnnotationMirror> {
        self.inner.get_annotation()
    }
    fn get_attribute(&self) -> crate::mirror::AttributeRef {
        self.inner.get_attribute()
    }
    fn get_value(&self) -> AnnotationValue {
        self.inner.get_value()
    }
    fn is_value_equivalent_to_default_value(&self) -> bool {
        self.inner.is_value_equivalent_to_default_value()
    }
    fn get_attribute_type(&self) -> crate::mirror::ValueKind {
        self.inner.get_attribute_type()
    }
    fn get_meta_annotation(
        &self,
        type_name: crate::mirror::AnnotationTypeName,
    ) -> Option<Arc<crate::mirror::AnnotationMirror>> {
        self.inner.get_meta_annotation(type_name)
    }
    fn is_wrapped(&self) -> bool {
        true
    }

    fn as_wrapped(&self) -> Option<&dyn WrappedAnnotationAttribute> {
        Some(self)
    }
}

impl WrappedAnnotationAttribute for MirroredAnnotationAttribute {
    fn get_original(&self) -> Arc<dyn AnnotationAttribute> {
        self.inner.get_original()
    }
    fn get_linked(&self) -> Arc<dyn AnnotationAttribute> {
        self.inner.get_linked()
    }
}

use super::{mirror_value, mirror_value_result};
