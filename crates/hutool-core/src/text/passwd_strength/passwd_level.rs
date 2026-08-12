//! 对齐: `cn.hutool.core.text.PasswdStrength`
//! 来源: hutool-core/src/main/java/cn/hutool/core/text/PasswdStrength.java
//!
//! 密码强度检测。

/// 对齐 Java: `PasswdStrength#PASSWD_LEVEL` 枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PasswdLevel {
    /// 弱
    Easy,
    /// 中
    Medium,
    /// 强
    Strong,
    /// 很强
    VeryStrong,
    /// 极强
    ExtremelyStrong,
}
