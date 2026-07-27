//! 对齐: `cn.hutool.core.lang.SystemProps`
//! 来源: `hutool-core/src/main/java/cn/hutool/core/lang/SystemProps.java`
//! 中文说明: 提供 hutool-system 兼容层使用的系统属性/环境变量键名常量。

/// 对齐: `cn.hutool.system.SystemProps`
/// 中文说明: 常用的系统环境变量/属性键名常量，对应 Hutool `SystemProps` 中的字段。
#[derive(Debug, Clone, Copy, Default)]
pub struct SystemPropsKeys;

impl SystemPropsKeys {
    /// 中文说明: Unix 系统上用户名环境变量键名。
    /// 对齐 Java 常量: `SystemProps.USER_NAME`
    pub const USER_NAME: &'static str = "USER";
    /// 中文说明: 用户主目录环境变量键名。
    /// 对齐 Java 常量: `SystemProps.USER_HOME`
    pub const USER_HOME: &'static str = "HOME";
    /// 中文说明: 临时目录环境变量键名。
    /// 对齐 Java 常量: `SystemProps.TEMP_DIR`
    pub const TEMP_DIR: &'static str = "TMPDIR";
    /// 中文说明: Java 安装目录环境变量键名。
    /// 对齐 Java 常量: `SystemProps.JAVA_HOME`
    pub const JAVA_HOME: &'static str = "JAVA_HOME";
    /// 中文说明: Java 版本覆盖键名，用于此便携式门面。
    /// 对齐 Java 常量: `SystemProps.JAVA_VERSION`
    pub const JAVA_VERSION: &'static str = "JAVA_VERSION";
}
