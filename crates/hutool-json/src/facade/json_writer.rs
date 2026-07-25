use std::io::Write;

use serde::{Serialize, de::DeserializeOwned};
use serde_json::Value;

use crate::{
    JSONArray, JSONConfig, JSONObject, JsonContainer, JsonError, Result, get_by_path, put_by_path,
};

use super::json_util::JSONUtil;

/// 对齐: `cn.hutool.json.JSONWriter`
/// 中文说明: 流式 JSON 写入器，支持显式所有权和错误传播。
///
/// Streaming JSON writer with explicit ownership and error propagation.
pub struct JSONWriter<W: Write> {
    writer: W,
    mode: Option<WriterMode>,
    first: bool,
    pending_key: bool,
    config: JSONConfig,
}

impl<W: Write> JSONWriter<W> {
    /// 中文说明: 创建基于目标写入器的 JSON 写入器。
    /// 对齐 Java 方法: `new JSONWriter(Writer)`
    #[must_use]
    pub const fn new(writer: W, config: JSONConfig) -> Self {
        Self {
            writer,
            mode: None,
            first: true,
            pending_key: false,
            config,
        }
    }

    /// 中文说明: 开始写入对象。
    /// 对齐 Java 方法: `startObject`
    pub fn begin_obj(&mut self) -> Result<&mut Self> {
        self.writer.write_all(b"{")?;
        self.mode = Some(WriterMode::Object);
        Ok(self)
    }

    /// 中文说明: 开始写入数组。
    /// 对齐 Java 方法: `startArray`
    pub fn begin_array(&mut self) -> Result<&mut Self> {
        self.writer.write_all(b"[")?;
        self.mode = Some(WriterMode::Array);
        Ok(self)
    }

    /// 中文说明: 写入对象的键。
    /// 对齐 Java 方法: `writeKey`
    pub fn write_key(&mut self, key: &str) -> Result<&mut Self> {
        if self.mode != Some(WriterMode::Object) || self.pending_key {
            return Err(JsonError::Syntax(
                "key outside object or without value".into(),
            ));
        }
        self.separator()?;
        write!(self.writer, "{}:", JSONUtil::quote(key))?;
        self.pending_key = true;
        Ok(self)
    }

    /// 中文说明: 写入一个数组元素或待处理对象键的值。
    /// 对齐 Java 方法: `writeValue`
    pub fn write_value(&mut self, value: &Value) -> Result<&mut Self> {
        match self.mode {
            Some(WriterMode::Object) if !self.pending_key => {
                return Err(JsonError::Syntax("object value requires a key".into()));
            }
            Some(WriterMode::Array) => self.separator()?,
            Some(WriterMode::Object) => self.pending_key = false,
            None => return Err(JsonError::Syntax("value outside container".into())),
        }
        let value = normalize_writer_value(value, &self.config);
        serde_json::to_writer(&mut self.writer, &value)?;
        Ok(self)
    }

    /// 中文说明: 写入一个对象字段，遵循 null 省略配置。
    /// 对齐 Java 方法: `writeField`
    pub fn write_field(&mut self, key: &str, value: &Value) -> Result<&mut Self> {
        if self.config.is_ignore_null_value() && value.is_null() {
            return Ok(self);
        }
        self.write_key(key)?.write_value(value)
    }

    /// 中文说明: 结束当前容器并刷新目标写入器。
    /// 对齐 Java 方法: `end`
    pub fn end(&mut self) -> Result<&mut Self> {
        if self.pending_key {
            return Err(JsonError::Syntax("object key has no value".into()));
        }
        let closing = match self.mode.take() {
            Some(WriterMode::Object) => b'}',
            Some(WriterMode::Array) => b']',
            None => return Err(JsonError::Syntax("no active container".into())),
        };
        self.writer.write_all(&[closing])?;
        self.writer.flush()?;
        Ok(self)
    }

    /// 中文说明: 返回拥有的目标写入器。
    /// 对齐 Java 方法: `getWriter`
    #[must_use]
    pub fn into_inner(self) -> W {
        self.writer
    }

    fn separator(&mut self) -> Result<()> {
        if self.first {
            self.first = false;
        } else {
            self.writer.write_all(b",")?;
        }
        Ok(())
    }
}

use super::{WriterMode, normalize_writer_value};
