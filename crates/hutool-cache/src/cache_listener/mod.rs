//! Callback invoked after an entry leaves a cache.
//!
//! Hutool 对齐: `cn.hutool.cache.CacheListener`

use std::sync::Arc;

mod cache_listener;
mod shared_listener;

pub use cache_listener::CacheListener;
pub use shared_listener::SharedListener;
