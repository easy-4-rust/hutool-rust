//! 对齐: `cn.hutool.core.map.MapUtil`
//! 来源: hutool-core/src/main/java/cn/hutool/core/map/MapUtil.java
//!
//! Rust 版本提供 Map 操作的 idiomatic 实现。保留既有 `&HashMap` API，
//! 并补充 Option / IndexMap / BTreeMap / 可变就地编辑等 Hutool 同名能力。

/// `MapUtil.createMap(Class)` 的 Rust 表达 —— 对齐常见 Map 实现类型。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CreateMapKind {
    /// `HashMap` / 默认
    Hash,
    /// `LinkedHashMap`（IndexMap）
    Linked,
    /// `TreeMap`（BTreeMap）
    Tree,
    /// `IdentityHashMap` —— Rust 无引用相等 Map，退化为 HashMap
    Identity,
    /// `ConcurrentHashMap` —— 无锁创建入口
    Concurrent,
}
