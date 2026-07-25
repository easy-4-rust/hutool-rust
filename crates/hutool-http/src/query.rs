//! 对齐: `cn.hutool.http.UrlQuery`
//! 来源: hutool-http/src/main/java/cn/hutool/http/UrlQuery.java
//! 中文说明: URL查询参数解析和构建，支持有序参数映射和URL编码

use hutool_core::{percent_decode, Rfc3986, UrlUtil};
use indexmap::IndexMap;
use std::borrow::Cow;

/// application/x-www-form-urlencoded component encoding (space → `+`).
fn encode_form_component(input: &str) -> String {
    let encoded = Rfc3986::encode_all(input);
    encoded.replace("%20", "+")
}

/// Ordered query parameter map matching Hutool `UrlQuery`.
#[derive(Debug, Clone, Default)]
pub struct QueryMap {
    pairs: IndexMap<String, Option<String>>,
    form_url_encoded: bool,
}

impl QueryMap {
    /// Creates an empty query map.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a query map with optional form-urlencoded semantics (`+` → space on decode).
    pub fn with_form_encoding(form_url_encoded: bool) -> Self {
        Self {
            pairs: IndexMap::new(),
            form_url_encoded,
        }
    }

    /// Parses a query string or URL fragment, optionally stripping the path before `?`.
    pub fn parse(input: &str, auto_remove_path: bool, form_url_encoded: bool) -> Self {
        let mut map = Self::with_form_encoding(form_url_encoded);
        if input.trim().is_empty() {
            return map;
        }
        let mut query = input;
        if auto_remove_path {
            if let Some(pos) = query.find('?') {
                query = &query[pos + 1..];
                if query.trim().is_empty() {
                    return map;
                }
            } else if query.starts_with("http://") || query.starts_with("https://") {
                return map;
            }
        }
        map.do_parse(query);
        map
    }

    /// Builds a query string; `encode` controls percent-encoding (`None` keeps raw pairs).
    pub fn build(&self, encode: bool) -> String {
        if self.pairs.is_empty() {
            return String::new();
        }
        let mut out = String::new();
        for (key, value) in &self.pairs {
            if !out.is_empty() {
                out.push('&');
            }
            if encode {
                if self.form_url_encoded {
                    // Hutool UrlQuery form mode ≈ encode-all (x-www-form-urlencoded)
                    out.push_str(&encode_form_component(key));
                } else {
                    out.push_str(&Rfc3986::encode_query_param_name(key));
                }
            } else {
                out.push_str(key);
            }
            if let Some(value) = value {
                out.push('=');
                if encode {
                    if self.form_url_encoded {
                        out.push_str(&encode_form_component(value));
                    } else {
                        out.push_str(&Rfc3986::encode_query_param_value(value));
                    }
                } else {
                    out.push_str(value);
                }
            }
        }
        out
    }

    /// Returns the ordered key/value pairs.
    pub fn pairs(&self) -> &IndexMap<String, Option<String>> {
        &self.pairs
    }

    /// Inserts a key/value pair.
    pub fn insert(&mut self, key: String, value: Option<String>) {
        self.pairs.insert(key, value);
    }

    /// Builds from flat string pairs without encoding.
    pub fn from_flat(params: &IndexMap<String, String>) -> Self {
        let mut map = Self::new();
        for (key, value) in params {
            map.insert(key.clone(), Some(value.clone()));
        }
        map
    }

    /// Builds from multi-value pairs without encoding.
    pub fn from_multi(params: &IndexMap<String, Vec<String>>) -> Self {
        let mut map = Self::new();
        for (key, values) in params {
            for value in values {
                map.insert(key.clone(), Some(value.clone()));
            }
        }
        map
    }

    /// Inserts pairs from a string-keyed map (values use `Display`).
    pub fn add_all<I, V>(mut self, params: I) -> Self
    where
        I: IntoIterator<Item = (String, V)>,
        V: ToString,
    {
        for (key, value) in params {
            if key.is_empty() {
                continue;
            }
            self.insert(key, Some(value.to_string()));
        }
        self
    }

