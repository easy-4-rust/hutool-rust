//! 对齐: `cn.hutool.core.util.XmlUtil`
//! 来源: hutool-core/src/main/java/cn/hutool/core/util/XmlUtil.java
//!
//! Rust 版本基于 `quick-xml` 提供 DOM 风格 XML 操作。

use indexmap::IndexMap;

use super::xml_child::XmlChild;

/// XML 元素节点。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XmlNode {
    /// 标签名。
    pub tag: String,
    /// 属性集合。
    pub attributes: IndexMap<String, String>,
    /// 子节点。
    pub children: Vec<XmlChild>,
}
