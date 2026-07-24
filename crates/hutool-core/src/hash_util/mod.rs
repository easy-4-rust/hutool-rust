//! Classic non-cryptographic hashes aligned with Hutool's UTF-16 and wrapping rules.

#![allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]

use crate::lang::hash::{CityHash, Number128};
use crate::IdKey;
use thiserror::Error;

mod hash_error;
mod hash_util;

pub use hash_error::HashError;
pub use hash_util::HashUtil;
