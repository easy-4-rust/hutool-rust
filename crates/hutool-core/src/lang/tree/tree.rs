//! 对齐: `cn.hutool.core.lang.tree.Tree`

#![allow(dead_code)] // 对齐 Java Tree，暂未接线，预留

use std::collections::HashMap;

/// 对齐 Java: `Tree<T>`
#[derive(Debug, Clone)]
pub struct Tree<T: Clone> {
    pub id: T,
    pub parent_id: T,
    pub name: String,
    pub weight: i32,
    pub children: Vec<Tree<T>>,
    pub extra: HashMap<String, String>,
    pub parent: Option<Box<Tree<T>>>,
}

impl<T: Clone + PartialEq> Tree<T> {
    /// 新建节点
    pub fn new(id: T, parent_id: T) -> Self {
        Self {
            id,
            parent_id,
            name: String::new(),
            weight: 0,
            children: Vec::new(),
            extra: HashMap::new(),
            parent: None,
        }
    }

    pub fn set_id(&mut self, id: T) { self.id = id; }
    pub fn set_parent_id(&mut self, pid: T) { self.parent_id = pid; }
    pub fn set_name(&mut self, name: impl Into<String>) { self.name = name.into(); }
    pub fn set_weight(&mut self, w: i32) { self.weight = w; }
    pub fn get_id(&self) -> &T { &self.id }
    pub fn get_parent_id(&self) -> &T { &self.parent_id }
    pub fn get_name(&self) -> &str { &self.name }
    pub fn get_children(&self) -> &[Tree<T>] { &self.children }
    pub fn put_extra(&mut self, k: impl Into<String>, v: impl Into<String>) {
        self.extra.insert(k.into(), v.into());
    }

    /// 对齐 `walk`
    pub fn walk<F: FnMut(&Tree<T>)>(&self, f: &mut F) {
        f(self);
        for c in &self.children {
            c.walk(f);
        }
    }

    /// 对齐 `cloneTree`
    pub fn clone_tree(&self) -> Tree<T> {
        let mut t = self.clone();
        t.parent = None;
        t
    }

    /// 对齐 `filter` — 保留谓词为真的子树
    pub fn filter<F: Fn(&Tree<T>) -> bool>(&mut self, pred: &F) -> bool {
        self.children.retain_mut(|c| c.filter(pred));
        pred(self) || !self.children.is_empty()
    }

    /// 对齐 `filterNew`
    pub fn filter_new<F: Fn(&Tree<T>) -> bool>(&self, pred: &F) -> Option<Tree<T>> {
        let mut t = self.clone_tree();
        if t.filter(pred) {
            Some(t)
        } else {
            None
        }
    }

    /// 获取父节点名链
    pub fn get_parents_name(&self, include_current: bool) -> Vec<String> {
        let mut names = Vec::new();
        if include_current {
            names.push(self.name.clone());
        }
        // parent 链接在 build 时可选填充；此处仅返回当前
        names
    }
}
