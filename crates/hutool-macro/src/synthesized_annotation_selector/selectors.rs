//! 对齐: `cn.hutool.core.annotation.SynthesizedAnnotationSelector`

use std::sync::Arc;

use crate::hierarchical::{
    ChooseSide, FarthestAndNewestPrioritySelector, FarthestAndOldestPrioritySelector,
    Hierarchical, HierarchicalSelector, NearestAndNewestPrioritySelector,
    NearestAndOldestPrioritySelector,
};
use crate::synthesized_annotation::SynthesizedAnnotation;

use crate::synthesized_annotation_selector::SynthesizedAnnotationSelector;

/// 预置选择器工厂。
pub struct Selectors;

impl Selectors {
    /// 更近且更旧优先。
    pub fn nearest_and_oldest_priority() -> Arc<dyn SynthesizedAnnotationSelector> {
        wrap(Arc::new(NearestAndOldestPrioritySelector))
    }

    /// 更近且更新优先。
    pub fn nearest_and_newest_priority() -> Arc<dyn SynthesizedAnnotationSelector> {
        wrap(Arc::new(NearestAndNewestPrioritySelector))
    }

    /// 更远且更旧优先。
    pub fn farthest_and_oldest_priority() -> Arc<dyn SynthesizedAnnotationSelector> {
        wrap(Arc::new(FarthestAndOldestPrioritySelector))
    }

    /// 更远且更新优先。
    pub fn farthest_and_newest_priority() -> Arc<dyn SynthesizedAnnotationSelector> {
        wrap(Arc::new(FarthestAndNewestPrioritySelector))
    }
}

use super::{SelectorAdapter, wrap};
