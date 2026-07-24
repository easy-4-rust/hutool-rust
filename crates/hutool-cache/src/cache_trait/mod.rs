//! Common [`Cache`] trait used across all Hutool-aligned cache engines.
//!
//! Hutool 对齐: `cn.hutool.cache.Cache`

use std::hash::Hash;

use crate::impl::cache_obj::CacheObj;

mod cache;
mod cache_into_iterator;

pub use cache::Cache;
pub use cache_into_iterator::CacheIntoIterator;
