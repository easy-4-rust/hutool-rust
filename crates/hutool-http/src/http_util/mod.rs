//! 对齐: `cn.hutool.http` (工具模块)
//! 来源: hutool-http/src/main/java/cn/hutool/http/HttpUtil.java
//! 中文说明: HTTP工具模块，包含HttpUtil和FormMap等实用工具

use indexmap::IndexMap;

mod http_util;
mod form_map;

pub use http_util::HttpUtil;
pub use form_map::FormMap;

fn extract_meta_charset(content: &str) -> Option<String> {
    let lower = content.to_ascii_lowercase();
    if !lower.contains("<meta") {
        return None;
    }
    HttpUtil::get_charset(content)
}

/// 将键值对列表收集为 `FormMap`，供表单参数处理使用。
pub fn form_map(pairs: &[(&str, &str)]) -> FormMap {
    pairs
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect()
}

/// 对齐 Java `HttpUtil` 内部的多值参数解析脚手架，当前未被直接调用。
#[allow(dead_code)]
pub fn param_list_map(pairs: &[(&str, &str)]) -> IndexMap<String, Vec<String>> {
    let mut map = IndexMap::new();
    for (k, v) in pairs {
        map.entry(k.to_string())
            .or_insert_with(Vec::new)
            .push(v.to_string());
    }
    map
}
