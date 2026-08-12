//! 对齐: `cn.hutool.core.io.FastByteBuffer`
//! 来源: hutool-core/src/main/java/cn/hutool/core/io/FastByteBuffer.java
//!
//! 快速可增长字节缓冲（Vec 实现，语义对齐 append/size/toArray）。

/// 对齐 Java 类: `cn.hutool.core.io.FastByteBuffer`
#[derive(Debug, Clone)]
pub struct FastByteBuffer {
    buf: Vec<u8>,
    /// 对齐 Java `minChunkLen`（预留增长粒度，Rust 侧仅作容量提示）。
    min_chunk_len: usize,
}

impl Default for FastByteBuffer {
    fn default() -> Self {
        Self::new()
    }
}

impl FastByteBuffer {
    /// 对齐 Java: `FastByteBuffer()`
    pub fn new() -> Self {
        Self::with_chunk(1024)
    }

    /// 对齐 Java: `FastByteBuffer(int size)`
    pub fn with_chunk(min_chunk_len: usize) -> Self {
        let min_chunk_len = if min_chunk_len == 0 {
            1024
        } else {
            min_chunk_len
        };
        Self {
            buf: Vec::with_capacity(min_chunk_len),
            min_chunk_len,
        }
    }

    /// 对齐 Java: `FastByteBuffer.append(byte)`
    pub fn append_u8(&mut self, b: u8) -> &mut Self {
        self.buf.push(b);
        self
    }

    /// 对齐 Java: `FastByteBuffer.append(byte[])`
    pub fn append(&mut self, data: &[u8]) -> &mut Self {
        self.buf.extend_from_slice(data);
        self
    }

    /// 对齐 Java: `FastByteBuffer.append(byte[], int, int)`
    pub fn append_range(&mut self, data: &[u8], offset: usize, length: usize) -> &mut Self {
        let end = (offset + length).min(data.len());
        if offset < end {
            self.buf.extend_from_slice(&data[offset..end]);
        }
        self
    }

    /// 对齐 Java: `FastByteBuffer.append(FastByteBuffer)`
    pub fn append_buffer(&mut self, other: &FastByteBuffer) -> &mut Self {
        self.buf.extend_from_slice(&other.buf);
        self
    }

    /// 对齐 Java: `FastByteBuffer.size()`
    pub fn size(&self) -> usize {
        self.buf.len()
    }

    /// 对齐 Java: `FastByteBuffer.isEmpty()`
    pub fn is_empty(&self) -> bool {
        self.buf.is_empty()
    }

    /// 对齐 Java: `FastByteBuffer.index()` — 当前块索引（单缓冲恒为 0 或 -1）。
    pub fn index(&self) -> isize {
        if self.buf.is_empty() { -1 } else { 0 }
    }

    /// 对齐 Java: `FastByteBuffer.offset()` — 当前写偏移（等于 size）。
    pub fn offset(&self) -> usize {
        self.buf.len()
    }

    /// 对齐 Java: `FastByteBuffer.array(int)` — 第 index 块；单缓冲仅 index=0。
    pub fn array(&self, index: usize) -> Option<&[u8]> {
        if index == 0 && !self.buf.is_empty() {
            Some(&self.buf)
        } else {
            None
        }
    }

    /// 对齐 Java: `FastByteBuffer.reset()`
    pub fn reset(&mut self) {
        self.buf.clear();
        if self.buf.capacity() < self.min_chunk_len {
            self.buf.reserve(self.min_chunk_len);
        }
    }

    /// 对齐 Java: `FastByteBuffer.toArray()`
    pub fn to_array(&self) -> Vec<u8> {
        self.buf.clone()
    }

    /// 对齐 Java: `FastByteBuffer.toArray(int start, int len)`
    pub fn to_array_range(&self, start: usize, len: usize) -> Vec<u8> {
        let end = (start + len).min(self.buf.len());
        if start >= end {
            Vec::new()
        } else {
            self.buf[start..end].to_vec()
        }
    }

    /// 对齐 Java: `FastByteBuffer.get(int)`
    pub fn get(&self, index: usize) -> Option<u8> {
        self.buf.get(index).copied()
    }

    /// 内部切片视图（Rust 便利）。
    pub fn as_slice(&self) -> &[u8] {
        &self.buf
    }
}
