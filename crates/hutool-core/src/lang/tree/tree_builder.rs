//! 对齐: `cn.hutool.core.lang.tree.TreeBuilder`

#![allow(dead_code)] // 对齐 Java TreeBuilder，暂未接线，预留

use crate::lang::tree::tree::Tree;

/// 对齐 Java: `TreeBuilder`
pub struct TreeBuilder<T: Clone> {
    root: Tree<T>,
    built: bool,
}

impl<T: Clone + PartialEq> TreeBuilder<T> {
    /// 创建
    pub fn of(root_id: T) -> Self {
        Self {
            root: Tree::new(root_id.clone(), root_id),
            built: false,
        }
    }

    /// 追加子节点
    pub fn append(&mut self, child: Tree<T>) -> &mut Self {
        self.root.children.push(child);
        self
    }

    /// 构建
    pub fn build(mut self) -> Tree<T> {
        self.built = true;
        self.root
    }

    /// 是否已构建
    pub fn is_built(&self) -> bool {
        self.built
    }
}
