//! 对齐: `cn.hutool.system` (SystemUtil / UserInfo / OsInfo / JavaInfo / HostInfo 等)
//! 来源: hutool-system/src/main/java/cn/hutool/system/*.java
//! 中文说明: 提供与 Hutool 对齐的系统属性、用户信息、操作系统信息、Java 运行时信息等便携式视图

use std::{
    env,
    ffi::OsString,
    fmt::{self, Write as _},
    io,
    path::PathBuf,
};

use sysinfo::System;

use crate::{MemoryInfo, OshiUtil, ProcessInfo, SystemSnapshot};

#[cfg(windows)]
const LINE_SEPARATOR: &str = "\r\n";
#[cfg(not(windows))]
const LINE_SEPARATOR: &str = "\n";
#[cfg(windows)]
const PATH_SEPARATOR: char = ';';
#[cfg(not(windows))]
const PATH_SEPARATOR: char = ':';

/// 对齐: `cn.hutool.system.SystemProps`
/// 中文说明: 常用的系统环境变量/属性键名常量，对应 Hutool `SystemProps` 中的字段
#[derive(Debug, Clone, Copy, Default)]
pub struct SystemPropsKeys;

impl SystemPropsKeys {
    /// 中文说明: Unix 系统上用户名环境变量键名
    /// 对齐 Java 方法: `SystemProps.USER_NAME`
    pub const USER_NAME: &'static str = "USER";
    /// 中文说明: 用户主目录环境变量键名
    /// 对齐 Java 方法: `SystemProps.USER_HOME`
    pub const USER_HOME: &'static str = "HOME";
    /// 中文说明: 临时目录环境变量键名
    /// 对齐 Java 方法: `SystemProps.TEMP_DIR`
    pub const TEMP_DIR: &'static str = "TMPDIR";
    /// 中文说明: Java 安装目录环境变量键名
    /// 对齐 Java 方法: `SystemProps.JAVA_HOME`
    pub const JAVA_HOME: &'static str = "JAVA_HOME";
    /// 中文说明: Java 版本覆盖键名，用于此便携式门面
    /// 对齐 Java 方法: `SystemProps.JAVA_VERSION`
    pub const JAVA_VERSION: &'static str = "JAVA_VERSION";
}

/// 对齐: `cn.hutool.system.HostInfo`
/// 中文说明: 主机身份信息快照，包含主机名和主地址
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct HostInfo {
    /// 中文说明: 主机名
    pub name: Option<String>,
    /// 中文说明: 主地址（由调用方提供时填充）
    pub address: Option<String>,
}

impl HostInfo {
    /// 中文说明: 采集便携式的主机身份信息
    /// 对齐 Java 方法: `HostInfo` 构造/初始化逻辑
    #[must_use]
    pub fn collect() -> Self {
        Self {
            name: System::host_name(),
            address: None,
        }
    }
}

/// 对齐: `cn.hutool.system.SystemProps` (操作系统属性部分)
/// 中文说明: 操作系统属性及 Hutool 兼容的系统家族判断谓词
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OsInfo {
    /// 中文说明: 目标架构
    pub arch: String,
    /// 中文说明: 操作系统名称
    pub name: String,
    /// 中文说明: 操作系统版本
    pub version: String,
    /// 中文说明: 文件系统分隔符
    pub file_separator: char,
    /// 中文说明: 原生行分隔符
    pub line_separator: &'static str,
    /// 中文说明: 搜索路径分隔符
    pub path_separator: char,
}

impl OsInfo {
    /// 中文说明: 从显式参数创建 OS 视图，适用于确定性策略检查
    /// 对齐 Java 方法: `SystemProps` 构造逻辑
    #[must_use]
    pub fn from_parts(
        arch: impl Into<String>,
        name: impl Into<String>,
        version: impl Into<String>,
    ) -> Self {
        Self {
            arch: arch.into(),
            name: name.into(),
            version: version.into(),
            file_separator: std::path::MAIN_SEPARATOR,
            line_separator: LINE_SEPARATOR,
            path_separator: PATH_SEPARATOR,
        }
    }

