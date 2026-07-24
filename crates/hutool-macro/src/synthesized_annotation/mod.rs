//! 对齐: `cn.hutool.core.annotation.SynthesizedAnnotation`

use std::collections::HashMap;
use std::sync::Arc;

use crate::annotation_attribute::AnnotationAttribute;
use crate::hierarchical::Hierarchical;
use crate::mirror::{AnnotationMirror, AnnotationTypeName, AnnotationValue, ValueKind};

mod synthesized_annotation;
mod annotation_attribute_value_provider;

pub use synthesized_annotation::SynthesizedAnnotation;
pub use annotation_attribute_value_provider::AnnotationAttributeValueProvider;
