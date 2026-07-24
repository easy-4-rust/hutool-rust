use std::{
    any::{Any, TypeId},
    collections::HashMap,
    sync::{Arc, OnceLock, RwLock},
};

use serde_json::Value;

use crate::{JsonError, Result};

mod json_serializer;
mod json_deserializer;
mod serialize_registry;
mod global_serialize_mapping;

pub use json_serializer::JSONSerializer;
pub use json_deserializer::JSONDeserializer;
pub use serialize_registry::SerializeRegistry;
pub use global_serialize_mapping::GlobalSerializeMapping;
