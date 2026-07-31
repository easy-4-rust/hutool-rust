use std::{
    any::{Any, TypeId},
    collections::HashMap,
    sync::Arc,
};

use serde_json::Value;

use crate::{JsonError, Result};

use super::json_deserializer::JSONDeserializer;
use super::json_serializer::JSONSerializer;

/// 对齐: `cn.hutool.json.JSONConverter`
/// 中文说明: 显式拥有的自定义序列化映射注册表。
///
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
    /// 中文说明: 创建空的序列化映射注册表。
    /// 对齐 Java 方法: `new SerializeRegistry`
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// 中文说明: 注册或替换类型化的序列化器。
    /// 对齐 Java 方法: `putSerializer`
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

    /// 中文说明: 注册或替换类型化的反序列化器。
    /// 对齐 Java 方法: `putDeserializer`
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

    /// 中文说明: 使用已注册的映射序列化值。
    /// 对齐 Java 方法: `serialize`
    pub fn serialize<T: Any + Send + Sync>(&self, value: &T) -> Result<Value> {
        self.serializers
            .get(&TypeId::of::<T>())
            .ok_or(JsonError::Mapping("serializer not registered"))?(value)
    }

    /// 中文说明: 使用已注册的映射反序列化值。
    /// 对齐 Java 方法: `deserialize`
    pub fn deserialize<T: Any + Send + Sync>(&self, value: &Value) -> Result<T> {
        Ok(*self
            .deserializers
            .get(&TypeId::of::<T>())
            .ok_or(JsonError::Mapping("deserializer not registered"))?(value)?
        .downcast::<T>()
        .map_err(|_| JsonError::Mapping("deserializer type mismatch"))?)
    }

    /// 中文说明: 判断是否没有注册任何映射。
    /// 对齐 Java 方法: `isEmpty`
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.serializers.is_empty() && self.deserializers.is_empty()
    }

    /// 中文说明: 清除所有已注册的映射。
    /// 对齐 Java 方法: `clear`
    pub fn clear(&mut self) {
        self.serializers.clear();
        self.deserializers.clear();
    }
}

type ErasedSerializer = Arc<dyn Fn(&dyn Any) -> Result<Value> + Send + Sync>;

type ErasedDeserializer = Arc<dyn Fn(&Value) -> Result<Box<dyn Any>> + Send + Sync>;
