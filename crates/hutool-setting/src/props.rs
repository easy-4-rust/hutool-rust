//! 对齐: `cn.hutool.core.util.Props`
//! 来源: hutool-core/src/main/java/cn/hutool/core/util/Props.java
//! 中文说明: 有序的 Java 属性文档，支持 Hutool 风格的类型化访问器（int/bool/date/BigDecimal 等）。

use crate::{SettingError, checked_path, read_text};
use chrono::{DateTime, FixedOffset};
use encoding_rs::{Encoding, WINDOWS_1252};
use indexmap::IndexMap;
use num_bigint::BigInt;
use rust_decimal::Decimal;
use serde::{Serialize, de::DeserializeOwned};
use std::{
    fmt::Write as _,
    path::{Path, PathBuf},
    str::FromStr,
};

/// 有序的 Java 属性文档，对应 Hutool 的 `cn.hutool.core.util.Props`。
///
/// 对齐 Java 类: `cn.hutool.core.util.Props`
/// 来源: hutool-core/src/main/java/cn/hutool/core/util/Props.java
///
/// 支持 Hutool 风格的类型化访问器（int/bool/date/BigDecimal 等）。
/// Ordered Java-properties document with Hutool-style typed accessors.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Props {
    values: IndexMap<String, String>,
    path: Option<PathBuf>,
    charset: &'static Encoding,
}

impl Default for Props {
    fn default() -> Self {
        Self::new()
    }
}

