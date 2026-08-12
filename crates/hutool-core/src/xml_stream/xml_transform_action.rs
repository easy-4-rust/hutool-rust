//! Bounded streaming XML reader, visitor, transformer, and writer.

/// 对齐: `cn.hutool.core.xml.XmlStream`
/// XML转换动作

/// Action returned by a streaming transform callback.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum XmlTransformAction {
    /// Copy the current event to the target.
    Keep,
    /// Omit the current event from the target.
    Drop,
}
