//! Hutool-aligned before/after aspects.

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
