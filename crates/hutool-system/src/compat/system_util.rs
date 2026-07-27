//! 对齐: `cn.hutool.system.SystemUtil`
//! 来源: `hutool-system/src/main/java/cn/hutool/system/SystemUtil.java`
//! 中文说明: 提供与 Hutool 对齐的静态系统工具门面，统一暴露系统属性、用户信息、运行时与内存信息查询能力。

use std::{env, fmt::Write as _, io};

use crate::{OshiUtil, SystemSnapshot};

use super::{
    CompilationInfo, HostInfo, JavaInfo, JavaRuntimeInfo, JavaSpecInfo, JvmInfo, JvmSpecInfo,
    ManagementInfo, OsInfo, RuntimeInfo, UserInfo,
};

/// 对齐: `cn.hutool.system.SystemUtil`
/// 中文说明: 与 Hutool 对齐的静态系统工具门面，提供系统属性、内存、进程等查询方法。
#[derive(Debug, Clone, Copy, Default)]
pub struct SystemUtil;

impl SystemUtil {
    /// 中文说明: 返回当前进程标识符。
    /// 对齐 Java 方法: `SystemUtil.getCurrentPID`
    #[must_use]
    pub fn current_pid() -> u32 {
        std::process::id()
    }

    /// 中文说明: 采集原生管理信息。
    /// 对齐 Java 方法: `SystemUtil.getManagementInfo`
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

    /// 中文说明: 返回 JVM 内存池列表；Rust 无托管堆池，固定返回空切片。
    /// 对齐 Java 方法: `SystemUtil.getMemoryPools`
    #[must_use]
    pub const fn memory_pools() -> &'static [&'static str] {
        &[]
    }

    /// 中文说明: 返回 JVM 内存管理器列表；Rust 无 JVM 管理器，固定返回空切片。
    /// 对齐 Java 方法: `SystemUtil.getMemoryManagers`
    #[must_use]
    pub const fn memory_managers() -> &'static [&'static str] {
        &[]
    }

    /// 中文说明: 返回 JVM 垃圾回收器列表；Rust 无 JVM GC，固定返回空切片。
    /// 对齐 Java 方法: `SystemUtil.getGarbageCollectors`
    #[must_use]
    pub const fn garbage_collectors() -> &'static [&'static str] {
        &[]
    }

    /// 中文说明: 返回环境提供的 Java 规范属性。
    /// 对齐 Java 方法: `SystemUtil.getJavaSpecInfo`
    #[must_use]
    pub fn java_spec_info() -> JavaSpecInfo {
        JavaSpecInfo::detect()
    }

    /// 中文说明: 返回环境提供的 JVM 属性。
    /// 对齐 Java 方法: `SystemUtil.getJvmInfo`
    #[must_use]
    pub fn jvm_info() -> JvmInfo {
        JvmInfo {
            name: env::var("JAVA_VM_NAME").ok(),
            version: env::var("JAVA_VM_VERSION").ok(),
            vendor: env::var("JAVA_VM_VENDOR").ok(),
            info: env::var("JAVA_VM_INFO").ok(),
        }
    }

    /// 中文说明: 返回环境提供的 JVM 规范属性。
    /// 对齐 Java 方法: `SystemUtil.getJvmSpecInfo`
    #[must_use]
    pub fn jvm_spec_info() -> JvmSpecInfo {
        JvmSpecInfo::detect()
    }

    /// 中文说明: 返回 Java 安装属性。
    /// 对齐 Java 方法: `SystemUtil.getJavaInfo`
    #[must_use]
    pub fn java_info() -> JavaInfo {
        JavaInfo::detect()
    }

    /// 中文说明: 返回 Java 运行时路径属性。
    /// 对齐 Java 方法: `SystemUtil.getJavaRuntimeInfo`
    #[must_use]
    pub fn java_runtime_info() -> JavaRuntimeInfo {
        JavaRuntimeInfo::detect()
    }

    /// 中文说明: 返回操作系统属性。
    /// 对齐 Java 方法: `SystemUtil.getOsInfo`
    #[must_use]
    pub fn os_info() -> OsInfo {
        OsInfo::collect()
    }

    /// 中文说明: 返回用户和区域设置属性。
    /// 对齐 Java 方法: `SystemUtil.getUserInfo`
    #[must_use]
    pub fn user_info() -> UserInfo {
        UserInfo::collect()
    }

    /// 中文说明: 返回主机身份信息。
    /// 对齐 Java 方法: `SystemUtil.getHostInfo`
    #[must_use]
    pub fn host_info() -> HostInfo {
        HostInfo::collect()
    }

    /// 中文说明: 返回原生运行时内存信息。
    /// 对齐 Java 方法: `SystemUtil.getRuntimeInfo`
    #[must_use]
    pub fn runtime_info() -> RuntimeInfo {
        RuntimeInfo::collect()
    }

    /// 中文说明: 返回总物理内存。
    /// 对齐 Java 方法: `SystemUtil.getTotalMemory`
    #[must_use]
    pub fn total_memory() -> u64 {
        OshiUtil::memory().total
    }

    /// 中文说明: 返回可用物理内存。
    /// 对齐 Java 方法: `SystemUtil.getFreeMemory`
    #[must_use]
    pub fn free_memory() -> u64 {
        OshiUtil::memory().available
    }

    /// 中文说明: 返回原生最大内存边界。
    /// 对齐 Java 方法: `SystemUtil.getMaxMemory`
    #[must_use]
    pub fn max_memory() -> u64 {
        Self::total_memory()
    }

    /// 中文说明: 返回便携式的线程执行容量。
    /// 对齐 Java 方法: `SystemUtil.getTotalThreadCount`
    #[must_use]
    pub fn total_thread_count() -> usize {
        std::thread::available_parallelism().map_or(1, usize::from)
    }

    /// 中文说明: 生成稳定的、人类可读的系统信息转储。
    /// 对齐 Java 方法: `SystemUtil.getSystemInfoDump`
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

    /// 中文说明: 将系统信息转储写入注入的写入器。
    /// 对齐 Java 方法: `SystemUtil.dumpSystemInfo`
    pub fn dump_system_info(writer: &mut dyn io::Write) -> io::Result<()> {
        writer.write_all(Self::system_info_dump().as_bytes())
    }
}
