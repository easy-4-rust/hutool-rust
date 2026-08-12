//! 对齐: `cn.hutool.core.annotation.RelationType`

/// 对齐 Java enum: `cn.hutool.core.annotation.RelationType`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RelationType {
    /// 镜像。
    MirrorFor,
    /// 别名。
    AliasFor,
    /// 强制别名。
    ForceAliasFor,
}
