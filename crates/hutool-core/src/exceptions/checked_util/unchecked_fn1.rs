//! 对齐: `cn.hutool.core.exceptions.CheckedUtil`
//! 来源: hutool-core/src/main/java/cn/hutool/core/exceptions/CheckedUtil.java
//!
//! 将可能失败的表达式包装为运行时错误，避免显式 try/catch。



/// 对齐 Java: `CheckedUtil.Func1Rt`
pub struct UncheckedFn1<P, R> {
    pub(crate) inner: Box<dyn Fn(P) -> R + Send + Sync>,
}

