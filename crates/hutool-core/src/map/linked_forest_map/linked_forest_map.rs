//! 对齐: `cn.hutool.core.map.TreeEntry` / `ForestMap` / `LinkedForestMap`
//! 来源: hutool-core/.../LinkedForestMap.java（简化可运行实现）

use std::collections::HashMap;
use std::hash::Hash;

use super::tree_entry::TreeEntry;

/// 对齐 Java 类: `cn.hutool.core.map.LinkedForestMap`
#[derive(Debug, Clone, Default)]
pub struct LinkedForestMap<K, V> {
    nodes: HashMap<K, TreeEntry<K, V>>,
}

impl<K: Eq + Hash + Clone, V: Clone> LinkedForestMap<K, V> {
    /// 新建空森林。
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
        }
    }

    /// 对齐 Java: `putNode`
    pub fn put_node(&mut self, key: K, value: V) -> Option<V> {
        if let Some(entry) = self.nodes.get_mut(&key) {
            Some(entry.set_value(value))
        } else {
            self.nodes.insert(key.clone(), TreeEntry::new(key, value));
            None
        }
    }

    /// 对齐 Java: `put` —— 等价 `putNode(key, node.getValue())`
    pub fn put(&mut self, key: K, entry: TreeEntry<K, V>) -> Option<TreeEntry<K, V>> {
        let old_v = self.put_node(key.clone(), entry.value.clone());
        old_v.map(|v| TreeEntry::new(key, v))
    }

    /// 对齐 Java: `get`
    pub fn get(&self, key: &K) -> Option<&TreeEntry<K, V>> {
        self.nodes.get(key)
    }

    /// 对齐 Java: `getNodeValue`
    pub fn get_node_value(&self, key: &K) -> Option<&V> {
        self.nodes.get(key).map(|e| e.value())
    }

    /// 对齐 Java: `linkNodes` —— parent -> child
    pub fn link_nodes(&mut self, parent_key: K, child_key: K) {
        if !self.nodes.contains_key(&parent_key) || !self.nodes.contains_key(&child_key) {
            return;
        }
        // unlink old parent of child
        if let Some(old_parent) = self.nodes.get(&child_key).and_then(|e| e.parent.clone()) {
            if let Some(p) = self.nodes.get_mut(&old_parent) {
                p.children.retain(|c| c != &child_key);
            }
        }
        if let Some(child) = self.nodes.get_mut(&child_key) {
            child.parent = Some(parent_key.clone());
        }
        if let Some(parent) = self.nodes.get_mut(&parent_key) {
            if !parent.children.contains(&child_key) {
                parent.children.push(child_key);
            }
        }
    }

    /// 对齐 Java: `unlinkNode`
    pub fn unlink_node(&mut self, parent_key: &K, child_key: &K) {
        if let Some(parent) = self.nodes.get_mut(parent_key) {
            parent.children.retain(|c| c != child_key);
        }
        if let Some(child) = self.nodes.get_mut(child_key) {
            if child.parent.as_ref() == Some(parent_key) {
                child.parent = None;
            }
        }
    }

    /// 对齐 Java: `putLinkedNodes`
    pub fn put_linked_nodes(
        &mut self,
        parent_key: K,
        parent_value: V,
        child_key: K,
        child_value: V,
    ) {
        self.put_node(parent_key.clone(), parent_value);
        self.put_node(child_key.clone(), child_value);
        self.link_nodes(parent_key, child_key);
    }

    /// 对齐 Java: `remove` —— 删除节点并桥接父子。
    pub fn remove(&mut self, key: &K) -> Option<TreeEntry<K, V>> {
        let entry = self.nodes.remove(key)?;
        if let Some(ref parent_key) = entry.parent {
            if let Some(parent) = self.nodes.get_mut(parent_key) {
                parent.children.retain(|c| c != key);
                for child in &entry.children {
                    if !parent.children.contains(child) {
                        parent.children.push(child.clone());
                    }
                }
            }
            for child in &entry.children {
                if let Some(c) = self.nodes.get_mut(child) {
                    c.parent = Some(parent_key.clone());
                }
            }
        } else {
            for child in &entry.children {
                if let Some(c) = self.nodes.get_mut(child) {
                    c.parent = None;
                }
            }
        }
        Some(entry)
    }

    /// 对齐 Java: `clear`
    pub fn clear(&mut self) {
        self.nodes.clear();
    }

    /// 对齐 Java: `getRootNode`
    pub fn get_root_node(&self, key: &K) -> Option<&TreeEntry<K, V>> {
        let mut current = key.clone();
        loop {
            let entry = self.nodes.get(&current)?;
            match &entry.parent {
                Some(p) => current = p.clone(),
                None => return Some(entry),
            }
        }
    }

    /// 对齐 Java: `getDeclaredParentNode`
    pub fn get_declared_parent_node(&self, key: &K) -> Option<&TreeEntry<K, V>> {
        let parent = self.nodes.get(key)?.parent.as_ref()?;
        self.nodes.get(parent)
    }

    /// 对齐 Java: `getParentNode` —— 同 declared（简化）。
    pub fn get_parent_node(&self, key: &K) -> Option<&TreeEntry<K, V>> {
        self.get_declared_parent_node(key)
    }

    /// 对齐 Java: `containsParentNode`
    pub fn contains_parent_node(&self, key: &K, parent_key: &K) -> bool {
        let mut current = key.clone();
        while let Some(entry) = self.nodes.get(&current) {
            match &entry.parent {
                Some(p) if p == parent_key => return true,
                Some(p) => current = p.clone(),
                None => return false,
            }
        }
        false
    }

    /// 对齐 Java: `containsChildNode`
    pub fn contains_child_node(&self, key: &K, child_key: &K) -> bool {
        self.contains_parent_node(child_key, key)
    }

    /// 对齐 Java: `getDeclaredChildNodes`
    pub fn get_declared_child_nodes(&self, key: &K) -> Vec<&TreeEntry<K, V>> {
        self.nodes
            .get(key)
            .map(|e| {
                e.children
                    .iter()
                    .filter_map(|c| self.nodes.get(c))
                    .collect()
            })
            .unwrap_or_default()
    }

    /// 对齐 Java: `getChildNodes` —— 递归全部子孙。
    pub fn get_child_nodes(&self, key: &K) -> Vec<&TreeEntry<K, V>> {
        let mut out = Vec::new();
        self.collect_children(key, &mut out);
        out
    }

    /// 对齐 Java: `getTreeNodes` —— 根到当前路径。
    pub fn get_tree_nodes(&self, key: &K) -> Vec<&TreeEntry<K, V>> {
        let mut path = Vec::new();
        let mut current = Some(key.clone());
        while let Some(k) = current {
            if let Some(entry) = self.nodes.get(&k) {
                path.push(entry);
                current = entry.parent.clone();
            } else {
                break;
            }
        }
        path.reverse();
        path
    }

    /// 对齐 Java: `putAll`
    pub fn put_all(&mut self, entries: HashMap<K, TreeEntry<K, V>>) {
        for (k, entry) in entries {
            if let Some(parent) = entry.parent.clone() {
                let parent_val = entry.value.clone();
                self.put_linked_nodes(parent, parent_val.clone(), k, entry.value);
            } else {
                self.put_node(k, entry.value);
            }
        }
    }

    /// 节点数。
    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    /// 是否为空。
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    fn collect_children<'a>(&'a self, key: &K, out: &mut Vec<&'a TreeEntry<K, V>>) {
        if let Some(entry) = self.nodes.get(key) {
            for child in &entry.children {
                if let Some(c) = self.nodes.get(child) {
                    out.push(c);
                    self.collect_children(child, out);
                }
            }
        }
    }
}
