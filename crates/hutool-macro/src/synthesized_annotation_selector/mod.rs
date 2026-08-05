//! 对齐: `cn.hutool.core.annotation.SynthesizedAnnotationSelector`

use std::sync::Arc;

use crate::hierarchical::HierarchicalSelector;

mod selectors;
mod synthesized_annotation_selector;
mod test_synthesized_annotation;

pub use selectors::Selectors;
pub use synthesized_annotation_selector::SynthesizedAnnotationSelector;
pub use test_synthesized_annotation::TestSynthesizedAnnotation;

struct SelectorAdapter {
    inner: Arc<dyn HierarchicalSelector>,
}

fn wrap(selector: Arc<dyn HierarchicalSelector>) -> Arc<dyn SynthesizedAnnotationSelector> {
    Arc::new(SelectorAdapter { inner: selector })
}
