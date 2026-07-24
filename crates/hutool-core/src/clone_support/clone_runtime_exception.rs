//! Rust-native equivalents of Hutool's `core.clone` package.

use std::{
    any::type_name,
    error::Error,
    fmt::{self, Debug, Display},
    ops::{Deref, DerefMut},
};

use crate::format_template;

/// Clone failure with an optional standard Rust error source chain.
#[derive(Debug)]
pub struct CloneRuntimeException {
    message: String,
    source: Option<Box<dyn Error + Send + Sync>>,
}

impl CloneRuntimeException {
    /// Creates an exception from a message.
    #[must_use]
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            source: None,
        }
    }

    /// Creates an exception whose message identifies and describes its source.
    pub fn from_error<E>(source: E) -> Self
    where
        E: Error + Send + Sync + 'static,
    {
        let kind = type_name::<E>()
            .rsplit("::")
            .next()
            .unwrap_or(type_name::<E>());
        Self {
            message: format!("{kind}: {source}"),
            source: Some(Box::new(source)),
        }
    }

    /// Creates an exception from a Hutool-style `{}` message template.
    #[must_use]
    pub fn formatted(template: &str, params: &[&dyn Display]) -> Self {
        Self::new(format_template(template, params))
    }

    /// Creates an exception from a message and error source.
    pub fn with_source<E>(message: impl Into<String>, source: E) -> Self
    where
        E: Error + Send + Sync + 'static,
    {
        Self {
            message: message.into(),
            source: Some(Box::new(source)),
        }
    }

    /// Creates a templated exception while preserving its error source.
    pub fn formatted_with_source<E>(source: E, template: &str, params: &[&dyn Display]) -> Self
    where
        E: Error + Send + Sync + 'static,
    {
        Self::with_source(format_template(template, params), source)
    }
}

impl Display for CloneRuntimeException {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl Error for CloneRuntimeException {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source
            .as_deref()
            .map(|source| source as &(dyn Error + 'static))
    }
}