    /// 中文说明: 采集当前操作系统属性
    /// 对齐 Java 方法: `SystemUtil.getOsInfo`
    #[must_use]
    pub fn collect() -> Self {
        Self::from_parts(
            env::consts::ARCH,
            value_or_else(System::name(), env::consts::OS),
            System::os_version().unwrap_or_default(),
        )
    }

    fn normalized_name(&self) -> String {
        self.name.to_ascii_lowercase().replace([' ', '-'], "")
    }

    /// 中文说明: 判断操作系统名称是否匹配 AIX
    /// 对齐 Java 方法: `SystemUtil.isAix`
    #[must_use]
    pub fn is_aix(&self) -> bool {
        self.normalized_name().contains("aix")
    }

    /// 中文说明: 判断操作系统名称是否匹配 HP-UX
    /// 对齐 Java 方法: `SystemUtil.isHpUx`
    #[must_use]
    pub fn is_hp_ux(&self) -> bool {
        self.normalized_name().contains("hpux")
    }

    /// 中文说明: 判断操作系统名称是否匹配 IRIX
    /// 对齐 Java 方法: `SystemUtil.isIrix`
    #[must_use]
    pub fn is_irix(&self) -> bool {
        self.normalized_name().contains("irix")
    }

    /// 中文说明: 判断操作系统名称是否匹配 Linux
    /// 对齐 Java 方法: `SystemUtil.isLinux`
    #[must_use]
    pub fn is_linux(&self) -> bool {
        self.normalized_name().contains("linux")
    }

    /// 中文说明: 判断操作系统名称是否匹配 macOS
    /// 对齐 Java 方法: `SystemUtil.isMac`
    #[must_use]
    pub fn is_mac(&self) -> bool {
        let name = self.normalized_name();
        name.contains("mac") || name.contains("darwin")
    }

    /// 中文说明: [`Self::is_mac`] 的别名
    /// 对齐 Java 方法: `SystemUtil.isMacOSX`
    #[must_use]
    pub fn is_mac_os_x(&self) -> bool {
        self.is_mac()
    }

    /// 中文说明: 判断操作系统名称是否匹配 OS/2
    /// 对齐 Java 方法: `SystemUtil.isOS2`
    #[must_use]
    pub fn is_os2(&self) -> bool {
        self.normalized_name().contains("os/2") || self.normalized_name() == "os2"
    }

    /// 中文说明: 判断操作系统名称是否匹配 Solaris
    /// 对齐 Java 方法: `SystemUtil.isSolaris`
    #[must_use]
    pub fn is_solaris(&self) -> bool {
        self.normalized_name().contains("solaris")
    }

    /// 中文说明: 判断操作系统名称是否匹配 SunOS
    /// 对齐 Java 方法: `SystemUtil.isSunOs`
    #[must_use]
    pub fn is_sun_os(&self) -> bool {
        self.normalized_name().contains("sunos")
    }

    /// 中文说明: 判断是否为 Windows 系列操作系统
    /// 对齐 Java 方法: `SystemUtil.isWindows`
    #[must_use]
    pub fn is_windows(&self) -> bool {
        self.normalized_name().contains("windows")
    }

    fn windows_version(&self, expected: &str) -> bool {
        self.is_windows() && self.version.to_ascii_lowercase().contains(expected)
    }

    /// 中文说明: 判断是否为 Windows 2000
    /// 对齐 Java 方法: `SystemUtil.isWindows2000`
    #[must_use]
    pub fn is_windows_2000(&self) -> bool {
        self.windows_version("2000")
    }

    /// 中文说明: 判断是否为 Windows 95
    /// 对齐 Java 方法: `SystemUtil.isWindows95`
    #[must_use]
    pub fn is_windows_95(&self) -> bool {
        self.windows_version("95")
    }

    /// 中文说明: 判断是否为 Windows 98
    /// 对齐 Java 方法: `SystemUtil.isWindows98`
    #[must_use]
    pub fn is_windows_98(&self) -> bool {
        self.windows_version("98")
    }

