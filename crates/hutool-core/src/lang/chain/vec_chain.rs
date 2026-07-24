//! 对齐: `cn.hutool.core.lang.Chain`
//! 来源: hutool-core/src/main/java/cn/hutool/core/lang/Chain.java

use super::chain::Chain;

/// 基于 `Vec` 的简易责任链实现。
#[derive(Debug, Clone, Default)]
pub struct VecChain<E> {
    items: Vec<E>,
}

impl<E> VecChain<E> {
    /// 创建空链。
    #[must_use]
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    /// 当前环节列表。
    #[must_use]
    pub fn as_slice(&self) -> &[E] {
        &self.items
    }
}

impl<E> Chain<E> for VecChain<E> {
    fn add_chain(mut self, element: E) -> Self {
        self.items.push(element);
        self
    }
}

impl<E> IntoIterator for VecChain<E> {
    type Item = E;
    type IntoIter = std::vec::IntoIter<E>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}
