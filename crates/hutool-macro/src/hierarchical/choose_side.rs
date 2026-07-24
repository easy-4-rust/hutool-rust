//! 对齐: `cn.hutool.core.annotation.Hierarchical`

use std::cmp::Ordering;

/// 选择结果侧。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChooseSide {
    Prev,
    Next,
}
