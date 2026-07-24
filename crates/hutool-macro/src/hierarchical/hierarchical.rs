//! 对齐: `cn.hutool.core.annotation.Hierarchical`

use std::cmp::Ordering;

/// 对齐 Java interface: `cn.hutool.core.annotation.Hierarchical`
pub trait Hierarchical: Send + Sync {
    /// 参照物。
    fn get_root(&self) -> Option<&dyn std::any::Any>;

    /// 与参照物的垂直距离。
    fn get_vertical_distance(&self) -> i32;

    /// 与参照物的水平距离。
    fn get_horizontal_distance(&self) -> i32;

    /// 默认层级比较器。
    fn compare_hierarchical(&self, other: &dyn Hierarchical) -> Ordering {
        self.get_vertical_distance()
            .cmp(&other.get_vertical_distance())
            .then_with(|| {
                self.get_horizontal_distance()
                    .cmp(&other.get_horizontal_distance())
            })
    }
}
