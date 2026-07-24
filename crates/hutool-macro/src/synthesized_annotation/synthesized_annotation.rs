//! 对齐: `cn.hutool.core.annotation.SynthesizedAnnotation`

use std::collections::HashMap;
use std::sync::Arc;

use crate::annotation_attribute::AnnotationAttribute;
use crate::hierarchical::Hierarchical;
use crate::mirror::{AnnotationMirror, AnnotationTypeName, AnnotationValue, ValueKind};

/// 对齐 Java interface: `cn.hutool.core.annotation.SynthesizedAnnotation`
pub trait SynthesizedAnnotation: Hierarchical + Send + Sync {
    /// 获取被合成的注解对象。
    fn get_annotation(&self) -> Arc<AnnotationMirror>;

    /// 是否存在指定属性。
    fn has_attribute(&self, attribute_name: &str, return_type: ValueKind) -> bool;

    /// 获取全部属性。
    fn get_attributes(&self) -> HashMap<String, Arc<dyn AnnotationAttribute>>;

    /// 设置属性。
    fn set_attribute(&self, attribute_name: &str, attribute: Arc<dyn AnnotationAttribute>);

    /// 替换属性。
    fn replace_attribute(
        &self,
        attribute_name: &str,
        operator: Box<dyn Fn(Arc<dyn AnnotationAttribute>) -> Arc<dyn AnnotationAttribute> + Send + Sync>,
    );

    /// 获取属性值。
    fn get_attribute_value(&self, attribute_name: &str) -> Option<AnnotationValue>;

    /// 注解类型。
    fn annotation_type(&self) -> AnnotationTypeName;
}
