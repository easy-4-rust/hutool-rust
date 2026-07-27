//! 对齐: `cn.hutool.system` (HostInfo / OsInfo / RuntimeInfo / SystemUtil 等)
//! 来源: hutool-system/src/main/java/cn/hutool/system/*.java
//! 中文说明: 提供与 Hutool 对齐的系统属性、用户信息、操作系统信息、运行时与管理信息等便携式视图

use crate::{MemoryInfo, OshiUtil, ProcessInfo};

mod host_info;
mod java_info;
mod java_spec_info;
mod java_runtime_info;
mod jvm_spec_info;
mod os_info;
mod system_util;
mod system_props_keys;
mod jvm_info;
mod user_info;

pub use host_info::HostInfo;
pub use java_info::JavaInfo;
pub use java_spec_info::JavaSpecInfo;
pub use java_runtime_info::JavaRuntimeInfo;
pub use jvm_spec_info::JvmSpecInfo;
pub use os_info::OsInfo;
pub use system_util::SystemUtil;
pub use system_props_keys::SystemPropsKeys;
pub use jvm_info::JvmInfo;
pub use user_info::UserInfo;

#[cfg(all(test, windows))]
const LINE_SEPARATOR: &str = "\r\n";
#[cfg(all(test, not(windows)))]
const LINE_SEPARATOR: &str = "\n";
#[cfg(all(test, windows))]
const PATH_SEPARATOR: char = ';';
#[cfg(all(test, not(windows)))]
const PATH_SEPARATOR: char = ':';

/// 对齐: `cn.hutool.system.RuntimeInfo`
/// 中文说明: Rust 进程/运行时内存信息，对应 Hutool 的 `RuntimeInfo`，包含最大内存、总内存、可用内存和进程内存
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RuntimeInfo {
    /// 中文说明: 最大可用内存（原生 Rust 为物理内存总量）
    pub max_memory: u64,
    /// 中文说明: 总物理内存
    pub total_memory: u64,
    /// 中文说明: 可用物理内存
    pub free_memory: u64,
    /// 中文说明: 当前进程常驻内存
    pub process_memory: u64,
}

impl RuntimeInfo {
    /// 中文说明: 采集运行时内存计数器
    /// 对齐 Java 方法: `RuntimeInfo` 构造/初始化逻辑
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

    /// 中文说明: 返回不超过原生主机限制的可用内存
    /// 对齐 Java 方法: `RuntimeInfo.getUsableMemory`
    #[must_use]
    pub fn usable_memory(self) -> u64 {
        self.free_memory.saturating_add(self.process_memory)
    }
}

/// 对齐: `java.lang.management.RuntimeMXBean` (替代)
/// 中文说明: 原生 Rust 编译器/运行时信息，替代 JVM 专有的 MXBeans
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompilationInfo {
    /// 中文说明: 编译器类型
    pub compiler: &'static str,
    /// 中文说明: 目标架构
    pub target_arch: &'static str,
    /// 中文说明: 是否启用调试断言
    pub debug_assertions: bool,
}

/// 对齐: `java.lang.management.ManagementFactory` (替代)
/// 中文说明: 便携式的原生管理信息集合，替代 JVM 的 ManagementFactory
#[derive(Debug, Clone, PartialEq)]
pub struct ManagementInfo {
    /// 中文说明: 当前进程信息
    pub process: Option<ProcessInfo>,
    /// 中文说明: 主机内存信息
    pub memory: MemoryInfo,
    /// 中文说明: 操作系统属性
    pub os: OsInfo,
    /// 中文说明: 编译属性
    pub compilation: CompilationInfo,
    /// 中文说明: 可用并行度，作为便携式的线程容量度量
    pub thread_capacity: usize,
}

#[cfg(test)]
mod tests {
    use std::{env, ffi::OsString, io, path::PathBuf};

    use super::*;
    use super::java_runtime_info::split_paths;
    use super::user_info::{first_env, non_empty, option_or_default, optional_path, result_path};

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

    fn value_or_else(value: Option<String>, fallback: &str) -> String {
        value.unwrap_or_else(|| fallback.to_owned())
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
