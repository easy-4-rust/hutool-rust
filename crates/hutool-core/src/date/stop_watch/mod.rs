//! 对齐: `cn.hutool.core.date.StopWatch`
//! 来源: hutool-core StopWatch（Spring Framework 风格秒表）

use std::fmt;
use std::time::{Duration, Instant};

mod time_unit;
mod task_info;
mod stop_watch;

pub use time_unit::TimeUnit;
pub use task_info::TaskInfo;
pub use stop_watch::StopWatch;
