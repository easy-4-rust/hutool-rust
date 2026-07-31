//! 对齐: `cn.hutool.http.FormMap`
//! 来源: hutool-http/src/main/java/cn/hutool/http/FormMap.java
//! 中文说明: 表单参数映射，有序键值对集合，支持URL编码和表单提交

use indexmap::IndexMap;

/// Convenience alias for building ordered form maps in tests.
pub type FormMap = IndexMap<String, String>;
