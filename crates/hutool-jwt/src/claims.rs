//! 对齐: `cn.hutool.jwt.Claims`
//! 来源: `hutool-jwt/src/main/java/cn/hutool/jwt/Claims.java`
//! 中文说明: Hutool Claims 的 Rust 包装，负责动态 claim 的增删改查与 JSON 编解码。

use std::fmt;

use base64::Engine as _;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use serde_json::{Map, Value};

use super::JWTException;

/// 可变的动态 JWT claim 集合。
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Claims(Map<String, Value>);

impl Claims {
    /// 从 JSON 对象字符串解析 claims。
    pub fn parse(input: &str) -> Result<Self, JWTException> {
        let value: Value = serde_json::from_str(input)?;
        value
            .as_object()
            .cloned()
            .map(Self)
            .ok_or_else(|| JWTException::new("JWT claims must be a JSON object"))
    }

    /// 获取指定 claim。
    #[must_use]
    pub fn get_claim(&self, name: &str) -> Option<&Value> {
        self.0.get(name)
    }

    /// 返回全部 claims 的 JSON 视图。
    #[must_use]
    pub const fn claims_json(&self) -> &Map<String, Value> {
        &self.0
    }

    /// 设置单个 claim；当值为 `null` 时移除对应条目。
    pub fn set_claim(&mut self, name: impl Into<String>, value: Value) -> &mut Self {
        let name = name.into();
        if value.is_null() {
            self.0.remove(&name);
        } else {
            self.0.insert(name, value);
        }
        self
    }

    /// 批量合并 claims，逐项复用 Hutool 风格的空值移除语义。
    pub fn put_all(&mut self, values: impl IntoIterator<Item = (String, Value)>) -> &mut Self {
        for (name, value) in values {
            self.set_claim(name, value);
        }
        self
    }

    /// 按 Hutool `JSONObject#getLong` 兼容逻辑读取 `i64`。
    #[must_use]
    pub fn get_long(&self, name: &str) -> Option<i64> {
        self.0.get(name).and_then(|value| {
            value
                .as_i64()
                .or_else(|| value.as_u64().and_then(|n| i64::try_from(n).ok()))
                .or_else(|| value.as_f64().map(|n| n as i64))
        })
    }

    /// 将 claims 编码为 JWT 片段使用的 URL-safe Base64 JSON。
    pub(super) fn encode(&self) -> String {
        URL_SAFE_NO_PAD.encode(
            serde_json::to_vec(&self.0).expect("serde_json::Value maps always serialize"),
        )
    }
}

impl fmt::Display for Claims {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let json = serde_json::to_string(&self.0)
            .expect("serde_json::Value maps always serialize to valid UTF-8");
        formatter.write_str(&json)
    }
}
