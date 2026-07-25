//! 对齐: `cn.hutool.aop.interceptor`
//! 来源: hutool-aop/src/main/java/cn/hutool/aop/interceptor/
//! 中文说明: 显式的 JDK/CGLIB 兼容切面拦截器，负责在代理调用时执行切面前后置逻辑。

use crate::{Method, aspects::Aspect};
use std::{fmt, marker::PhantomData, sync::Arc};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum InvocationMode {
    Jdk,
    Cglib,
}

struct AspectInterceptor<T, A, R, E> {
    target: T,
    aspect: Arc<dyn Aspect<T, A, R, E>>,
    mode: InvocationMode,
    marker: PhantomData<fn(A) -> Result<R, E>>,
}

impl<T: fmt::Debug, A, R, E> fmt::Debug for AspectInterceptor<T, A, R, E> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("AspectInterceptor")
            .field("target", &self.target)
            .field("mode", &self.mode)
            .finish_non_exhaustive()
    }
}

impl<T, A, R, E> AspectInterceptor<T, A, R, E> {
    fn new<I>(target: T, aspect: I, mode: InvocationMode) -> Self
    where
        I: Aspect<T, A, R, E> + 'static,
    {
        Self::with_shared(target, Arc::new(aspect), mode)
    }

    fn with_shared(target: T, aspect: Arc<dyn Aspect<T, A, R, E>>, mode: InvocationMode) -> Self {
        Self {
            target,
            aspect,
            mode,
            marker: PhantomData,
        }
    }

    fn get_target(&self) -> &T {
        &self.target
    }

    fn get_target_mut(&mut self) -> &mut T {
        &mut self.target
    }

    fn into_target(self) -> T {
        self.target
    }

    fn invoke<F>(&mut self, method: &Method, args: &mut A, operation: F) -> Result<Option<R>, E>
    where
        F: FnOnce(&mut T, &mut A) -> Result<R, E>,
    {
        let aspect = Arc::clone(&self.aspect);
        if !aspect.before(&self.target, method, args) {
            if self.mode == InvocationMode::Cglib {
                let _ = aspect.after(&self.target, method, args, None);
            }
            return Ok(None);
        }

        match operation(&mut self.target, args) {
            Ok(value) => {
                if aspect.after(&self.target, method, args, Some(&value)) {
                    Ok(Some(value))
                } else {
                    Ok(None)
                }
            }
            Err(error) => {
                if aspect.after_exception(&self.target, method, args, &error) {
                    Err(error)
                } else {
                    let _ = aspect.after(&self.target, method, args, None);
                    Ok(None)
                }
            }
        }
    }
}

macro_rules! define_interceptor {
    ($(#[$meta:meta])* $name:ident, $mode:expr) => {
        $(#[$meta])*
        pub struct $name<T, A, R, E>(AspectInterceptor<T, A, R, E>);

        impl<T: fmt::Debug, A, R, E> fmt::Debug for $name<T, A, R, E> {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.debug_tuple(stringify!($name)).field(&self.0).finish()
            }
        }

        impl<T, A, R, E> $name<T, A, R, E> {
            /// Creates an interceptor from an owned aspect.
            pub fn new<I>(target: T, aspect: I) -> Self
            where
                I: Aspect<T, A, R, E> + 'static,
            {
                Self(AspectInterceptor::new(target, aspect, $mode))
            }

            /// Creates an interceptor from a shared aspect.
            pub fn with_shared(target: T, aspect: Arc<dyn Aspect<T, A, R, E>>) -> Self {
                Self(AspectInterceptor::with_shared(target, aspect, $mode))
            }

            /// Returns the target.
            #[must_use]
            pub fn get_target(&self) -> &T {
                self.0.get_target()
            }

            /// Returns the mutable target.
            #[must_use]
            pub fn get_target_mut(&mut self) -> &mut T {
                self.0.get_target_mut()
            }

            /// Consumes the interceptor and returns its target.
            #[must_use]
            pub fn into_target(self) -> T {
                self.0.into_target()
            }

            /// Invokes one operation through the aspect.
            pub fn invoke<F>(
                &mut self,
                method: &Method,
                args: &mut A,
                operation: F,
            ) -> Result<Option<R>, E>
            where
                F: FnOnce(&mut T, &mut A) -> Result<R, E>,
            {
                self.0.invoke(method, args, operation)
            }
        }
    };
}

define_interceptor!(
    /// 对齐: `cn.hutool.aop.interceptor.JdkInterceptor`
    /// 中文说明: JDK 风格拦截器，当 `before` 拒绝时跳过 `after` 回调。
    JdkInterceptor,
    InvocationMode::Jdk
);
define_interceptor!(
    /// 对齐: `cn.hutool.aop.interceptor.CglibInterceptor`
    /// 中文说明: CGLIB 风格拦截器，即使 `before` 拒绝也会执行 `after` 回调。
    CglibInterceptor,
    InvocationMode::Cglib
);
define_interceptor!(
    /// 对齐: `cn.hutool.aop.interceptor.SpringCglibInterceptor`
    /// 中文说明: Spring CGLIB 兼容拦截器，使用 CGLIB 回调顺序。
    SpringCglibInterceptor,
    InvocationMode::Cglib
);
