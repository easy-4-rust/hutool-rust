//! Stateful and view-based collection adapters aligned with Hutool.

/// 对齐: `cn.hutool.core.collection.NodeListIter`
/// 节点列表迭代器



/// XML node lists map to the same resettable borrowed-slice iterator in Rust.
pub type NodeListIter<'a, T> = ArrayIter<'a, T>;
