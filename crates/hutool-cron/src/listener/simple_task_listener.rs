//! 对齐: `cn.hutool.cron.listener.SimpleTaskListener`
//! 来源: hutool-cron/src/main/java/cn/hutool/cron/listener/SimpleTaskListener.java
//! 中文说明: 不执行任何操作的监听器实现，便于按需覆盖个别方法。

use super::TaskListener;

/// 对齐: `cn.hutool.cron.listener.SimpleTaskListener`
/// 中文说明: 空操作监听器。
#[derive(Debug, Clone, Copy, Default)]
pub struct SimpleTaskListener;

impl TaskListener for SimpleTaskListener {}
