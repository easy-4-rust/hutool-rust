//! 对齐: `cn.hutool.core.exceptions.CheckedUtil`
//! 来源: hutool-core/src/main/java/cn/hutool/core/exceptions/CheckedUtil.java
//!
//! 将可能失败的表达式包装为运行时错误，避免显式 try/catch。

use std::panic::{catch_unwind, AssertUnwindSafe};

use super::checked_util::CheckedUtil;
use super::unchecked_fn0::UncheckedFn0;
use super::unchecked_fn1::UncheckedFn1;
use super::wrapped_runtime::WrappedRuntime;

/// 对齐 Java: `CheckedUtil.VoidFunc0Rt`
pub struct UncheckedVoidFn0 {
    pub(crate) inner: Box<dyn Fn() + Send + Sync>,
}

impl CheckedUtil {
    /// 对齐 Java: `uncheck(Func0<R>)`
    pub fn uncheck0<R, F>(expression: F) -> UncheckedFn0<R>
    where
        F: Fn() -> R + Send + Sync + 'static,
        R: Send + 'static,
    {
        UncheckedFn0 {
            inner: Box::new(move || {
                catch_unwind(AssertUnwindSafe(&expression))
                    .unwrap_or_else(|_| panic!("unchecked expression panicked"))
            }),
        }
    }

    /// 对齐 Java: `uncheck(Func1<P,R>)`
    pub fn uncheck1<P, R, F>(expression: F) -> UncheckedFn1<P, R>
    where
        F: Fn(P) -> R + Send + Sync + 'static,
        P: Send + 'static,
        R: Send + 'static,
    {
        UncheckedFn1 {
            inner: Box::new(move |param| {
                catch_unwind(AssertUnwindSafe(|| expression(param)))
                    .unwrap_or_else(|_| panic!("unchecked expression panicked"))
            }),
        }
    }

    /// 对齐 Java: `uncheck(VoidFunc0)`
    pub fn uncheck_void<F>(expression: F) -> UncheckedVoidFn0
    where
        F: Fn() + Send + Sync + 'static,
    {
        UncheckedVoidFn0 {
            inner: Box::new(move || {
                let _ = catch_unwind(AssertUnwindSafe(&expression))
                    .unwrap_or_else(|_| panic!("unchecked expression panicked"));
            }),
        }
    }

    /// 对齐 Java: `uncheck(Func0<R>)` —— `Result` 版。
    pub fn uncheck_result0<R, E, F>(expression: F) -> UncheckedFn0<R>
    where
        F: Fn() -> Result<R, E> + Send + Sync + 'static,
        E: std::fmt::Display + Send + 'static,
        R: Send + 'static,
    {
        UncheckedFn0 {
            inner: Box::new(move || expression().unwrap_or_else(|e| panic!("{}", e))),
        }
    }

    /// 对齐 Java: `uncheck(Func1<P,R>)` —— `Result` 版。
    pub fn uncheck_result1<P, R, E, F>(expression: F) -> UncheckedFn1<P, R>
    where
        F: Fn(P) -> Result<R, E> + Send + Sync + 'static,
        E: std::fmt::Display + Send + 'static,
        P: Send + 'static,
        R: Send + 'static,
    {
        UncheckedFn1 {
            inner: Box::new(move |param| expression(param).unwrap_or_else(|e| panic!("{}", e))),
        }
    }

    /// 对齐 Java: `uncheck` 带自定义 RuntimeException 映射。
    pub fn uncheck0_map<R, E, F, M>(expression: F, map: M) -> UncheckedFn0<R>
    where
        F: Fn() -> Result<R, E> + Send + Sync + 'static,
        M: Fn(E) -> WrappedRuntime + Send + Sync + 'static,
        R: Send + 'static,
        E: Send + 'static,
    {
        UncheckedFn0 {
            inner: Box::new(move || {
                expression().unwrap_or_else(|e| panic!("{}", map(e)))
            }),
        }
    }
}

impl<R> UncheckedFn0<R> {
    /// 对齐 Java: `call()`
    pub fn call(&self) -> R {
        (self.inner)()
    }
}

impl<P, R> UncheckedFn1<P, R> {
    /// 对齐 Java: `call(P)`
    pub fn call(&self, param: P) -> R {
        (self.inner)(param)
    }
}

impl UncheckedVoidFn0 {
    /// 对齐 Java: `call()`
    pub fn call(&self) {
        (self.inner)();
    }

    /// 对齐 Java: `callWithRuntimeException()` 别名。
    pub fn call_with_runtime_exception(&self) {
        self.call();
    }
}

