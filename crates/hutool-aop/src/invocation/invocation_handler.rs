//! Typed method metadata and explicit invocation handlers.

use std::{borrow::Cow, fmt};

use super::method::Method;

/// A typed equivalent of Java's `InvocationHandler`.
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
