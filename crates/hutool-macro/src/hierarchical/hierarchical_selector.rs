//! 对齐: `cn.hutool.core.annotation.Hierarchical`

use std::cmp::Ordering;

use super::choose_side::ChooseSide;
use super::hierarchical::Hierarchical;

/// 层级选择器，对齐 `Hierarchical.Selector`。
pub trait HierarchicalSelector: Send + Sync {
    /// 从两个层级对象中选择一个。
    fn choose(&self, prev: &dyn Hierarchical, next: &dyn Hierarchical) -> ChooseSide;
}
