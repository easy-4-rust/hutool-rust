//! 对齐: `cn.hutool.system.JavaRuntimeInfo`
//! 来源: `hutool-system/src/main/java/cn/hutool/system/JavaRuntimeInfo.java`
//! 中文说明: 保存 Java 运行时相关路径与版本属性，兼容 Hutool 的 JRE 运行时信息对象。

use std::{env, path::PathBuf};

use super::SystemPropsKeys;

/// 对齐: `cn.hutool.system.JavaRuntimeInfo`
/// 中文说明: Java 运行时路径属性，仅在显式配置时保留。
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct JavaRuntimeInfo {
    /// 中文说明: 运行时名称，对应 Java `java.runtime.name`。
    pub name: Option<String>,
    /// 中文说明: 运行时版本，对应 Java `java.runtime.version` / `java.version`。
    pub version: Option<String>,
    /// 中文说明: Java 安装目录，对应 Java `java.home`。
    pub home_dir: Option<PathBuf>,
    /// 中文说明: 类路径，对应 Java `java.class.path` / 环境 `CLASSPATH`。
    pub class_path: Option<String>,
    /// 中文说明: 原生库路径，对应 Java `java.library.path`。
    pub library_path: Option<String>,
    /// 中文说明: 架构数据模型，对应 Java `sun.arch.data.model`。
    pub arch_data_model: Option<String>,
    /// 中文说明: 引导类路径（如提供），对应 Java `sun.boot.class.path`。
    pub boot_class_path: Option<String>,
    /// 中文说明: 扩展目录（如提供），对应 Java `java.ext.dirs`。
    pub ext_dirs: Option<String>,
    /// 中文说明: 认可目录（如提供），对应 Java `java.endorsed.dirs`。
    pub endorsed_dirs: Option<String>,
    /// 中文说明: 类文件版本（如提供），对应 Java `java.class.version`。
    pub class_version: Option<String>,
    /// 中文说明: 协议处理器包（如提供），对应 Java `java.protocol.handler.pkgs`。
    pub protocol_packages: Option<String>,
}

impl JavaRuntimeInfo {
    /// 中文说明: 检测 Java 运行时环境变量，无需执行 Java。
    /// 对齐 Java 方法: `JavaRuntimeInfo` 的静态检测逻辑
    #[must_use]
    pub fn detect() -> Self {
        Self {
            name: env::var("JAVA_RUNTIME_NAME").ok(),
            version: env::var(SystemPropsKeys::JAVA_VERSION).ok(),
            home_dir: env::var_os(SystemPropsKeys::JAVA_HOME).map(PathBuf::from),
            class_path: env::var("CLASSPATH").ok(),
            library_path: env::var("JAVA_LIBRARY_PATH").ok(),
            arch_data_model: env::var("SUN_ARCH_DATA_MODEL").ok(),
            boot_class_path: env::var("SUN_BOOT_CLASS_PATH").ok(),
            ext_dirs: env::var("JAVA_EXT_DIRS").ok(),
            endorsed_dirs: env::var("JAVA_ENDORSED_DIRS").ok(),
            class_version: env::var("JAVA_CLASS_VERSION").ok(),
            protocol_packages: env::var("JAVA_PROTOCOL_HANDLER_PKGS").ok(),
        }
    }

    /// 中文说明: 使用主机路径分隔符拆分类路径。
    /// 对齐 Java 方法: `JavaRuntimeInfo.getClassPathArray`
    #[must_use]
    pub fn class_path_array(&self) -> Vec<PathBuf> {
        split_paths(self.class_path.as_deref())
    }

    /// 中文说明: 使用主机路径分隔符拆分原生库路径。
    /// 对齐 Java 方法: `JavaRuntimeInfo.getLibraryPathArray`
    #[must_use]
    pub fn library_path_array(&self) -> Vec<PathBuf> {
        split_paths(self.library_path.as_deref())
    }
}

pub(super) fn split_paths(value: Option<&str>) -> Vec<PathBuf> {
    value.map_or_else(Vec::new, |paths| env::split_paths(paths).collect())
}
