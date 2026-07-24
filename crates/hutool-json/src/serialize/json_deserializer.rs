use std::{
    any::{Any, TypeId},
    collections::HashMap,
    sync::{Arc, OnceLock, RwLock},
};

use serde_json::Value;

use crate::{JsonError, Result};

/// Typed custom deserializer contract.
pub trait JSONDeserializer<T>: Send + Sync {
    /// Deserializes one value.
    fn deserialize(&self, value: &Value) -> Result<T>;
}

impl<T, F> JSONDeserializer<T> for F
where
    F: Fn(&Value) -> Result<T> + Send + Sync,
{
    fn deserialize(&self, value: &Value) -> Result<T> {
        self(value)
    }
}
