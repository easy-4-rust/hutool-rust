//! 对齐: `cn.hutool.core.lang.tree.TreeNodeConfig`

#![allow(dead_code)] // 对齐 Java TreeNodeConfig，暂未接线，预留

/// 对齐 Java: `TreeNodeConfig`
#[derive(Debug, Clone)]
pub struct TreeNodeConfig {
    pub id_key: String,
    pub parent_id_key: String,
    pub weight_key: String,
    pub name_key: String,
    pub children_key: String,
    pub deep: Option<usize>,
}

impl Default for TreeNodeConfig {
    fn default() -> Self {
        Self {
            id_key: "id".into(),
            parent_id_key: "parentId".into(),
            weight_key: "weight".into(),
            name_key: "name".into(),
            children_key: "children".into(),
            deep: None,
        }
    }
}

impl TreeNodeConfig {
    /// 设置权重字段名
    pub fn set_weight_key(&mut self, key: impl Into<String>) {
        self.weight_key = key.into();
    }
    /// 设置 id 字段名
    pub fn set_id_key(&mut self, key: impl Into<String>) {
        self.id_key = key.into();
    }
    /// 设置最大深度
    pub fn set_deep(&mut self, deep: usize) {
        self.deep = Some(deep);
    }
}
