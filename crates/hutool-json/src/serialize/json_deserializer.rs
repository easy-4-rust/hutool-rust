use serde_json::Value;

use crate::Result;

/// 对齐: `cn.hutool.json.JSONDeserializer`
/// 中文说明: 类型化的自定义反序列化器 trait。
///
/// Typed custom deserializer contract.
pub trait JSONDeserializer<T>: Send + Sync {
    /// 中文说明: 反序列化一个值。
    /// 对齐 Java 方法: `deserialize`
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
