//! 对齐: `java.lang.reflect.InvocationHandler`
//! 来源: hutool-aop/src/main/java/cn/hutool/aop/
//! 中文说明: 调用处理器接口，定义代理对象方法调用的统一分发逻辑。

use super::method::Method;

/// 对齐: `java.lang.reflect.InvocationHandler`
/// 中文说明: 类型化的调用处理器接口，等价于 Java 的 `InvocationHandler`，用于代理方法分发。
pub trait InvocationHandler<T, A, R, E>: Send + Sync {
    /// Invokes `method` against `target` with mutable arguments.
    fn invoke(&self, target: &mut T, method: &Method, args: &mut A) -> Result<R, E>;
}

impl<T, A, R, E, F> InvocationHandler<T, A, R, E> for F
where
    F: Fn(&mut T, &Method, &mut A) -> Result<R, E> + Send + Sync,
{
    fn invoke(&self, target: &mut T, method: &Method, args: &mut A) -> Result<R, E> {
        self(target, method, args)
    }
}
