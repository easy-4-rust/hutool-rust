//! 对齐: `cn.hutool.system.oshi.NetworkIF`
//! 来源: hutool-system/src/main/java/cn/hutool/system/oshi/NetworkIF.java
//! 中文说明: 网络接口传输快照，包含接口名称、接收字节总数和发送字节总数

/// 对齐: `cn.hutool.system.oshi.NetworkIF`
/// 中文说明: 网络接口传输快照，包含接口名称、接收字节总数和发送字节总数
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NetworkInfo {
    /// 中文说明: 接口名称
    pub name: String,
    /// 中文说明: 自启动/计数器重置以来的接收字节总数
    pub received: u64,
    /// 中文说明: 自启动/计数器重置以来的发送字节总数
    pub transmitted: u64,
}