impl Props {
    /// Creates an empty document.
    #[must_use]
    pub fn new() -> Self {
        Self {
            values: IndexMap::new(),
            path: None,
            charset: WINDOWS_1252,
        }
    }
    /// Alias matching Hutool.
    #[must_use]
    pub fn create() -> Self {
        Self::new()
    }
    /// Creates a UTF-8-independent map-backed document.
    #[must_use]
    pub fn from_map(values: IndexMap<String, String>) -> Self {
        Self {
            values,
            ..Self::new()
        }
    }
    /// Loads a path using Java properties' ISO-8859-1-compatible default.
    pub fn from_path(path: impl AsRef<Path>) -> Result<Self, SettingError> {
        Self::from_path_with_encoding(path, WINDOWS_1252)
    }
    /// Loads a path using an explicit encoding.
    pub fn from_path_with_encoding(
        path: impl AsRef<Path>,
        charset: &'static Encoding,
    ) -> Result<Self, SettingError> {
        Self::from_path_ref(path.as_ref(), charset)
    }
    fn from_path_ref(path: &Path, charset: &'static Encoding) -> Result<Self, SettingError> {
        let path = checked_path(path)?;
        let mut result = Self {
            path: Some(path),
            charset,
            ..Self::new()
        };
        result.load()?;
        Ok(result)
    }
    /// Replaces values by parsing Java properties text.
    pub fn load_text(&mut self, text: &str) -> Result<(), SettingError> {
        let mut values = IndexMap::new();
        let mut logical = String::new();
        for raw in text.lines() {
            let trimmed = raw.trim_start();
            if trimmed.starts_with('#') || trimmed.starts_with('!') {
                continue;
            }
            logical.push_str(raw.trim_end_matches('\\'));
            if raw.ends_with('\\') && !raw.ends_with("\\\\") {
                continue;
            }
            let line = logical.trim();
            if !line.is_empty() {
                let split = find_property_split(line);
                let key = unescape(line[..split].trim())?;
                let value =
                    unescape(line[split..].trim_start_matches(|c: char| {
                        c.is_whitespace() || matches!(c, '=' | ':')
                    }))?;
                values.insert(key, value);
            }
            logical.clear();
        }
        if !logical.is_empty() {
            return Err(SettingError::Invalid(
                "dangling property continuation".into(),
            ));
        }
        self.values = values;
        Ok(())
    }
    /// Reloads the configured path.
    pub fn load(&mut self) -> Result<(), SettingError> {
        let path = self
            .path
            .as_deref()
            .ok_or_else(|| SettingError::Invalid("properties have no path".into()))?;
        let text = read_text(path, self.charset)?;
        self.load_text(&text)
    }
    /// Returns an object-like string.
    #[must_use]
    pub fn get_obj(&self, key: &str) -> Option<&str> {
        self.values.get(key).map(String::as_str)
    }
    /// Gets a string or default.
    #[must_use]
    pub fn get_str_or<'a>(&'a self, key: &str, default: &'a str) -> &'a str {
        self.get_obj(key).unwrap_or(default)
    }
    /// Gets a string.
    #[must_use]
    pub fn get_str(&self, key: &str) -> Option<&str> {
        self.get_obj(key)
    }
    /// Parses a typed scalar.
    pub fn get_parse<T: FromStr>(&self, key: &str) -> Option<T> {
        self.get_obj(key)?.parse().ok()
    }
    /// Parses a scalar or returns the default.
    pub fn get_parse_or<T: FromStr>(&self, key: &str, default: T) -> T {
        self.get_parse(key).unwrap_or(default)
    }
    /// Gets an integer.
    pub fn get_int(&self, key: &str) -> Option<i32> {
        self.get_parse(key)
    }
    /// Gets a boolean accepting Java/Hutool common forms.
    pub fn get_bool(&self, key: &str) -> Option<bool> {
        match self.get_obj(key)?.trim().to_ascii_lowercase().as_str() {
            "true" | "yes" | "y" | "1" => Some(true),
            "false" | "no" | "n" | "0" => Some(false),
            _ => None,
        }
    }
    /// Gets a long.
    pub fn get_long(&self, key: &str) -> Option<i64> {
        self.get_parse(key)
    }
    /// Gets the first character.
    #[must_use]
    pub fn get_char(&self, key: &str) -> Option<char> {
        self.get_obj(key)?.chars().next()
    }
    /// Gets a float.
    pub fn get_float(&self, key: &str) -> Option<f32> {
        self.get_parse(key)
    }
    /// Gets a double.
    pub fn get_double(&self, key: &str) -> Option<f64> {
        self.get_parse(key)
    }
    /// Gets a short.
    pub fn get_short(&self, key: &str) -> Option<i16> {
        self.get_parse(key)
    }
    /// Gets a byte.
    pub fn get_byte(&self, key: &str) -> Option<i8> {
        self.get_parse(key)
    }
    /// Gets an arbitrary-precision decimal.
    pub fn get_big_decimal(&self, key: &str) -> Option<Decimal> {
        self.get_parse(key)
    }
    /// Gets an arbitrary-precision integer.
    pub fn get_big_integer(&self, key: &str) -> Option<BigInt> {
        self.get_parse(key)
    }
    /// Parses an enum or any `FromStr` value.
    pub fn get_enum<T: FromStr>(&self, key: &str) -> Option<T> {
        self.get_parse(key)
    }
    /// Parses an RFC3339 timestamp.
    pub fn get_date(&self, key: &str) -> Option<DateTime<FixedOffset>> {
        DateTime::parse_from_rfc3339(self.get_obj(key)?).ok()
    }
    /// Removes and returns the first matching string.
    pub fn get_and_remove_str(&mut self, keys: &[&str]) -> Option<String> {
        for key in keys {
            if let Some(value) = self.values.shift_remove(*key) {
                return Some(value);
            }
        }
        None
    }
    /// Returns a snapshot.
    #[must_use]
    pub fn to_properties(&self) -> IndexMap<String, String> {
        self.values.clone()
    }
    /// Deserializes the whole map or a key prefix.
    pub fn to_bean<T: DeserializeOwned>(&self, prefix: Option<&str>) -> Result<T, SettingError> {
        self.to_bean_with(prefix, crate::config_from_string_map)
    }
    fn to_bean_with<T: DeserializeOwned>(
        &self,
        prefix: Option<&str>,
        converter: fn(
            &std::collections::HashMap<String, String>,
        ) -> Result<config::Config, config::ConfigError>,
    ) -> Result<T, SettingError> {
        let values: std::collections::HashMap<String, String> =
            self.prefixed(prefix).into_iter().collect();
        Ok(converter(&values)?.try_deserialize()?)
    }
    fn prefixed(&self, prefix: Option<&str>) -> IndexMap<String, String> {
        let prefix = prefix.unwrap_or("");
        self.values
            .iter()
            .filter_map(|(k, v)| {
                k.strip_prefix(prefix)
                    .map(|key| (key.trim_start_matches('.').to_owned(), v.clone()))
            })
            .collect()
    }
    /// Overwrites values from a serializable object using JSON field names.
    pub fn fill_from<T: Serialize>(
        &mut self,
        value: &T,
        prefix: Option<&str>,
    ) -> Result<&mut Self, SettingError> {
        let value =
            serde_json::to_value(value).map_err(|e| SettingError::Invalid(e.to_string()))?;
        self.fill_value(value, prefix)
    }

