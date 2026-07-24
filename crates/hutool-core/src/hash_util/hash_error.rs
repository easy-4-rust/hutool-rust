//! Classic non-cryptographic hashes aligned with Hutool's UTF-16 and wrapping rules.

#![allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]

use crate::lang::hash::{CityHash, Number128};
use crate::IdKey;
use thiserror::Error;

/// Validation errors for table-driven hash functions.
#[derive(Debug, Clone, Error, PartialEq, Eq)]
pub enum HashError {
    /// Java would throw an arithmetic exception for a zero modulus.
    #[error("hash prime must not be zero")]
    ZeroPrime,
    /// Universal hashing requires eight table entries per key unit.
    #[error("universal hash table requires {expected} entries, received {actual}")]
    UniversalTable {
        /// Minimum required entry count.
        expected: usize,
        /// Supplied entry count.
        actual: usize,
    },
    /// Zobrist hashing requires one row per key unit.
    #[error("zobrist table requires {expected} rows, received {actual}")]
    ZobristRows {
        /// Minimum required row count.
        expected: usize,
        /// Supplied row count.
        actual: usize,
    },
    /// A Zobrist row does not cover the UTF-16 value used as its index.
    #[error("zobrist row {row} requires {expected} entries, received {actual}")]
    ZobristColumns {
        /// Zero-based row index.
        row: usize,
        /// Minimum required column count.
        expected: usize,
        /// Supplied column count.
        actual: usize,
    },
}