    fn do_parse(&mut self, query: &str) {
        let mut name: Option<String> = None;
        let mut pos = 0;
        let chars: Vec<char> = query.chars().collect();
        let len = chars.len();
        let mut i = 0;
        while i < len {
            match chars[i] {
                '=' if name.is_none() => {
                    name = Some(chars[pos..i].iter().collect());
                    pos = i + 1;
                }
                '&' => {
                    let segment: String = chars[pos..i].iter().collect();
                    self.add_param(name.take(), segment);
                    if i + 4 < len && &query[i + 1..i + 5] == "amp;" {
                        i += 4;
                    }
                    pos = i + 1;
                }
                _ => {}
            }
            i += 1;
        }
        let tail: String = chars[pos..].iter().collect();
        self.add_param(name, tail);
    }

    fn add_param(&mut self, key: Option<String>, value: String) {
        if let Some(key) = key {
            let decoded_key = decode_component(&key, self.form_url_encoded);
            let decoded_value = decode_component(&value, self.form_url_encoded);
            self.pairs.insert(decoded_key, Some(decoded_value));
        } else if !value.is_empty() {
            self.pairs
                .insert(decode_component(&value, self.form_url_encoded), None);
        }
    }
}

/// Normalizes a parameter segment (Hutool `HttpUtil.normalizeParams`).
pub fn normalize_params(param_part: &str, encode: bool) -> String {
    if param_part.is_empty() {
        return param_part.to_string();
    }
    let mut builder = String::with_capacity(param_part.len() + 16);
    let chars: Vec<char> = param_part.chars().collect();
    let len = chars.len();
    let mut name: Option<String> = None;
    let mut pos = 0;
    let mut i = 0;
    while i < len {
        match chars[i] {
            '=' if name.is_none() => {
                name = Some(if pos == i {
                    String::new()
                } else {
                    chars[pos..i].iter().collect()
                });
                pos = i + 1;
            }
            '&' => {
                if name.is_none() {
                    if pos != i {
                        let key: String = chars[pos..i].iter().collect();
                        builder.push_str(&encode_name(&key, encode));
                        builder.push('=');
                    }
                } else {
                    let key = name.take().unwrap_or_default();
                    let value: String = chars[pos..i].iter().collect();
                    builder.push_str(&encode_name(&key, encode));
                    builder.push('=');
                    builder.push_str(&encode_value(&value, encode));
                    builder.push('&');
                }
                pos = i + 1;
            }
            _ => {}
        }
        i += 1;
    }
    let mut had_name = false;
    if let Some(key) = name {
        builder.push_str(&encode_name(&key, encode));
        builder.push('=');
        had_name = true;
    }
    if pos != i {
        if !had_name && pos > 0 {
            builder.push('=');
        }
        let tail: String = chars[pos..].iter().collect();
        builder.push_str(&encode_value(&tail, encode));
    }
    if builder.ends_with('&') {
        builder.pop();
    }
    builder
}

fn encode_name(value: &str, encode: bool) -> String {
    if encode {
        Rfc3986::encode_query_param_name(value)
    } else {
        value.to_string()
    }
}

fn encode_value(value: &str, encode: bool) -> String {
    if encode {
        Rfc3986::encode_query_param_value(value)
    } else {
        value.to_string()
    }
}

fn decode_component(raw: &str, form: bool) -> String {
    if form {
        UrlUtil::decode(raw)
    } else {
        percent_decode(raw).unwrap_or_else(|_| raw.to_string())
    }
}

/// Splits `urlWithParams` into URL and query sections.
pub fn split_url_params(input: &str) -> (Option<Cow<'_, str>>, Cow<'_, str>) {
    if let Some(pos) = input.find('?') {
        let url = &input[..pos];
        let params = &input[pos + 1..];
        if params.is_empty() {
            (Some(Cow::Borrowed(url)), Cow::Borrowed(""))
        } else {
            (Some(Cow::Borrowed(url)), Cow::Borrowed(params))
        }
    } else if !input.contains('=') {
        (Some(Cow::Borrowed(input)), Cow::Borrowed(""))
    } else {
        (None, Cow::Borrowed(input))
    }
}
