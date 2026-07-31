//! 对齐: `cn.hutool.http.HttpCookie`
//! 来源: hutool-http/src/main/java/cn/hutool/http/HttpCookie.java
//! 中文说明: HTTP Cookie实现，解析Set-Cookie头部的名称/值对

/// Simple cookie name/value pair parsed from `Set-Cookie` (Hutool `HttpCookie` subset).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HttpCookie {
    name: String,
    value: String,
}

impl HttpCookie {
    /// Creates a cookie with the given name and value.
    ///
    /// Java: `java.net.HttpCookie` name/value accessors used by Hutool.
    #[must_use]
    pub fn new(name: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            value: value.into(),
        }
    }

    /// Returns the cookie name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the cookie value.
    #[must_use]
    pub fn value(&self) -> &str {
        &self.value
    }
}
