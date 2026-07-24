use std::{
    collections::HashMap,
    fmt,
    sync::{Arc, OnceLock, RwLock},
};

use super::log_factory::LogFactory;

/// Explicit compatibility access to Hutool's process-wide factory.
pub struct GlobalLogFactory;

impl GlobalLogFactory {
    /// Returns a handle sharing the current global factory's cache.
    ///
    /// # Panics
    ///
    /// Panics when another thread poisoned the global factory lock.
    #[must_use]
    pub fn get() -> LogFactory {
        global_slot()
            .read()
            .expect("global log factory read lock poisoned")
            .clone()
    }
    /// Replaces the compatibility global and returns its previous value.
    ///
    /// # Panics
    ///
    /// Panics when another thread poisoned the global factory lock.
    pub fn set(factory: LogFactory) -> LogFactory {
        std::mem::replace(
            &mut *global_slot()
                .write()
                .expect("global log factory write lock poisoned"),
            factory,
        )
    }
    /// Restores the production tracing factory.
    pub fn reset() -> LogFactory {
        Self::set(LogFactory::default())
    }
}

use super::{format_message, global_slot};