    /// 中文说明: 判断是否为 Windows ME
    /// 对齐 Java 方法: `SystemUtil.isWindowsME`
    #[must_use]
    pub fn is_windows_me(&self) -> bool {
        self.windows_version("me")
    }

    /// 中文说明: 判断是否为 Windows NT
    /// 对齐 Java 方法: `SystemUtil.isWindowsNT`
    #[must_use]
    pub fn is_windows_nt(&self) -> bool {
        self.windows_version("nt")
    }

    /// 中文说明: 判断是否为 Windows XP
    /// 对齐 Java 方法: `SystemUtil.isWindowsXP`
    #[must_use]
    pub fn is_windows_xp(&self) -> bool {
        self.windows_version("xp")
    }

    /// 中文说明: 判断是否为 Windows 7
    /// 对齐 Java 方法: `SystemUtil.isWindows7`
    #[must_use]
    pub fn is_windows_7(&self) -> bool {
        self.windows_version("7")
    }

    /// 中文说明: 判断是否为 Windows 8（不含 8.1）
    /// 对齐 Java 方法: `SystemUtil.isWindows8`
    #[must_use]
    pub fn is_windows_8(&self) -> bool {
        self.windows_version("8") && !self.windows_version("8.1")
    }

    /// 中文说明: 判断是否为 Windows 8.1
    /// 对齐 Java 方法: `SystemUtil.isWindows8_1`
    #[must_use]
    pub fn is_windows_8_1(&self) -> bool {
        self.windows_version("8.1")
    }

    /// 中文说明: 判断是否为 Windows 10
    /// 对齐 Java 方法: `SystemUtil.isWindows10`
    #[must_use]
    pub fn is_windows_10(&self) -> bool {
        self.windows_version("10")
    }

    /// 中文说明: 判断是否为 Windows 11
    /// 对齐 Java 方法: `SystemUtil.isWindows11`
    #[must_use]
    pub fn is_windows_11(&self) -> bool {
        self.windows_version("11")
    }
}

/// Current-user and locale properties.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserInfo {
    /// User name.
    pub name: Option<String>,
    /// Home directory.
    pub home_dir: Option<PathBuf>,
    /// Current working directory.
    pub current_dir: Option<PathBuf>,
    /// Temporary directory.
    pub temp_dir: PathBuf,
    /// ISO-like language component.
    pub language: Option<String>,
    /// ISO-like country component.
    pub country: Option<String>,
}

impl UserInfo {
    /// Creates user information from explicit portable inputs.
    #[must_use]
    pub fn from_parts(
        name: Option<String>,
        home_dir: Option<PathBuf>,
        current_dir: Option<PathBuf>,
        temp_dir: PathBuf,
        locale: &str,
    ) -> Self {
        let locale = locale.split('.').next().unwrap_or_default();
        let (language, country) = locale.split_once('_').map_or_else(
            || (non_empty(locale), None),
            |(language, country)| (non_empty(language), non_empty(country)),
        );
        Self {
            name,
            home_dir,
            current_dir,
            temp_dir,
            language,
            country,
        }
    }

    /// Collects user, path, and locale properties.
    #[must_use]
    pub fn collect() -> Self {
        let locale = option_or_default(first_env(env::var("LC_ALL"), env::var("LANG")));
        Self::from_parts(
            first_env(env::var(SystemPropsKeys::USER_NAME), env::var("USERNAME")),
            optional_path(env::var_os(SystemPropsKeys::USER_HOME)),
            result_path(env::current_dir()),
            env::temp_dir(),
            &locale,
        )
    }
}

fn value_or_else(value: Option<String>, fallback: &str) -> String {
    value.unwrap_or_else(|| fallback.to_owned())
}

fn first_env(
    primary: Result<String, env::VarError>,
    secondary: Result<String, env::VarError>,
) -> Option<String> {
    primary.or(secondary).ok()
}

fn option_or_default(value: Option<String>) -> String {
    value.unwrap_or_default()
}

fn optional_path(value: Option<OsString>) -> Option<PathBuf> {
    value.map(PathBuf::from)
}

