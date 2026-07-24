//! Rust-native equivalents of Hutool's `core.clone` package.

use std::{
    any::type_name,
    error::Error,
    fmt::{self, Debug, Display},
    ops::{Deref, DerefMut},
};

use crate::format_template;

mod clone_support;
mod default_cloneable;
mod clone_runtime_exception;

pub use clone_support::CloneSupport;
pub use default_cloneable::DefaultCloneable;
pub use clone_runtime_exception::CloneRuntimeException;
