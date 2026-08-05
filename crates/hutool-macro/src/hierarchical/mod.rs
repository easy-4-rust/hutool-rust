//! 对齐: `cn.hutool.core.annotation.Hierarchical`

use std::cmp::Ordering;

mod choose_side;
mod farthest_and_newest_priority_selector;
mod farthest_and_oldest_priority_selector;
mod hierarchical;
mod hierarchical_selector;
mod nearest_and_newest_priority_selector;
mod nearest_and_oldest_priority_selector;

pub use choose_side::ChooseSide;
pub use farthest_and_newest_priority_selector::FarthestAndNewestPrioritySelector;
pub use farthest_and_oldest_priority_selector::FarthestAndOldestPrioritySelector;
pub use hierarchical::Hierarchical;
pub use hierarchical_selector::HierarchicalSelector;
pub use nearest_and_newest_priority_selector::NearestAndNewestPrioritySelector;
pub use nearest_and_oldest_priority_selector::NearestAndOldestPrioritySelector;

/// 默认层级比较函数。
pub fn default_hierarchical_cmp(a: &dyn Hierarchical, b: &dyn Hierarchical) -> Ordering {
    a.compare_hierarchical(b)
}