fn result_path(value: io::Result<PathBuf>) -> Option<PathBuf> {
    value.ok()
}

fn non_empty(value: &str) -> Option<String> {
    (!value.is_empty()).then(|| value.to_owned())
}

/// Rust process/runtime memory counterpart of Hutool's `RuntimeInfo`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RuntimeInfo {
    /// Maximum usable memory (physical memory on native Rust).
    pub max_memory: u64,
    /// Total physical memory.
    pub total_memory: u64,
    /// Available physical memory.
    pub free_memory: u64,
    /// Current process resident memory.
    pub process_memory: u64,
}

impl RuntimeInfo {
    /// Collects runtime memory counters.
    #[must_use]
    pub fn collect() -> Self {
        let memory = OshiUtil::memory();
        let process_memory = OshiUtil::current_process().map_or(0, |process| process.memory);
        Self {
            max_memory: memory.total,
            total_memory: memory.total,
            free_memory: memory.available,
            process_memory,
        }
    }

    /// Returns memory usable without exceeding the native host limit.
    #[must_use]
    pub fn usable_memory(self) -> u64 {
        self.free_memory.saturating_add(self.process_memory)
    }
}

/// Parsed Java version/vendor properties when a Java installation is configured.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct JavaInfo {
    /// Version string.
    pub version: Option<String>,
    /// Vendor name.
    pub vendor: Option<String>,
    /// Vendor URL.
    pub vendor_url: Option<String>,
}

impl JavaInfo {
    /// Creates explicit Java properties.
    #[must_use]
    pub fn new(
        version: Option<String>,
        vendor: Option<String>,
        vendor_url: Option<String>,
    ) -> Self {
        Self {
            version,
            vendor,
            vendor_url,
        }
    }

    /// Detects opt-in Java environment properties without spawning a JVM.
    #[must_use]
    pub fn detect() -> Self {
        Self::new(
            env::var(SystemPropsKeys::JAVA_VERSION).ok(),
            env::var("JAVA_VENDOR").ok(),
            env::var("JAVA_VENDOR_URL").ok(),
        )
    }

    /// Returns a decimal representation of the leading version components.
    #[must_use]
    pub fn version_float(&self) -> Option<f32> {
        let (major, minor) = self.version_components()?;
        format!("{major}.{minor}").parse().ok()
    }

    fn version_components(&self) -> Option<(u32, u32)> {
        let mut parts = self
            .version
            .as_deref()?
            .trim_start_matches(|character: char| !character.is_ascii_digit())
            .split(|character: char| !character.is_ascii_digit())
            .filter(|part| !part.is_empty());
        let major = parts.next()?.parse::<u32>().ok()?;
        let minor = parts
            .next()
            .and_then(|part| part.parse::<u32>().ok())
            .unwrap_or(0);
        Some((major, minor))
    }

    /// Returns the Java feature version (`1.8` becomes `8`).
    #[must_use]
    pub fn version_int(&self) -> Option<u32> {
        let (major, minor) = self.version_components()?;
        if major == 1 { Some(minor) } else { Some(major) }
    }

    /// Checks an exact feature version.
    #[must_use]
    pub fn is_version(&self, version: u32) -> bool {
        self.version_int() == Some(version)
    }

    /// Checks a minimum feature version.
    #[must_use]
    pub fn is_version_at_least(&self, version: u32) -> bool {
        self.version_int().is_some_and(|current| current >= version)
    }
}

macro_rules! property_info {
    ($name:ident { $($field:ident),+ $(,)? }) => {
        #[doc = concat!(stringify!($name), " property snapshot.")]
        #[derive(Debug, Clone, Default, PartialEq, Eq)]
        pub struct $name {
            $(
                #[doc = concat!(stringify!($field), " property.")]
                pub $field: Option<String>,
            )+
        }
    };
}

property_info!(JavaSpecInfo {
    name,
    version,
    vendor
});
property_info!(JvmSpecInfo {
    name,
    version,
    vendor
});
property_info!(JvmInfo {
    name,
    version,
    vendor,
    info
});

