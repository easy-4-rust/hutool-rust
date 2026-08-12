//! 对齐: `cn.hutool.json.JSONNull`
//! 来源: `/Users/wandl/workspaces/workspace-github/hutool/hutool-json/src/main/java/cn/hutool/json/JSONNull.java`
//! 中文说明: 提供 Hutool 风格的 `null` 单例类型表示。

use std::fmt;

/// JSON null 单例。
///
/// 对齐 Java 类: `cn.hutool.json.JSONNull`
/// 来源: `cn.hutool.json.JSONNull`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct JSONNull;

impl fmt::Display for JSONNull {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("null")
    }
}
