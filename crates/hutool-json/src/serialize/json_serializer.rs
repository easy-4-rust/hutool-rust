use std::{
    any::{Any, TypeId},
    collections::HashMap,
    sync::{Arc, OnceLock, RwLock},
};

use serde_json::Value;

use crate::{JsonError, Result};

/// 对齐: `cn.hutool.json.JSONSerializer`
/// 中文说明: 类型化的自定义序列化器 trait。
///
/// Typed custom serializer contract.
pub trait JSONSerializer<T>: Send + Sync {
    /// 中文说明: 序列化一个值。
    /// 对齐 Java 方法: `serialize`
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
