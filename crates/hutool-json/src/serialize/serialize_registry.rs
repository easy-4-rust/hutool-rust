use std::{
    any::{Any, TypeId},
    collections::HashMap,
    sync::{Arc, OnceLock, RwLock},
};

use serde_json::Value;

use crate::{JsonError, Result};

use super::json_deserializer::JSONDeserializer;
use super::json_serializer::JSONSerializer;

/// Explicitly owned custom serialization mapping.
#[derive(Clone, Default)]
pub struct SerializeRegistry {
    serializers: HashMap<TypeId, ErasedSerializer>,
    deserializers: HashMap<TypeId, ErasedDeserializer>,
}

impl std::fmt::Debug for SerializeRegistry {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("SerializeRegistry")
            .field("serializers", &self.serializers.len())
            .field("deserializers", &self.deserializers.len())
            .finish()
    }
}

impl SerializeRegistry {
    /// Creates an empty mapping.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers or replaces a typed serializer.
    pub fn put_serializer<T: Any + Send + Sync>(
        &mut self,
        serializer: impl JSONSerializer<T> + 'static,
    ) -> &mut Self {
        self.serializers.insert(
            TypeId::of::<T>(),
            Arc::new(move |value| {
                serializer.serialize(
                    value
                        .downcast_ref::<T>()
                        .ok_or(JsonError::Mapping("serializer type mismatch"))?,
                )
            }),
        );
        self
    }

    /// Registers or replaces a typed deserializer.
    pub fn put_deserializer<T: Any + Send + Sync>(
        &mut self,
        deserializer: impl JSONDeserializer<T> + 'static,
    ) -> &mut Self {
        self.deserializers.insert(
            TypeId::of::<T>(),
            Arc::new(move |value| Ok(Box::new(deserializer.deserialize(value)?) as Box<dyn Any>)),
        );
        self
    }

    /// Serializes a value using its registered mapping.
    pub fn serialize<T: Any + Send + Sync>(&self, value: &T) -> Result<Value> {
        self.serializers
            .get(&TypeId::of::<T>())
            .ok_or(JsonError::Mapping("serializer not registered"))?(value)
    }

    /// Deserializes a value using its registered mapping.
    pub fn deserialize<T: Any + Send + Sync>(&self, value: &Value) -> Result<T> {
        Ok(*self
            .deserializers
            .get(&TypeId::of::<T>())
            .ok_or(JsonError::Mapping("deserializer not registered"))?(value)?
        .downcast::<T>()
        .map_err(|_| JsonError::Mapping("deserializer type mismatch"))?)
    }

    /// Returns whether no mappings are registered.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.serializers.is_empty() && self.deserializers.is_empty()
    }

    /// Removes all mappings.
    pub fn clear(&mut self) {
        self.serializers.clear();
        self.deserializers.clear();
    }
}

type ErasedSerializer = Arc<dyn Fn(&dyn Any) -> Result<Value> + Send + Sync>;

type ErasedDeserializer = Arc<dyn Fn(&Value) -> Result<Box<dyn Any>> + Send + Sync>;
