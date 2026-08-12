//! General collection operations aligned with Hutool's `CollUtil` capability model.

/// 对齐: `cn.hutool.core.collection.CollUtil`
/// 集合类型

/// Concrete collection kinds replacing Java's reflective `Class<?>` factory.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CollectionKind {
    /// Contiguous growable list.
    List,
    /// Double-ended linked-style list.
    Deque,
    /// Unordered unique collection.
    Set,
    /// Insertion-ordered unique collection.
    OrderedSet,
    /// Key-ordered unique collection.
    SortedSet,
}
