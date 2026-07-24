//! Provider capability model used by the thin Hutool-compatible facade.

#![allow(missing_docs, clippy::enum_glob_use, clippy::match_same_arms)]

use crate::Message;
use serde_json::{Map, Value, json};
use std::{path::PathBuf, sync::Arc};

/// Normalized raw provider response.
#[derive(Debug, Clone, PartialEq)]
pub enum AIResponse {
    /// JSON payload.
    Json(Value),
    /// Binary media payload.
    Bytes(Vec<u8>),
}

impl AIResponse {
    /// Serializes JSON or returns a lossy textual representation of bytes.
    #[must_use]
    pub fn into_text(self) -> String {
        match self {
            Self::Json(value) => value.to_string(),
            Self::Bytes(bytes) => String::from_utf8_lossy(&bytes).into_owned(),
        }
    }

    /// Extracts binary media, serializing JSON when necessary.
    #[must_use]
    pub fn into_bytes(self) -> Vec<u8> {
        match self {
            Self::Json(value) => value.to_string().into_bytes(),
            Self::Bytes(bytes) => bytes,
        }
    }
}