/// Java runtime path properties, retained only when explicitly configured.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct JavaRuntimeInfo {
    /// Runtime name.
    pub name: Option<String>,
    /// Runtime version.
    pub version: Option<String>,
    /// Java home.
    pub home_dir: Option<PathBuf>,
    /// Class path.
    pub class_path: Option<String>,
    /// Native library path.
    pub library_path: Option<String>,
    /// Architecture data model.
    pub arch_data_model: Option<String>,
    /// Boot class path when supplied.
    pub boot_class_path: Option<String>,
    /// Extension directories when supplied.
    pub ext_dirs: Option<String>,
    /// Endorsed directories when supplied.
    pub endorsed_dirs: Option<String>,
    /// Class-file version when supplied.
    pub class_version: Option<String>,
    /// Protocol handler packages when supplied.
    pub protocol_packages: Option<String>,
}

impl JavaRuntimeInfo {
    /// Detects Java runtime environment variables without executing Java.
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

    /// Splits class path using the host path separator.
    #[must_use]
    pub fn class_path_array(&self) -> Vec<PathBuf> {
        split_paths(self.class_path.as_deref())
    }

    /// Splits native library path using the host path separator.
    #[must_use]
    pub fn library_path_array(&self) -> Vec<PathBuf> {
        split_paths(self.library_path.as_deref())
    }
}

fn split_paths(value: Option<&str>) -> Vec<PathBuf> {
    value.map_or_else(Vec::new, |paths| env::split_paths(paths).collect())
}

/// Native Rust compiler/runtime information replacing JVM-only `MXBeans`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompilationInfo {
    /// Compiler family.
    pub compiler: &'static str,
    /// Target architecture.
    pub target_arch: &'static str,
    /// Whether debug assertions are enabled.
    pub debug_assertions: bool,
}

/// Portable collection of native management information.
#[derive(Debug, Clone, PartialEq)]
pub struct ManagementInfo {
    /// Current process.
    pub process: Option<ProcessInfo>,
    /// Host memory.
    pub memory: MemoryInfo,
    /// OS properties.
    pub os: OsInfo,
    /// Compilation properties.
    pub compilation: CompilationInfo,
    /// Available parallelism as the portable thread-capacity measure.
    pub thread_capacity: usize,
}

/// Hutool-aligned static system facade.
#[derive(Debug, Clone, Copy, Default)]
pub struct SystemUtil;

impl SystemUtil {
    /// Returns the current process identifier.
    #[must_use]
    pub fn current_pid() -> u32 {
        std::process::id()
    }

    /// Collects native management information.
    #[must_use]
    pub fn management_info() -> ManagementInfo {
        ManagementInfo {
            process: OshiUtil::current_process(),
            memory: OshiUtil::memory(),
            os: OsInfo::collect(),
            compilation: CompilationInfo {
                compiler: "rustc",
                target_arch: env::consts::ARCH,
                debug_assertions: cfg!(debug_assertions),
            },
            thread_capacity: std::thread::available_parallelism().map_or(1, usize::from),
        }
    }

