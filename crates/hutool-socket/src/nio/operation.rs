//! `Operation` 的 Rust 对齐实现。
//!
//! Java 来源：`cn.hutool.socket.nio.Operation`
//! 作用：封装 SelectionKey 的操作位掩码（READ/WRITE/CONNECT/ACCEPT）。

/// SelectionKey Operation 枚举封装。
///
/// 对齐: cn.hutool.socket.nio.Operation
/// 来源: hutool-socket/src/main/java/cn/hutool/socket/nio/Operation.java
/// 中文说明: READ/WRITE/CONNECT/ACCEPT 四种操作，对应 Java NIO SelectionKey 掩码。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Operation {
    /// 读操作（对应 `SelectionKey.OP_READ = 1`）。
    Read = 1,
    /// 写操作（对应 `SelectionKey.OP_WRITE = 4`）。
    Write = 4,
    /// 连接就绪（对应 `SelectionKey.OP_CONNECT = 8`）。
    Connect = 8,
    /// 接受连接就绪（对应 `SelectionKey.OP_ACCEPT = 16`）。
    Accept = 16,
}

impl Operation {
    /// 返回与 Java `SelectionKey` 兼容的位值。
    #[must_use]
    pub const fn value(self) -> u8 {
        self as u8
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn values_match_java_selection_key_constants() {
        assert_eq!(Operation::Read.value(), 1);
        assert_eq!(Operation::Write.value(), 4);
        assert_eq!(Operation::Connect.value(), 8);
        assert_eq!(Operation::Accept.value(), 16);
    }
}