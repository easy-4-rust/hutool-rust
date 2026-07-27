//! 对齐: 通用日志模块 helper（cn.hutool.log 顶层）
//! 来源: hutool-log/src/main/java/cn/hutool/log/StaticLog.java
//!
//! 中文说明: Hutool 日志 `{}` 模板替换 + 全局日志工厂 helper。
//! Java 端 `String.format(format, args)` 的 `{}` 占位符由本模块实现；
//! `global_slot()` 提供进程级单例 `LogFactory` 访问。

use std::fmt;
use std::sync::{OnceLock, RwLock};

use crate::log_factory::LogFactory;

/// 按 `{}` 占位符替换模板，等价 Java `String.format(format, args)` 的 hutool 风格。
///
/// 对应 Java `cn.hutool.log.StaticLog.format` 私有工具。
pub fn format_message(template: &str, arguments: &[&dyn fmt::Display]) -> String {
    if arguments.is_empty() {
        return template.to_owned();
    }
    let mut result = String::with_capacity(template.len());
    let mut remaining = template;
    let mut arguments = arguments.iter();
    while let Some(index) = remaining.find("{}") {
        result.push_str(&remaining[..index]);
        if let Some(argument) = arguments.next() {
            result.push_str(&argument.to_string());
        } else {
            result.push_str("{}");
            remaining = &remaining[index + 2..];
            result.push_str(remaining);
            return result;
        }
        remaining = &remaining[index + 2..];
    }
    result.push_str(remaining);
    result
}

/// 进程级 `LogFactory` 单例槽。
///
/// 对应 Java `cn.hutool.log.StaticLogFactory` 静态字段。
pub(crate) fn global_slot() -> &'static RwLock<LogFactory> {
    static FACTORY: OnceLock<RwLock<LogFactory>> = OnceLock::new();
    FACTORY.get_or_init(|| RwLock::new(LogFactory::default()))
}
