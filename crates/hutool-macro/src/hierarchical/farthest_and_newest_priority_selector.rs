//! 对齐: `cn.hutool.core.annotation.Hierarchical`

use std::cmp::Ordering;

use super::choose_side::ChooseSide;
use super::hierarchical::Hierarchical;
use super::hierarchical_selector::HierarchicalSelector;

/// 更远且更新优先。
#[derive(Debug, Clone, Copy, Default)]
pub struct FarthestAndNewestPrioritySelector;

impl HierarchicalSelector for FarthestAndNewestPrioritySelector {
    fn choose(&self, prev: &dyn Hierarchical, next: &dyn Hierarchical) -> ChooseSide {
        if next.get_vertical_distance() >= prev.get_vertical_distance() {
            ChooseSide::Next
        } else {
            ChooseSide::Prev
        }
    }
}