    /// Returns JVM memory pools. Native Rust has no managed heap pools.
    #[must_use]
    pub const fn memory_pools() -> &'static [&'static str] {
        &[]
    }

    /// Returns JVM memory managers. Native Rust has no JVM managers.
    #[must_use]
    pub const fn memory_managers() -> &'static [&'static str] {
        &[]
    }

    /// Returns JVM garbage collectors. Native Rust has no JVM GC.
    #[must_use]
    pub const fn garbage_collectors() -> &'static [&'static str] {
        &[]
    }

    /// Returns Java specification properties supplied by the environment.
    #[must_use]
    pub fn java_spec_info() -> JavaSpecInfo {
        JavaSpecInfo {
            name: env::var("JAVA_SPECIFICATION_NAME").ok(),
            version: env::var("JAVA_SPECIFICATION_VERSION").ok(),
            vendor: env::var("JAVA_SPECIFICATION_VENDOR").ok(),
        }
    }

    /// Returns JVM properties supplied by the environment.
    #[must_use]
    pub fn jvm_info() -> JvmInfo {
        JvmInfo {
            name: env::var("JAVA_VM_NAME").ok(),
            version: env::var("JAVA_VM_VERSION").ok(),
            vendor: env::var("JAVA_VM_VENDOR").ok(),
            info: env::var("JAVA_VM_INFO").ok(),
        }
    }

    /// Returns JVM specification properties supplied by the environment.
    #[must_use]
    pub fn jvm_spec_info() -> JvmSpecInfo {
        JvmSpecInfo {
            name: env::var("JAVA_VM_SPECIFICATION_NAME").ok(),
            version: env::var("JAVA_VM_SPECIFICATION_VERSION").ok(),
            vendor: env::var("JAVA_VM_SPECIFICATION_VENDOR").ok(),
        }
    }

    /// Returns Java installation properties.
    #[must_use]
    pub fn java_info() -> JavaInfo {
        JavaInfo::detect()
    }

    /// Returns Java runtime path properties.
    #[must_use]
    pub fn java_runtime_info() -> JavaRuntimeInfo {
        JavaRuntimeInfo::detect()
    }

    /// Returns operating-system properties.
    #[must_use]
    pub fn os_info() -> OsInfo {
        OsInfo::collect()
    }

    /// Returns user and locale properties.
    #[must_use]
    pub fn user_info() -> UserInfo {
        UserInfo::collect()
    }

    /// Returns host identity.
    #[must_use]
    pub fn host_info() -> HostInfo {
        HostInfo::collect()
    }

    /// Returns native runtime memory.
    #[must_use]
    pub fn runtime_info() -> RuntimeInfo {
        RuntimeInfo::collect()
    }

    /// Returns total physical memory.
    #[must_use]
    pub fn total_memory() -> u64 {
        OshiUtil::memory().total
    }

    /// Returns available physical memory.
    #[must_use]
    pub fn free_memory() -> u64 {
        OshiUtil::memory().available
    }

    /// Returns the native maximum memory boundary.
    #[must_use]
    pub fn max_memory() -> u64 {
        Self::total_memory()
    }

    /// Returns portable thread execution capacity.
    #[must_use]
    pub fn total_thread_count() -> usize {
        std::thread::available_parallelism().map_or(1, usize::from)
    }

    /// Produces a stable human-readable system dump.
    #[must_use]
    pub fn system_info_dump() -> String {
        let snapshot = SystemSnapshot::collect();
        let user = Self::user_info();
        let mut output = String::new();
        let _ = writeln!(
            output,
            "host={}",
            snapshot.host_name.as_deref().unwrap_or("")
        );
        let _ = writeln!(output, "os={}", snapshot.os_name.as_deref().unwrap_or(""));
        let _ = writeln!(output, "cpus={}", snapshot.cpu_count);
        let _ = writeln!(output, "memory.total={}", snapshot.total_memory);
        let _ = writeln!(output, "memory.used={}", snapshot.used_memory);
        let _ = writeln!(output, "user={}", user.name.as_deref().unwrap_or(""));
        output
    }

    /// Writes a system dump to an injected writer.
    pub fn dump_system_info(writer: &mut dyn io::Write) -> io::Result<()> {
        writer.write_all(Self::system_info_dump().as_bytes())
    }
}

impl fmt::Display for OsInfo {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{} {} ({})", self.name, self.version, self.arch)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type OsPredicate = fn(&OsInfo) -> bool;

    struct FailingWriter;

