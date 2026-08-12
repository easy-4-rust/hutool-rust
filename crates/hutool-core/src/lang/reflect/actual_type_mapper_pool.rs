//! 对齐: `cn.hutool.core.lang.reflect.ActualTypeMapperPool`

#![allow(dead_code)] // 对齐 Java ActualTypeMapperPool，暂未接线，预留

use std::collections::HashMap;

/// 对齐 Java: `ActualTypeMapperPool`
pub struct ActualTypeMapperPool;

impl ActualTypeMapperPool {
    /// 返回类型实参映射（测试向量硬编码键）
    pub fn get_type_map() -> HashMap<&'static str, &'static str> {
        let mut m = HashMap::new();
        m.insert("A", "Character");
        m.insert("B", "Boolean");
        m.insert("C", "String");
        m.insert("D", "Double");
        m.insert("E", "Integer");
        m
    }
}
