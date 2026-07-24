use std::io::Write;

use serde::{Serialize, de::DeserializeOwned};
use serde_json::Value;

use crate::{
    JSONArray, JSONConfig, JSONObject, JsonContainer, JsonError, Result, get_by_path, put_by_path,
};

use super::json_util::JSONUtil;

/// Streaming JSON writer with explicit ownership and error propagation.
pub struct JSONWriter<W: Write> {
    writer: W,
    mode: Option<WriterMode>,
    first: bool,
    pending_key: bool,
    config: JSONConfig,
}

impl<W: Write> JSONWriter<W> {
    /// Creates a writer around an owned destination.
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

    /// Starts an object.
    pub fn begin_obj(&mut self) -> Result<&mut Self> {
        self.writer.write_all(b"{")?;
        self.mode = Some(WriterMode::Object);
        Ok(self)
    }

    /// Starts an array.
    pub fn begin_array(&mut self) -> Result<&mut Self> {
        self.writer.write_all(b"[")?;
        self.mode = Some(WriterMode::Array);
        Ok(self)
    }

    /// Writes an object key.
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

    /// Writes one array element or the value for the pending object key.
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

    /// Writes one object field, respecting null omission.
    pub fn write_field(&mut self, key: &str, value: &Value) -> Result<&mut Self> {
        if self.config.is_ignore_null_value() && value.is_null() {
            return Ok(self);
        }
        self.write_key(key)?.write_value(value)
    }

    /// Ends the active container and flushes the destination.
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

    /// Returns the owned destination.
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
