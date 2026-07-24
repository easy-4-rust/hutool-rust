//! 对齐: `cn.hutool.core.annotation.Hierarchical`

use std::cmp::Ordering;

use super::choose_side::ChooseSide;
use super::hierarchical::Hierarchical;
use super::hierarchical_selector::HierarchicalSelector;

/// 更近且更旧优先。
#[derive(Debug, Clone, Copy, Default)]
pub struct NearestAndOldestPrioritySelector;

impl HierarchicalSelector for NearestAndOldestPrioritySelector {
    fn choose(&self, prev: &dyn Hierarchical, next: &dyn Hierarchical) -> ChooseSide {
        if next.get_vertical_distance() < prev.get_vertical_distance() {
            ChooseSide::Next
        } else {
            ChooseSide::Prev
        }
    }
}
