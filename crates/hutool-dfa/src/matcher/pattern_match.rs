//! 对齐: `cn.hutool.dfa.PatternMatch` (Rust 独有)
//! 中文说明: 单次匹配结果，包含匹配的模式索引、起止偏移和匹配文本

/// One immutable-engine match in UTF-8 byte offsets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PatternMatch {
    /// Index of the matching pattern supplied at construction time.
    pub pattern_index: usize,
    /// Inclusive UTF-8 byte start offset.
    pub start: usize,
    /// Exclusive UTF-8 byte end offset.
    pub end: usize,
    /// Matching pattern text.
    pub pattern: String,
}
