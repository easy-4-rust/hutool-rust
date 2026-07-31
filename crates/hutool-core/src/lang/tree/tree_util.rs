//! 对齐: `cn.hutool.core.lang.tree.TreeUtil`

#![allow(dead_code)] // 对齐 Java TreeUtil，暂未接线，预留

use crate::lang::tree::tree::Tree;
use crate::lang::tree::tree_node::TreeNode;
use crate::lang::tree::tree_node_config::TreeNodeConfig;

/// 对齐 Java: `TreeUtil`
pub struct TreeUtil;

impl TreeUtil {
    /// 对齐 `build(list, rootId)`
    pub fn build<T: Clone + PartialEq + Eq + std::hash::Hash>(
        nodes: &[TreeNode<T>],
        root_id: T,
    ) -> Vec<Tree<T>> {
        Self::build_with_config(nodes, root_id, &TreeNodeConfig::default(), |n, t| {
            t.set_id(n.id.clone());
            t.set_parent_id(n.parent_id.clone());
            t.set_name(n.name.clone());
            t.set_weight(n.weight);
        })
    }

    /// 对齐 `buildSingle`
    pub fn build_single<T: Clone + PartialEq + Eq + std::hash::Hash>(
        nodes: &[TreeNode<T>],
        root_id: T,
    ) -> Tree<T> {
        let mut root = Tree::new(root_id.clone(), root_id.clone());
        root.set_name("root");
        let children = Self::build(nodes, root_id);
        root.children = children;
        // 回填 parent 引用（简化：不设 Box parent，测试用 getChildren/getParent 近似）
        for c in &mut root.children {
            link_parent(c);
        }
        root
    }

    /// 带配置与转换器
    pub fn build_with_config<T, F>(
        nodes: &[TreeNode<T>],
        root_id: T,
        _config: &TreeNodeConfig,
        mut converter: F,
    ) -> Vec<Tree<T>>
    where
        T: Clone + PartialEq + Eq + std::hash::Hash,
        F: FnMut(&TreeNode<T>, &mut Tree<T>),
    {
        let mut by_parent: std::collections::HashMap<T, Vec<TreeNode<T>>> =
            std::collections::HashMap::new();
        for n in nodes {
            by_parent
                .entry(n.parent_id.clone())
                .or_default()
                .push(n.clone());
        }
        fn build_children<T, F>(
            parent_id: &T,
            by_parent: &std::collections::HashMap<T, Vec<TreeNode<T>>>,
            converter: &mut F,
            depth: usize,
            max_depth: Option<usize>,
        ) -> Vec<Tree<T>>
        where
            T: Clone + PartialEq + Eq + std::hash::Hash,
            F: FnMut(&TreeNode<T>, &mut Tree<T>),
        {
            if max_depth.is_some_and(|m| depth > m) {
                return Vec::new();
            }
            let Some(list) = by_parent.get(parent_id) else {
                return Vec::new();
            };
            let mut out = Vec::new();
            for n in list {
                let mut t = Tree::new(n.id.clone(), n.parent_id.clone());
                converter(n, &mut t);
                t.children = build_children(&n.id, by_parent, converter, depth + 1, max_depth);
                out.push(t);
            }
            out
        }
        build_children(&root_id, &by_parent, &mut converter, 1, _config.deep)
    }
}

fn link_parent<T: Clone + PartialEq>(_t: &mut Tree<T>) {
    // parent 指针在 Rust 中易形成环；测试通过 children 关系验证
}
