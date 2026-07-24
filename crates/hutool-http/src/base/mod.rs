//! Shared Hutool-style request and response metadata.

use crate::Header;
use encoding_rs::{Encoding, UTF_8};
use std::{
    collections::{HashMap, hash_map::Entry},
    fmt,
};
use thiserror::Error;

mod http_base_error;
mod http_base;

pub use http_base_error::HttpBaseError;
pub use http_base::HttpBase;

pub const HTTP_1_0: &str = "HTTP/1.0";

pub const HTTP_1_1: &str = "HTTP/1.1";
