//! 对齐: `cn.hutool.core.lang.Dict`
//! 来源: hutool-core/src/main/java/cn/hutool/core/lang/Dict.java
//!
//! Rust 以 `HashMap<String, serde_json::Value>` 表达 Hutool Dict 动态袋；
//! Serde 路径覆盖 `parse` / `toBean`，反射字段注入保持 planned。

use std::collections::HashMap;

use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_json::Value;

/// 对齐 Java: `cn.hutool.core.lang.Dict`
pub type Dict = HashMap<String, Value>;
