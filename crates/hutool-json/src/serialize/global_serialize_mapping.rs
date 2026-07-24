use std::{
    any::{Any, TypeId},
    collections::HashMap,
    sync::{Arc, OnceLock, RwLock},
};

use serde_json::Value;

use crate::{JsonError, Result};

use super::serialize_registry::SerializeRegistry;

/// Explicit compatibility access to Hutool's global serialization mapping.
pub struct GlobalSerializeMapping;

impl GlobalSerializeMapping {
    /// Returns a snapshot sharing registered closures.
    ///
    /// # Panics
    ///
    /// Panics when another thread poisoned the compatibility-global lock.
    #[must_use]
    pub fn get() -> SerializeRegistry {
        global_slot()
            .read()
            .expect("global JSON mapping read lock poisoned")
            .clone()
    }

    /// Replaces the compatibility global and returns its previous mapping.
    ///
    /// # Panics
    ///
    /// Panics when another thread poisoned the compatibility-global lock.
    pub fn set(registry: SerializeRegistry) -> SerializeRegistry {
        std::mem::replace(
            &mut *global_slot()
                .write()
                .expect("global JSON mapping write lock poisoned"),
            registry,
        )
    }

    /// Restores an empty global mapping.
    pub fn reset() -> SerializeRegistry {
        Self::set(SerializeRegistry::new())
    }
}

fn global_slot() -> &'static RwLock<SerializeRegistry> {
    static REGISTRY: OnceLock<RwLock<SerializeRegistry>> = OnceLock::new();
    REGISTRY.get_or_init(|| RwLock::new(SerializeRegistry::new()))
}
