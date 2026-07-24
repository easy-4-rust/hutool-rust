//! Provider capability model used by the thin Hutool-compatible facade.

#![allow(missing_docs, clippy::enum_glob_use, clippy::match_same_arms)]

use crate::Message;
use serde_json::{Map, Value, json};
use std::{path::PathBuf, sync::Arc};

/// A video generation command-line style parameter.
#[derive(Debug, Clone, PartialEq)]
pub struct VideoParameter {
    /// Provider option name, such as `--rt`.
    pub kind: String,
    /// Provider option value.
    pub value: Value,
}

impl VideoParameter {
    /// Creates a typed video option.
    #[must_use]
    pub fn new(kind: impl Into<String>, value: impl Into<Value>) -> Self {
        Self {
            kind: kind.into(),
            value: value.into(),
        }
    }
}
