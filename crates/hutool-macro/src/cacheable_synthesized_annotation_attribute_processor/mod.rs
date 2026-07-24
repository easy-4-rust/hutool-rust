//! 对齐: `cn.hutool.core.annotation.CacheableSynthesizedAnnotationAttributeProcessor`

use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use crate::hierarchical::default_hierarchical_cmp;
use crate::mirror::{AnnotationValue, ValueKind};
use crate::synthesized_annotation::SynthesizedAnnotation;
use crate::synthesized_annotation_attribute_processor::SynthesizedAnnotationAttributeProcessor;

mod cacheable_synthesized_annotation_attribute_processor;
mod test_value_synthesized_annotation;

pub use cacheable_synthesized_annotation_attribute_processor::CacheableSynthesizedAnnotationAttributeProcessor;
pub use test_value_synthesized_annotation::TestValueSynthesizedAnnotation;
