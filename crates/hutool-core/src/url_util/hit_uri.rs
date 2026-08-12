//! 对齐: `cn.hutool.core.util.URLUtil`
//! 来源: hutool-core/src/main/java/cn/hutool/core/util/URLUtil.java
//!
//! Rust 版本提供 URL 操作的 idiomatic 实现。

/// 对齐 Java `java.net.URI` 的轻量封装,用于 `URLUtil::to_uri` 返回值。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HitUri {
    pub(crate) raw: String,
}

impl HitUri {
    /// 返回 URI 原始字符串。
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.raw
    }

    /// 对齐 Java: `URI.getPath()`
    #[must_use]
    pub fn path(&self) -> Option<&str> {
        extract_path(&self.raw)
    }

    /// 对齐 Java: `URI.resolve(String)`
    #[must_use]
    pub fn resolve(&self, other: &str) -> Self {
        let base_path = self.path().unwrap_or("");
        let resolved = resolve_path(base_path, other);
        Self {
            raw: if resolved.starts_with('/') {
                resolved
            } else {
                format!("/{resolved}")
            },
        }
    }
}

impl std::fmt::Display for HitUri {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.raw)
    }
}

use super::{extract_path, resolve_path};
