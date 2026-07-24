//! Shared Hutool-style request and response metadata.

use crate::Header;
use encoding_rs::{Encoding, UTF_8};
use std::{
    collections::{HashMap, hash_map::Entry},
    fmt,
};
use thiserror::Error;

/// Errors returned while configuring shared HTTP metadata.
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum HttpBaseError {
    /// The requested character encoding is not supported by Encoding Standard.
    #[error("unsupported HTTP character encoding: {0}")]
    UnsupportedCharset(String),
}

use super::{HTTP_1_0, HTTP_1_1};
