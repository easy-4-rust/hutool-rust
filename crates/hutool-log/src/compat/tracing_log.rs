use std::{
    collections::HashMap,
    fmt,
    sync::{Arc, OnceLock, RwLock},
};

use super::abstract_log::AbstractLog;

/// The native `HiTool` logger; compatibility dialect names are aliases of this type.
pub type TracingLog = AbstractLog;

use super::{format_message, global_slot};
