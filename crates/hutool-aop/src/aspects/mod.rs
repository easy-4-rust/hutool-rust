//! 对齐: `cn.hutool.aop` 包中的切面相关类
//! 来源: hutool-aop/src/main/java/cn/hutool/aop/
//! 中文说明: Hutool 切面前后置处理模块，提供 Aspect 接口及多种切面实现。

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

mod aspect;
mod simple_aspect;
mod timing_event;
mod time_interval_aspect;

pub use aspect::Aspect;
pub use simple_aspect::SimpleAspect;
pub use timing_event::TimingEvent;
pub use time_interval_aspect::TimeIntervalAspect;
