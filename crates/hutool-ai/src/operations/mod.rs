//! Provider capability model used by the thin Hutool-compatible facade.

#![allow(missing_docs, clippy::enum_glob_use, clippy::match_same_arms)]

mod ai_response;
mod operation;
mod stream_callback;
mod video_parameter;

pub use ai_response::AIResponse;
pub use operation::Operation;
pub use stream_callback::StreamCallback;
pub use video_parameter::VideoParameter;
