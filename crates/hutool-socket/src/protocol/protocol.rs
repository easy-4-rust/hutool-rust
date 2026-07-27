//! `Protocol` 的 Rust 对齐实现。
//!
//! Java 来源：`cn.hutool.socket.protocol.Protocol`
//! 作用：组合 `MsgEncoder` 与 `MsgDecoder`，作为统一的协议门面。

use crate::protocol::msg_decoder::MsgDecoder;
use crate::protocol::msg_encoder::MsgEncoder;

/// 协议接口（解码 + 编码）。
///
/// 对齐: cn.hutool.socket.protocol.Protocol
/// 来源: hutool-socket/src/main/java/cn/hutool/socket/protocol/Protocol.java
/// 中文说明: 组合 MsgDecoder + MsgEncoder 的统一协议门面。
pub trait Protocol<T>: MsgDecoder<T> + MsgEncoder<T> {}

impl<T, P: MsgDecoder<T> + MsgEncoder<T>> Protocol<T> for P {}