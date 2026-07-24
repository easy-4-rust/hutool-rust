//! Provider capability model used by the thin Hutool-compatible facade.

#![allow(missing_docs, clippy::enum_glob_use, clippy::match_same_arms)]

use crate::Message;
use serde_json::{Map, Value, json};
use std::{path::PathBuf, sync::Arc};

mod stream_callback;
mod video_parameter;
mod operation;
mod ai_response;

pub use stream_callback::StreamCallback;
pub use video_parameter::VideoParameter;
pub use operation::Operation;
pub use ai_response::AIResponse;