    fn fill_value(
        &mut self,
        value: serde_json::Value,
        prefix: Option<&str>,
    ) -> Result<&mut Self, SettingError> {
        let serde_json::Value::Object(map) = value else {
            return Err(SettingError::Invalid(
                "bean must serialize as an object".into(),
            ));
        };
        let prefix = prefix.unwrap_or("");
        for (key, value) in map {
            let value = match value {
                serde_json::Value::String(v) => v,
                other => other.to_string(),
            };
            let key = if prefix.is_empty() {
                key
            } else {
                format!("{prefix}.{key}")
            };
            self.values.insert(key, value);
        }
        Ok(self)
    }
    /// Sets a property.
    #[allow(clippy::needless_pass_by_value)]
    pub fn set_property(&mut self, key: impl Into<String>, value: impl ToString) -> &mut Self {
        self.values.insert(key.into(), value.to_string());
        self
    }
    /// Stores in deterministic Java-properties syntax.
    pub fn store(&self, path: impl AsRef<Path>) -> Result<(), SettingError> {
        self.store_path(path.as_ref())
    }
    fn store_path(&self, path: &Path) -> Result<(), SettingError> {
        let mut output = String::new();
        for (key, value) in &self.values {
            output.push_str(&escape(key));
            output.push('=');
            output.push_str(&escape(value));
            output.push('\n');
        }
        std::fs::write(path, output)?;
        Ok(())
    }
    /// Number of properties.
    #[must_use]
    pub fn len(&self) -> usize {
        self.values.len()
    }
    /// Whether there are no properties.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
}

