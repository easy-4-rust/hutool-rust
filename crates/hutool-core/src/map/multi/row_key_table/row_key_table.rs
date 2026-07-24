//! 对齐: `cn.hutool.core.map.multi.RowKeyTable` / `Table` / `AbsTable`
//! 来源: hutool-core/.../multi/RowKeyTable.java

use std::collections::HashMap;
use std::hash::Hash;

use super::table_cell::TableCell;

/// 对齐 Java 类: `cn.hutool.core.map.multi.RowKeyTable`
///
/// 以行为主键的二维表：`row -> (column -> value)`。
#[derive(Debug, Clone, Default)]
pub struct RowKeyTable<R, C, V> {
    rows: HashMap<R, HashMap<C, V>>,
}

impl<R: Eq + Hash + Clone, C: Eq + Hash + Clone, V: Clone> RowKeyTable<R, C, V> {
    /// 对齐 Java: `RowKeyTable()`
    pub fn new() -> Self {
        Self {
            rows: HashMap::new(),
        }
    }

    /// 对齐 Java: `RowKeyTable(Map)`
    pub fn from_raw(raw: HashMap<R, HashMap<C, V>>) -> Self {
        Self { rows: raw }
    }

    /// 对齐 Java: `rowMap`
    pub fn row_map(&self) -> &HashMap<R, HashMap<C, V>> {
        &self.rows
    }

    /// 对齐 Java: `put(R, C, V)`
    pub fn put(&mut self, row: R, column: C, value: V) -> Option<V> {
        self.rows.entry(row).or_default().insert(column, value)
    }

    /// 对齐 Java: `get(R, C)`
    pub fn get(&self, row: &R, column: &C) -> Option<&V> {
        self.rows.get(row).and_then(|cols| cols.get(column))
    }

    /// 对齐 Java: `remove(R, C)`
    pub fn remove(&mut self, row: &R, column: &C) -> Option<V> {
        let val = self.rows.get_mut(row)?.remove(column);
        if self.rows.get(row).map(|c| c.is_empty()).unwrap_or(false) {
            self.rows.remove(row);
        }
        val
    }

    /// 对齐 Java: `getRow`
    pub fn get_row(&self, row: &R) -> Option<&HashMap<C, V>> {
        self.rows.get(row)
    }

    /// 对齐 Java: `contains` / `containsRow`
    pub fn contains(&self, row: &R, column: &C) -> bool {
        self.get(row, column).is_some()
    }

    /// 对齐 Java: `containsRow`
    pub fn contains_row(&self, row: &R) -> bool {
        self.rows.contains_key(row)
    }

    /// 对齐 Java: `containsColumn`
    pub fn contains_column(&self, column: &C) -> bool {
        self.rows.values().any(|cols| cols.contains_key(column))
    }

    /// 对齐 Java: `containsValue`
    pub fn contains_value(&self, value: &V) -> bool
    where
        V: PartialEq,
    {
        self.rows
            .values()
            .any(|cols| cols.values().any(|v| v == value))
    }

    /// 对齐 Java: `rowKeySet`
    pub fn row_key_set(&self) -> impl Iterator<Item = &R> {
        self.rows.keys()
    }

    /// 对齐 Java: `columnKeySet`
    pub fn column_key_set(&self) -> Vec<C> {
        let mut cols = Vec::new();
        for row in self.rows.values() {
            for c in row.keys() {
                if !cols.contains(c) {
                    cols.push(c.clone());
                }
            }
        }
        cols
    }

    /// 对齐 Java: `columnKeys`
    pub fn column_keys(&self) -> Vec<C> {
        self.column_key_set()
    }

    /// 对齐 Java: `getColumn`
    pub fn get_column(&self, column: &C) -> HashMap<R, V> {
        let mut out = HashMap::new();
        for (r, cols) in &self.rows {
            if let Some(v) = cols.get(column) {
                out.insert(r.clone(), v.clone());
            }
        }
        out
    }

    /// 对齐 Java: `columnMap`
    pub fn column_map(&self) -> HashMap<C, HashMap<R, V>> {
        let mut out: HashMap<C, HashMap<R, V>> = HashMap::new();
        for (r, cols) in &self.rows {
            for (c, v) in cols {
                out.entry(c.clone()).or_default().insert(r.clone(), v.clone());
            }
        }
        out
    }

    /// 对齐 Java: `isEmpty`
    pub fn is_empty(&self) -> bool {
        self.rows.is_empty() || self.rows.values().all(|c| c.is_empty())
    }

    /// 对齐 Java: `clear`
    pub fn clear(&mut self) {
        self.rows.clear();
    }

    /// 单元格数量。
    pub fn size(&self) -> usize {
        self.rows.values().map(|c| c.len()).sum()
    }

    /// 对齐 Java: `cellSet` / 迭代
    pub fn cells(&self) -> Vec<TableCell<R, C, V>> {
        let mut out = Vec::new();
        for (r, cols) in &self.rows {
            for (c, v) in cols {
                out.push(TableCell {
                    row_key: r.clone(),
                    column_key: c.clone(),
                    value: v.clone(),
                });
            }
        }
        out
    }

    /// 对齐 Java: `values`
    pub fn values(&self) -> Vec<&V> {
        self.rows.values().flat_map(|c| c.values()).collect()
    }
}
