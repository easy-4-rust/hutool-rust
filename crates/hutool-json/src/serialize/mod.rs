//! 对齐: `cn.hutool.json` 包中的序列化组件
//! 来源: hutool-json/src/main/java/cn/hutool/json/JSONSerializer.java, JSONDeserializer.java
//! 中文说明: 提供自定义序列化/反序列化映射注册表和全局映射。

mod json_serializer;
mod json_deserializer;
mod serialize_registry;
mod global_serialize_mapping;

pub use json_serializer::JSONSerializer;
pub use json_deserializer::JSONDeserializer;
pub use serialize_registry::SerializeRegistry;
pub use global_serialize_mapping::GlobalSerializeMapping;
