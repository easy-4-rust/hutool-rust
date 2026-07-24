use std::{
    any::{Any, TypeId},
    collections::HashMap,
    sync::{Arc, OnceLock, RwLock},
};

use serde_json::Value;

use crate::{JsonError, Result};

/// Typed custom serializer contract.
pub trait JSONSerializer<T>: Send + Sync {
    /// Serializes one value.
    fn serialize(&self, value: &T) -> Result<Value>;
}

impl<T, F> JSONSerializer<T> for F
where
    F: Fn(&T) -> Result<Value> + Send + Sync,
{
    fn serialize(&self, value: &T) -> Result<Value> {
        self(value)
    }
}
