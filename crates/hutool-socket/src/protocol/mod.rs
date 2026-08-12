//! Hutool `cn.hutool.socket.protocol` 协议子包对齐。
//!
//! 文件拆分：
//! - `protocol.rs` → Protocol
//! - `msg_decoder.rs` → MsgDecoder
//! - `msg_encoder.rs` → MsgEncoder

mod msg_decoder;
mod msg_encoder;
mod protocol;

pub use msg_decoder::MsgDecoder;
pub use msg_encoder::MsgEncoder;
pub use protocol::Protocol;
