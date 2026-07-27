//! `SocketConfig` 的 Rust 对齐实现。
//!
//! Java 来源：`cn.hutool.socket.SocketConfig`
//! 作用：承接 Hutool 的 Socket 通讯配置语义，并映射到 Tokio 会话并发、
//! 读写超时与缓冲区限制。

use std::time::Duration;

use super::SocketRuntimeException;

/// Socket 通讯配置。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SocketConfig {
    /// Hutool `threadPoolSize`：接收与处理连接的共享并发上限。
    thread_pool_size: usize,
    read_timeout: Duration,
    write_timeout: Duration,
    read_buffer_size: usize,
    write_buffer_size: usize,
}

impl SocketConfig {
    /// 创建默认配置。
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// 获取共享线程池大小。
    #[must_use]
    pub const fn thread_pool_size(&self) -> usize {
        self.thread_pool_size
    }

    /// 设置共享线程池大小，范围为 `1..=1024`。
    pub fn set_thread_pool_size(
        &mut self,
        size: usize,
    ) -> Result<&mut Self, SocketRuntimeException> {
        if size == 0 || size > 1_024 {
            return Err(SocketRuntimeException::new(
                "thread pool size must be 1..=1024",
            ));
        }
        self.thread_pool_size = size;
        Ok(self)
    }

    /// 获取读取超时时长，`0` 表示不启用超时。
    #[must_use]
    pub const fn read_timeout(&self) -> Duration {
        self.read_timeout
    }

    /// 设置读取超时时长。
    pub fn set_read_timeout(&mut self, timeout: Duration) -> &mut Self {
        self.read_timeout = timeout;
        self
    }

    /// 获取写出超时时长，`0` 表示不启用超时。
    #[must_use]
    pub const fn write_timeout(&self) -> Duration {
        self.write_timeout
    }

    /// 设置写出超时时长。
    pub fn set_write_timeout(&mut self, timeout: Duration) -> &mut Self {
        self.write_timeout = timeout;
        self
    }

    /// 获取单次读取的最大字节数。
    #[must_use]
    pub const fn read_buffer_size(&self) -> usize {
        self.read_buffer_size
    }

    /// 设置读取缓冲区大小。
    pub fn set_read_buffer_size(
        &mut self,
        size: usize,
    ) -> Result<&mut Self, SocketRuntimeException> {
        validate_buffer(size)?;
        self.read_buffer_size = size;
        Ok(self)
    }

    /// 获取单次写入允许的最大字节数。
    #[must_use]
    pub const fn write_buffer_size(&self) -> usize {
        self.write_buffer_size
    }

    /// 设置写出缓冲区大小。
    pub fn set_write_buffer_size(
        &mut self,
        size: usize,
    ) -> Result<&mut Self, SocketRuntimeException> {
        validate_buffer(size)?;
        self.write_buffer_size = size;
        Ok(self)
    }
}

impl Default for SocketConfig {
    fn default() -> Self {
        Self {
            thread_pool_size: std::thread::available_parallelism().map_or(1, usize::from),
            read_timeout: Duration::ZERO,
            write_timeout: Duration::ZERO,
            read_buffer_size: 8_192,
            write_buffer_size: 8_192,
        }
    }
}

fn validate_buffer(size: usize) -> Result<(), SocketRuntimeException> {
    if !(1..=16 * 1024 * 1024).contains(&size) {
        return Err(SocketRuntimeException::new(
            "socket buffer size must be 1..=16777216",
        ));
    }
    Ok(())
}
