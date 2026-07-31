//! 对齐: `cn.hutool.core.util.XmlUtil`
//! 来源: hutool-core/src/main/java/cn/hutool/core/util/XmlUtil.java
//!
//! Rust 版本基于 `quick-xml` 提供 DOM 风格 XML 操作。

use super::xml_node::XmlNode;

/// XML 文档根节点。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XmlDocument {
    /// 根元素。
    pub root: XmlNode,
}
