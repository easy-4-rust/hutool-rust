//! Rust-native equivalents of Hutool's `core.clone` package.

use std::{
    any::type_name,
    error::Error,
    fmt::{self, Debug, Display},
    ops::{Deref, DerefMut},
};

use crate::format_template;

/// A transparent clone-support wrapper for values that implement [`Clone`].
#[repr(transparent)]
#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct CloneSupport<T>(T);

impl<T> CloneSupport<T> {
    /// Wraps a value with an explicit clone-support type.
    pub const fn new(value: T) -> Self {
        Self(value)
    }

    /// Unwraps the owned value.
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T> Deref for CloneSupport<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for CloneSupport<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
