//! Typed method metadata and explicit invocation handlers.

use std::{borrow::Cow, fmt};

use super::invocation_handler::InvocationHandler;
use super::method::Method;

/// Explicit proxy backed by a typed invocation handler.
pub struct HandlerProxy<T, H> {
    target: T,
    handler: H,
}

impl<T: fmt::Debug, H> fmt::Debug for HandlerProxy<T, H> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("HandlerProxy")
            .field("target", &self.target)
            .finish_non_exhaustive()
    }
}

impl<T, H> HandlerProxy<T, H> {
    /// Creates a handler-backed proxy.
    pub fn new(target: T, handler: H) -> Self {
        Self { target, handler }
    }

    /// Returns the target.
    #[must_use]
    pub fn get_target(&self) -> &T {
        &self.target
    }

    /// Returns the mutable target.
    #[must_use]
    pub fn get_target_mut(&mut self) -> &mut T {
        &mut self.target
    }

    /// Consumes the proxy and returns its target.
    #[must_use]
    pub fn into_target(self) -> T {
        self.target
    }

    /// Invokes the configured handler.
    pub fn invoke<A, R, E>(&mut self, method: &Method, args: &mut A) -> Result<R, E>
    where
        H: InvocationHandler<T, A, R, E>,
    {
        self.handler.invoke(&mut self.target, method, args)
    }
}
