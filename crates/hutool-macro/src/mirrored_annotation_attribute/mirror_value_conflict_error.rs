//! 对齐: `cn.hutool.core.annotation.MirroredAnnotationAttribute`

/// 镜像属性值冲突异常。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MirrorValueConflictError {
    /// 冲突描述信息。
    pub message: String,
}

impl std::fmt::Display for MirrorValueConflictError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for MirrorValueConflictError {}
