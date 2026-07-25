//! 对齐: `cn.hutool.aop` 包中的计时事件
//! 来源: hutool-aop/src/main/java/cn/hutool/aop/
//! 中文说明: 计时事件数据结构，记录一次切面拦截的调用耗时信息。

use crate::Method;
use parking_lot::Mutex;
use std::{
    any::type_name,
    collections::HashMap,
    fmt,
    sync::Arc,
    thread::{self, ThreadId},
    time::{Duration, Instant},
};

/// 对齐: Hutool 计时切面的事件记录
/// 中文说明: 一次已完成的计时调用事件，包含目标类型、方法名、耗时和返回值信息。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TimingEvent {
    /// Rust type name of the target.
    pub target_type: &'static str,
    /// Operation name.
    pub method: String,
    /// Measured wall-clock duration.
    pub elapsed: Duration,
    /// Debug-formatted return value, when present.
    pub return_value: Option<String>,
}
