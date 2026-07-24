//! Typed method metadata and explicit invocation handlers.

use std::{borrow::Cow, fmt};

/// Stable metadata for one proxied operation.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Method {
    name: Cow<'static, str>,
}

impl Method {
    /// Creates method metadata from an owned or static name.
    pub fn new(name: impl Into<Cow<'static, str>>) -> Self {
        Self { name: name.into() }
    }

    /// Returns the operation name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }
}
