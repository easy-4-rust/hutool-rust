//! 对齐: `cn.hutool.aop.Aspect`
//! 来源: hutool-aop/src/main/java/cn/hutool/aop/Aspect.java
//! 中文说明: 切面接口定义，提供前置、后置和异常处理的切面回调。

use crate::Method;

/// 对齐: `cn.hutool.aop.Aspect`
/// 中文说明: 类型化的 Hutool 兼容切面接口。`before` 返回 `false` 跳过操作，
/// `after` 返回 `false` 抑制返回值，`after_exception` 返回 `true` 允许错误传播。
pub trait Aspect<T, A, R, E>: Send + Sync {
    /// Runs before the target and decides whether it may execute.
    fn before(&self, _target: &T, _method: &Method, _args: &A) -> bool {
        true
    }

    /// Runs after a successful or deliberately suppressed invocation.
    fn after(&self, _target: &T, _method: &Method, _args: &A, _return_value: Option<&R>) -> bool {
        true
    }

    /// Runs after a target error and decides whether it propagates.
    fn after_exception(&self, _target: &T, _method: &Method, _args: &A, _error: &E) -> bool {
        true
    }
}
