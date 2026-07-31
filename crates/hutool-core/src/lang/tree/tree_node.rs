//! 对齐: `cn.hutool.core.lang.tree.TreeNode`

#![allow(dead_code)] // 对齐 Java TreeNode，暂未接线，预留

/// 对齐 Java: `TreeNode<T>`
#[derive(Debug, Clone)]
pub struct TreeNode<T: Clone> {
    pub id: T,
    pub parent_id: T,
    pub name: String,
    pub weight: i32,
}

impl<T: Clone> TreeNode<T> {
    /// 对齐构造
    pub fn new(id: T, parent_id: T, name: impl Into<String>, weight: i32) -> Self {
        Self {
            id,
            parent_id,
            name: name.into(),
            weight,
        }
    }
}