pub(crate) fn unescape(input: &str) -> Result<String, SettingError> {
    let mut output = String::new();
    let mut chars = input.chars();
    while let Some(ch) = chars.next() {
        if ch != '\\' {
            output.push(ch);
            continue;
        }
        let escaped = chars
            .next()
            .ok_or(SettingError::Invalid("dangling property escape".into()))?;
        match escaped {
            't' => output.push('\t'),
            'n' => output.push('\n'),
            'r' => output.push('\r'),
            'f' => output.push('\u{000c}'),
            'u' => {
                let digits: String = chars.by_ref().take(4).collect();
                if digits.len() != 4 {
                    return Err(SettingError::Invalid("short unicode escape".into()));
                }
                let value = u16::from_str_radix(&digits, 16).map_err(invalid_unicode_escape)?;
                if (0xD800..=0xDBFF).contains(&value) {
                    if chars.next() != Some('\\') || chars.next() != Some('u') {
                        return Err(SettingError::Invalid("missing low surrogate".into()));
                    }
                    let low_digits: String = chars.by_ref().take(4).collect();
                    let low =
                        u16::from_str_radix(&low_digits, 16).map_err(invalid_low_surrogate)?;
                    let decoded = char::decode_utf16([value, low])
                        .next()
                        .and_then(Result::ok)
                        .ok_or(SettingError::Invalid("invalid surrogate pair".into()))?;
                    output.push(decoded);
                } else {
                    output.push(
                        char::from_u32(u32::from(value))
                            .ok_or(SettingError::Invalid("invalid unicode scalar".into()))?,
                    );
                }
            }
            other => output.push(other),
        }
    }
    Ok(output)
}
pub(crate) fn invalid_unicode_escape(_: std::num::ParseIntError) -> SettingError {
    SettingError::Invalid("invalid unicode escape".into())
}
pub(crate) fn invalid_low_surrogate(_: std::num::ParseIntError) -> SettingError {
    SettingError::Invalid("invalid low surrogate".into())
}
pub(crate) fn find_property_split(line: &str) -> usize {
    let mut escaped = false;
    for (index, character) in line.char_indices() {
        if escaped {
            escaped = false;
        } else if character == '\\' {
            escaped = true;
        } else if matches!(character, '=' | ':') || character.is_whitespace() {
            return index;
        }
    }
    line.len()
}
pub(crate) fn escape(input: &str) -> String {
    let mut output = String::new();
    for ch in input.chars() {
        match ch {
            '\\' => output.push_str("\\\\"),
            '\n' => output.push_str("\\n"),
            '\r' => output.push_str("\\r"),
            '\t' => output.push_str("\\t"),
            '=' | ':' => {
                output.push('\\');
                output.push(ch);
            }
            ch if !ch.is_ascii() => {
                for unit in ch.encode_utf16(&mut [0; 2]).iter() {
                    let _ = write!(output, "\\u{unit:04X}");
                }
            }
            other => output.push(other),
        }
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{DEFAULT_ENCODING, PropsUtil};
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct Bean {
        name: String,
        count: i32,
    }
    struct ToggleSerialize(std::cell::Cell<bool>);
    fn failing_config(
        _: &std::collections::HashMap<String, String>,
    ) -> Result<config::Config, config::ConfigError> {
        Err(config::ConfigError::Message("intentional".into()))
    }
    impl Serialize for ToggleSerialize {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            if self.0.get() {
                Bean {
                    name: "generic".into(),
                    count: 10,
                }
                .serialize(serializer)
            } else {
                Err(serde::ser::Error::custom("intentional"))
            }
        }
    }

    #[test]
    #[allow(clippy::too_many_lines)]
    fn properties_parse_convert_mutate_and_round_trip() {
        let mut props = Props::new();
        props.load_text("# header\nname = HiTool\ncount: 42\nenabled=yes\nlong=9000\nchar=蟹\nfloat=1.5\ndouble=2.25\nshort=12\nbyte=7\ndecimal=12.50\nbig=99999999999999999999\ndate=2026-07-17T10:00:00+08:00\nescaped=hello\\ world\\nline\nemoji=\\uD83E\\uDD80\ncontinued=one\\\ntwo\ninvalid_bool=maybe\n").unwrap();
        assert_eq!(props.len(), 16);
        assert_eq!(props.get_obj("name"), Some("HiTool"));
        assert_eq!(props.get_str_or("missing", "fallback"), "fallback");
        assert_eq!(props.get_str("escaped"), Some("hello world\nline"));
        assert_eq!(props.get_str("emoji"), Some("🦀"));
        assert_eq!(Props::default(), Props::new());
        assert_eq!(props.get_int("count"), Some(42));
        assert_eq!(props.get_parse_or("missing", 5_i32), 5);
        assert_eq!(props.get_bool("enabled"), Some(true));
        assert_eq!(props.get_bool("missing"), None);
        props.set_property("disabled", "no");
        assert_eq!(props.get_bool("disabled"), Some(false));
        assert_eq!(props.get_bool("invalid_bool"), None);
        assert_eq!(props.get_long("long"), Some(9000));
        assert_eq!(props.get_char("char"), Some('蟹'));
        assert_eq!(props.get_char("missing"), None);
        assert_eq!(props.get_float("float"), Some(1.5));
        assert_eq!(props.get_double("double"), Some(2.25));
        assert_eq!(props.get_short("short"), Some(12));
        assert_eq!(props.get_byte("byte"), Some(7));
        assert_eq!(
            props.get_big_decimal("decimal").unwrap().to_string(),
            "12.50"
        );
        assert_eq!(
            props.get_big_integer("big").unwrap().to_string(),
            "99999999999999999999"
        );
        assert_eq!(props.get_enum::<i32>("count"), Some(42));
        assert!(props.get_date("date").is_some());
        assert_eq!(props.get_date("missing"), None);
        props.set_property("invalid_number", "not-a-number");
        assert_eq!(props.get_int("missing"), None);
        assert_eq!(props.get_int("invalid_number"), None);
        assert_eq!(props.get_long("missing"), None);
        assert_eq!(props.get_long("invalid_number"), None);
        assert_eq!(props.get_float("missing"), None);
        assert_eq!(props.get_float("invalid_number"), None);
        assert_eq!(props.get_double("missing"), None);
        assert_eq!(props.get_double("invalid_number"), None);
        assert_eq!(props.get_short("missing"), None);
        assert_eq!(props.get_short("invalid_number"), None);
        assert_eq!(props.get_byte("missing"), None);
        assert_eq!(props.get_byte("invalid_number"), None);
        assert_eq!(props.get_big_decimal("missing"), None);
        assert_eq!(props.get_big_decimal("invalid_number"), None);
        assert_eq!(props.get_big_integer("missing"), None);
        assert_eq!(props.get_big_integer("invalid_number"), None);
        assert_eq!(props.get_enum::<i32>("missing"), None);
        assert_eq!(props.get_enum::<i32>("invalid_number"), None);
        assert_eq!(
            props.get_and_remove_str(&["none", "continued"]),
            Some("onetwo".into())
        );
        assert_eq!(props.get_and_remove_str(&["none"]), None);
        assert_eq!(
            props.to_properties().get("name").map(String::as_str),
            Some("HiTool")
        );
        props
            .set_property("bean.name", "configured")
            .set_property("bean.count", 3);
        let bean: Bean = props.to_bean(Some("bean")).unwrap();
        assert_eq!(
            bean,
            Bean {
                name: "configured".into(),
                count: 3
            }
        );
        assert!(props.to_bean::<Bean>(Some("missing")).is_err());
        assert!(
            props
                .to_bean_with::<Bean>(Some("bean"), failing_config)
                .is_err()
        );
        props
            .fill_value(serde_json::json!({"name":"new","count":8}), Some("copy"))
            .unwrap();
        props
            .fill_value(serde_json::json!({"name":"plain","count":9}), None)
            .unwrap();
        assert_eq!(props.get_str("copy.name"), Some("new"));
        assert!(props.fill_value(serde_json::json!("scalar"), None).is_err());
        let toggle = ToggleSerialize(std::cell::Cell::new(true));
        props.fill_from(&toggle, Some("typed")).unwrap();
        toggle.0.set(false);
        assert!(props.fill_from(&toggle, None).is_err());

        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("saved.properties");
        props.store(&path).unwrap();
        let loaded = Props::from_path_with_encoding(&path, DEFAULT_ENCODING).unwrap();
        assert_eq!(loaded.get_str("emoji"), Some("🦀"));
        assert!(!loaded.is_empty());
        assert!(props.store(directory.path()).is_err());
        let mut escaped = Props::new();
        escaped.set_property("special:key", "\\\t\r\n");
        let escaped_path = directory.path().join("escaped.properties");
        escaped.store(&escaped_path).unwrap();
        assert_eq!(
            Props::from_path_with_encoding(&escaped_path, DEFAULT_ENCODING)
                .unwrap()
                .get_str("special:key"),
            Some("\\\t\r\n")
        );
    }

    #[test]
    fn properties_helpers_and_invalid_inputs_are_explicit() {
        for text in [
            "dangling=\\",
            "bad=\\u12",
            "bad=\\uZZZZ",
            "bad=\\uD83Ex",
            "bad=\\uD83E\\uZZZZ",
            "bad=\\uD83E\\u0041",
            "bad=\\uDC00",
        ] {
            let mut props = Props::new();
            assert!(props.load_text(text).is_err(), "{text}");
        }
        let mut invalid_key = Props::new();
        assert!(invalid_key.load_text("\\uZZZZ=value\n").is_err());
        assert!(unescape("dangling\\").is_err());
        let mut blank = Props::new();
        blank.load_text("\n! comment\nkeyOnly\nform=\\f\n").unwrap();
        assert_eq!(blank.get_str("keyOnly"), Some(""));
        assert_eq!(blank.get_str("form"), Some("\u{000c}"));
        assert!(Props::from_path("").is_err());
        let directory = tempfile::tempdir().unwrap();
        assert!(Props::from_path(directory.path().join("missing")).is_err());
        assert!(Props::new().load().is_err());
        let base = directory.path().join("app");
        std::fs::write(base.with_extension("properties"), "x=1\n").unwrap();
        assert_eq!(PropsUtil::get(&base).unwrap().get_int("x"), Some(1));
        assert!(
            PropsUtil::get_first_found([directory.path().join("none"), base])
                .unwrap()
                .is_some()
        );
        assert!(
            PropsUtil::get_first_found([directory.path().join("none")])
                .unwrap()
                .is_none()
        );
        assert!(!PropsUtil::get_system_props().is_empty());
        assert_eq!(Props::create(), Props::new());
        let map = IndexMap::from([("a".into(), "b".into())]);
        assert_eq!(Props::from_map(map).get_str("a"), Some("b"));
    }
}
