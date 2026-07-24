//! 对齐: `cn.hutool.core.annotation.SynthesizedAnnotationSelector`

use std::sync::Arc;

use crate::hierarchical::{
    ChooseSide, FarthestAndNewestPrioritySelector, FarthestAndOldestPrioritySelector,
    Hierarchical, HierarchicalSelector, NearestAndNewestPrioritySelector,
    NearestAndOldestPrioritySelector,
};
use crate::synthesized_annotation::SynthesizedAnnotation;

/// 对齐 Java interface: `cn.hutool.core.annotation.SynthesizedAnnotationSelector`
pub trait SynthesizedAnnotationSelector: Send + Sync {
    /// 比较两个合成注解并返回选中项。
    fn choose(
        &self,
        old_annotation: Arc<dyn SynthesizedAnnotation>,
        new_annotation: Arc<dyn SynthesizedAnnotation>,
    ) -> Arc<dyn SynthesizedAnnotation>;
}

impl SynthesizedAnnotationSelector for SelectorAdapter {
    fn choose(
        &self,
        old_annotation: Arc<dyn SynthesizedAnnotation>,
        new_annotation: Arc<dyn SynthesizedAnnotation>,
    ) -> Arc<dyn SynthesizedAnnotation> {
        match self.inner.choose(old_annotation.as_ref(), new_annotation.as_ref()) {
            ChooseSide::Next => new_annotation,
            ChooseSide::Prev => old_annotation,
        }
    }
}

use super::{SelectorAdapter, wrap};
