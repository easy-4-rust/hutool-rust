//! Boolean conversion and aggregation helpers aligned with Hutool.

use std::{any::TypeId, fmt};
use thiserror::Error;

mod boolean_error;
mod boolean_util;

pub use boolean_error::BooleanError;
pub use boolean_util::BooleanUtil;
