//! Bounded streaming XML reader, visitor, transformer, and writer.

/// 对齐: `cn.hutool.core.xml.XmlStream`
/// 命名空间模式

/// Namespace handling applied when XML names are copied into the DOM.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum NamespaceMode {
    /// Keep qualified names such as `soap:Envelope`.
    #[default]
    Preserve,
    /// Keep only the local part such as `Envelope`.
    LocalName,
}

