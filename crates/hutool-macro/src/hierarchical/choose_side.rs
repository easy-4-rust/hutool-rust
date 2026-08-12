//! 对齐: `cn.hutool.core.annotation.Hierarchical`

/// 选择结果侧。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChooseSide {
    /// 选择前一个。
    Prev,
    /// 选择后一个。
    Next,
}