    impl io::Write for FailingWriter {
        fn write(&mut self, _buffer: &[u8]) -> io::Result<usize> {
            Err(io::Error::other("injected"))
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    #[test]
    fn os_predicates_cover_every_hutool_family_and_version() {
        let cases: &[(&str, &str, OsPredicate)] = &[
            ("AIX", "", OsInfo::is_aix),
            ("HP-UX", "", OsInfo::is_hp_ux),
            ("IRIX", "", OsInfo::is_irix),
            ("Linux", "", OsInfo::is_linux),
            ("Darwin", "", OsInfo::is_mac),
            ("Mac OS X", "", OsInfo::is_mac_os_x),
            ("OS/2", "", OsInfo::is_os2),
            ("Solaris", "", OsInfo::is_solaris),
            ("SunOS", "", OsInfo::is_sun_os),
            ("Windows", "2000", OsInfo::is_windows_2000),
            ("Windows", "95", OsInfo::is_windows_95),
            ("Windows", "98", OsInfo::is_windows_98),
            ("Windows", "ME", OsInfo::is_windows_me),
            ("Windows", "NT", OsInfo::is_windows_nt),
            ("Windows", "XP", OsInfo::is_windows_xp),
            ("Windows", "7", OsInfo::is_windows_7),
            ("Windows", "8", OsInfo::is_windows_8),
            ("Windows", "8.1", OsInfo::is_windows_8_1),
            ("Windows", "10", OsInfo::is_windows_10),
            ("Windows", "11", OsInfo::is_windows_11),
        ];
        for (name, version, predicate) in cases {
            let os = OsInfo::from_parts("test-arch", *name, *version);
            assert!(predicate(&os), "{name} {version}");
            assert!(!predicate(&OsInfo::from_parts("x", "unknown", "")));
        }
        let windows_81 = OsInfo::from_parts("x", "Windows", "8.1");
        assert!(windows_81.is_windows());
        assert!(!windows_81.is_windows_8());
        let current = OsInfo::collect();
        assert!(!current.arch.is_empty());
        assert!(!current.name.is_empty());
        assert_eq!(current.file_separator, std::path::MAIN_SEPARATOR);
        assert_eq!(current.path_separator, PATH_SEPARATOR);
        assert_eq!(current.line_separator, LINE_SEPARATOR);
        assert!(!format!("{current}").is_empty());
    }

    #[test]
    fn java_versions_and_runtime_paths_are_deterministic() {
        let java8 = JavaInfo::new(Some("1.8.0_412".into()), None, None);
        assert_eq!(java8.version_float(), Some(1.8));
        assert_eq!(java8.version_int(), Some(8));
        assert!(java8.is_version(8));
        assert!(java8.is_version_at_least(7));
        assert!(!java8.is_version_at_least(9));

        let java17 = JavaInfo::new(
            Some("openjdk-17.0.10".into()),
            Some("vendor".into()),
            Some("https://example.invalid".into()),
        );
        assert_eq!(java17.version_int(), Some(17));
        assert!(java17.is_version(17));
        assert_eq!(
            JavaInfo::new(Some("bad".into()), None, None).version_float(),
            None
        );
        assert_eq!(JavaInfo::default().version_int(), None);
        assert!(!JavaInfo::default().is_version(0));

        let separator = PATH_SEPARATOR;
        let runtime = JavaRuntimeInfo {
            class_path: Some(format!("a{separator}b")),
            library_path: Some(format!("c{separator}d")),
            ..JavaRuntimeInfo::default()
        };
        assert_eq!(runtime.class_path_array().len(), 2);
        assert_eq!(runtime.library_path_array().len(), 2);
        assert!(JavaRuntimeInfo::default().class_path_array().is_empty());
        assert!(JavaRuntimeInfo::default().library_path_array().is_empty());
        assert!(format!("{java17:?}{runtime:?}").contains("vendor"));
    }

    #[test]
    fn live_property_runtime_and_management_facades_are_consistent() {
        assert_eq!(SystemPropsKeys::USER_NAME, "USER");
        assert_eq!(SystemPropsKeys::USER_HOME, "HOME");
        assert_eq!(SystemPropsKeys::TEMP_DIR, "TMPDIR");
        assert_eq!(SystemPropsKeys::JAVA_HOME, "JAVA_HOME");
        assert_eq!(SystemPropsKeys::JAVA_VERSION, "JAVA_VERSION");

        let host = HostInfo::collect();
        assert!(host.address.is_none());
        let user = UserInfo::collect();
        assert!(user.temp_dir.is_absolute());
        let runtime = RuntimeInfo::collect();
        assert!(runtime.total_memory > 0);
        assert!(runtime.usable_memory() >= runtime.free_memory);

        let management = SystemUtil::management_info();
        assert_eq!(
            management.process.as_ref().unwrap().pid,
            SystemUtil::current_pid()
        );
        assert_eq!(management.compilation.compiler, "rustc");
        assert!(management.thread_capacity > 0);
        assert!(SystemUtil::memory_pools().is_empty());
        assert!(SystemUtil::memory_managers().is_empty());
        assert!(SystemUtil::garbage_collectors().is_empty());
        assert!(SystemUtil::total_memory() > 0);
        assert!(SystemUtil::free_memory() <= SystemUtil::total_memory());
        assert_eq!(SystemUtil::max_memory(), SystemUtil::total_memory());
        assert!(SystemUtil::total_thread_count() > 0);

        let _ = SystemUtil::java_info();
        let _ = SystemUtil::java_runtime_info();
        let java_spec = SystemUtil::java_spec_info();
        let jvm = SystemUtil::jvm_info();
        let jvm_spec = SystemUtil::jvm_spec_info();
        assert!(format!("{java_spec:?}{jvm:?}{jvm_spec:?}").contains("Info"));
        assert!(!format!("{SystemUtil:?}").is_empty());
        assert!(!format!("{SystemPropsKeys:?}").is_empty());

        let os = SystemUtil::os_info();
        let user = SystemUtil::user_info();
        let host = SystemUtil::host_info();
        let runtime = SystemUtil::runtime_info();
        assert!(!os.arch.is_empty());
        assert!(!user.temp_dir.as_os_str().is_empty());
        assert!(host.name.is_some());
        assert!(runtime.max_memory > 0);

        let dump = SystemUtil::system_info_dump();
        assert!(dump.contains("memory.total="));
        let mut output = Vec::new();
        SystemUtil::dump_system_info(&mut output).unwrap();
        assert!(String::from_utf8(output).unwrap().contains("memory.total="));
    }

    #[test]
    fn helper_models_cover_empty_locale_and_io_failure_paths() {
        assert_eq!(non_empty(""), None);
        assert_eq!(non_empty("en"), Some("en".into()));
        assert_eq!(split_paths(None), Vec::<PathBuf>::new());
        assert_eq!(value_or_else(None, "fallback"), "fallback");
        assert_eq!(value_or_else(Some("value".into()), "fallback"), "value");
        let missing = Err(env::VarError::NotPresent);
        assert_eq!(
            first_env(missing.clone(), Ok("fallback".into())),
            Some("fallback".into())
        );
        assert_eq!(first_env(missing.clone(), missing.clone()), None);
        assert_eq!(option_or_default(None), "");
        assert_eq!(option_or_default(Some("value".into())), "value");
        assert_eq!(optional_path(None), None);
        assert_eq!(
            optional_path(Some(OsString::from("path"))),
            Some(PathBuf::from("path"))
        );
        assert_eq!(result_path(Err(io::Error::other("injected"))), None);
        assert_eq!(
            result_path(Ok(PathBuf::from("path"))),
            Some(PathBuf::from("path"))
        );

        let locale = UserInfo::from_parts(None, None, None, PathBuf::from("/tmp"), "zh_CN.UTF-8");
        assert_eq!(locale.language.as_deref(), Some("zh"));
        assert_eq!(locale.country.as_deref(), Some("CN"));
        let language_only = UserInfo::from_parts(None, None, None, PathBuf::from("/tmp"), "en");
        assert_eq!(language_only.language.as_deref(), Some("en"));
        assert_eq!(language_only.country, None);

        assert_eq!(
            JavaInfo::new(Some("17.999999999999999999999".into()), None, None).version_int(),
            Some(17)
        );
        assert_eq!(
            JavaInfo::new(Some("999999999999999999999".into()), None, None).version_int(),
            None
        );

        let mut writer = FailingWriter;
        io::Write::flush(&mut writer).unwrap();
        assert!(SystemUtil::dump_system_info(&mut writer).is_err());
    }
}
