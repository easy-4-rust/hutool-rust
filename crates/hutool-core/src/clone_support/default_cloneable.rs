//! Rust-native equivalents of Hutool's `core.clone` package.

use std::{
    any::type_name,
    error::Error,
    fmt::{self, Debug, Display},
    ops::{Deref, DerefMut},
};

use crate::format_template;

/// Provides Hutool's generic `clone0` behavior without reflection.
pub trait DefaultCloneable: Clone {
    /// Clones `self` with the concrete return type preserved.
    #[must_use]
    fn clone0(&self) -> Self {
        self.clone()
    }
}

impl<T: Clone> DefaultCloneable for T {}
