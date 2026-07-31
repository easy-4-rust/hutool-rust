//! 对齐: `cn.hutool.core.text.PasswdStrength`
//! 来源: hutool-core/src/main/java/cn/hutool/core/text/PasswdStrength.java
//!
//! 密码强度检测。

/// 对齐 Java: `PasswdStrength#CHAR_TYPE` 枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CharType {
    /// 数字
    Num,
    /// 小写字母
    SmallLetter,
    /// 大写字母
    CapitalLetter,
    /// 其他字符
    OtherChar,
}
