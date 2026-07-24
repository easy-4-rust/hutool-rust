//! 对齐: `cn.hutool.core.lang.Chain`
//! 来源: hutool-core/src/main/java/cn/hutool/core/lang/Chain.java

/// 对齐 Java: `Chain<E, T>` — 可迭代责任链，`add_chain` 返回 `Self` 以链式调用。
pub trait Chain<E>: IntoIterator {
    /// 对齐 Java: `addChain(E)` — 追加环节并返回自身。
    fn add_chain(self, element: E) -> Self
    where
        Self: Sized;
}
