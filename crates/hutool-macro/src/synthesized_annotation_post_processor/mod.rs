//! 对齐: `cn.hutool.core.annotation.SynthesizedAnnotationPostProcessor`

use std::sync::Arc;

use crate::annotation_synthesizer::AnnotationSynthesizer;
use crate::synthesized_annotation::SynthesizedAnnotation;

mod synthesized_annotation_post_processor;
mod post_processors;

pub use synthesized_annotation_post_processor::SynthesizedAnnotationPostProcessor;
pub use post_processors::PostProcessors;
