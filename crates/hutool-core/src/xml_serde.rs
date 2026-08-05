//! Optional direct Serde integration for XML.

/// 对齐: `cn.hutool.core.xml.XmlUtil`
/// XML 序列化/反序列化
use std::io::{BufRead, Write};

use serde::{Serialize, de::DeserializeOwned};

use crate::{CoreError, Result};

/// Direct `quick-xml` Serde facade without a `serde_json::Value` intermediate.
#[derive(Debug, Clone, Copy, Default)]
pub struct XmlSerde;

impl XmlSerde {
    /// Deserializes an XML string.
    pub fn from_str<T>(xml: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        quick_xml::de::from_str(xml).map_err(|error| CoreError::Xml(error.to_string()))
    }

    /// Deserializes XML incrementally from a buffered reader.
    pub fn from_reader<T, R>(reader: R) -> Result<T>
    where
        T: DeserializeOwned,
        R: BufRead,
    {
        quick_xml::de::from_reader(reader).map_err(|error| CoreError::Xml(error.to_string()))
    }

    /// Serializes a value to an XML string.
    pub fn to_string<T>(value: &T) -> Result<String>
    where
        T: ?Sized + Serialize,
    {
        quick_xml::se::to_string(value).map_err(|error| CoreError::Xml(error.to_string()))
    }

    /// Serializes UTF-8 XML directly to an I/O writer.
    pub fn to_writer<T, W>(writer: W, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
        W: Write,
    {
        quick_xml::se::to_utf8_io_writer(writer, value)
            .map(|_| ())
            .map_err(|error| CoreError::Xml(error.to_string()))
    }
}
