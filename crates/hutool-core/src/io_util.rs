//! 对齐: `cn.hutool.core.io.IoUtil`
//! 来源: hutool-core/src/main/java/cn/hutool/core/io/IoUtil.java
//!
//! Rust 版本提供 IO 操作的 idiomatic 实现。

use std::io::{self, BufRead, BufReader, BufWriter, Read, Write};

use sha2::{Digest, Sha256};

/// 对齐 Java: `cn.hutool.core.io.IoUtil`
#[derive(Debug, Clone, Copy, Default)]
pub struct IoUtil;

impl IoUtil {
    // ── 流操作 ──

    /// 对齐 Java: `IoUtil.copy(InputStream, OutputStream)`
    pub fn copy<R: Read, W: Write>(reader: &mut R, writer: &mut W) -> io::Result<u64> {
        io::copy(reader, writer)
    }

    /// 对齐 Java: `IoUtil.copy(InputStream, OutputStream, int)`
    pub fn copy_with_buffer<R: Read, W: Write>(
        reader: &mut R,
        writer: &mut W,
        buffer_size: usize,
    ) -> io::Result<u64> {
        let mut buf = vec![0u8; buffer_size];
        let mut total = 0u64;
        loop {
            let n = reader.read(&mut buf)?;
            if n == 0 {
                break;
            }
            writer.write_all(&buf[..n])?;
            total += n as u64;
        }
        Ok(total)
    }

    // ── 读取操作 ──

    /// 对齐 Java: `IoUtil.read(InputStream)`
    pub fn read_all<R: Read>(reader: &mut R) -> io::Result<Vec<u8>> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        Ok(buf)
    }

    /// 对齐 Java: `IoUtil.read(InputStream, String)`
    pub fn read_to_string<R: Read>(reader: &mut R) -> io::Result<String> {
        let mut buf = String::new();
        reader.read_to_string(&mut buf)?;
        Ok(buf)
    }

    /// 对齐 Java: `IoUtil.readLines(InputStream, String)`
    pub fn read_lines<R: Read>(reader: R) -> io::Result<Vec<String>> {
        let buf_reader = BufReader::new(reader);
        let mut lines = Vec::new();
        for line in buf_reader.lines() {
            lines.push(line?);
        }
        Ok(lines)
    }

    // ── 写入操作 ──

    /// 对齐 Java: `IoUtil.write(OutputStream, byte[])`
    pub fn write_all<W: Write>(writer: &mut W, data: &[u8]) -> io::Result<()> {
        writer.write_all(data)?;
        writer.flush()
    }

    /// 对齐 Java: `IoUtil.write(OutputStream, String)`
    pub fn write_string<W: Write>(writer: &mut W, data: &str) -> io::Result<()> {
        writer.write_all(data.as_bytes())?;
        writer.flush()
    }

    // ── 缓冲操作 ──

    /// 对齐 Java: `IoUtil.toBuffered(InputStream)`
    pub fn buffered_reader<R: Read>(reader: R) -> BufReader<R> {
        BufReader::new(reader)
    }

    /// 对齐 Java: `IoUtil.toBuffered(OutputStream)`
    pub fn buffered_writer<W: Write>(writer: W) -> BufWriter<W> {
        BufWriter::new(writer)
    }

    // ── 工具方法 ──

    /// 对齐 Java: `IoUtil.close(AutoCloseable)`
    pub fn close<W: Write>(writer: &mut W) -> io::Result<()> {
        writer.flush()
    }

    /// 对齐 Java: `IoUtil.toStream(Iterable)`
    pub fn bytes_to_hex(bytes: &[u8]) -> String {
        bytes.iter().map(|b| format!("{:02x}", b)).collect()
    }

    /// 十六进制字符串转字节
    pub fn hex_to_bytes(hex: &str) -> Result<Vec<u8>, String> {
        if hex.len() % 2 != 0 {
            return Err("hex string must have even length".to_string());
        }
        (0..hex.len())
            .step_by(2)
            .map(|i| {
                u8::from_str_radix(&hex[i..i + 2], 16)
                    .map_err(|e| format!("invalid hex at position {}: {}", i, e))
            })
            .collect()
    }

    // ── 流转换 ──

    /// 对齐 Java: `IoUtil.toMarkSupportStream(InputStream)`
    pub fn read_u8<R: Read>(reader: &mut R) -> io::Result<Option<u8>> {
        let mut buf = [0u8; 1];
        match reader.read(&mut buf)? {
            0 => Ok(None),
            _ => Ok(Some(buf[0])),
        }
    }

    /// 读取指定数量的字节
    pub fn read_exact<R: Read>(reader: &mut R, len: usize) -> io::Result<Vec<u8>> {
        let mut buf = vec![0u8; len];
        reader.read_exact(&mut buf)?;
        Ok(buf)
    }

    /// 对齐 Java: `IoUtil.readUtf8(InputStream)`
    pub fn read_utf8<R: Read>(reader: &mut R) -> io::Result<String> {
        Self::read_to_string(reader)
    }

    /// 对齐 Java: `IoUtil.writeUtf8(OutputStream, String)`
    pub fn write_utf8<W: Write>(writer: &mut W, data: &str) -> io::Result<()> {
        Self::write_string(writer, data)
    }

    /// 对齐 Java: `IoUtil.flush(Flushable)`
    pub fn flush<W: Write>(writer: &mut W) -> io::Result<()> {
        writer.flush()
    }

    /// 对齐 Java: `IoUtil.contentEquals(InputStream, InputStream)`
    pub fn content_equals<R1: Read, R2: Read>(a: &mut R1, b: &mut R2) -> io::Result<bool> {
        Ok(Self::read_all(a)? == Self::read_all(b)?)
    }

    /// 对齐 Java: `IoUtil.toStr(InputStream, Charset)`（UTF-8）
    pub fn to_str<R: Read>(reader: &mut R) -> io::Result<String> {
        Self::read_to_string(reader)
    }

    /// 对齐 Java: `IoUtil.readBytes(InputStream)`
    pub fn read_bytes<R: Read>(reader: &mut R) -> io::Result<Vec<u8>> {
        Self::read_all(reader)
    }

    /// 对齐 Java: `IoUtil.readHex(InputStream, int)`（小写 hex）
    pub fn read_hex<R: Read>(reader: &mut R, length: usize) -> io::Result<String> {
        let bytes = Self::read_exact(reader, length)?;
        Ok(Self::bytes_to_hex(&bytes))
    }

    /// 对齐 Java: `IoUtil.closeIfPossible(Closeable)`（忽略错误）
    pub fn close_if_possible<W: Write>(writer: &mut W) {
        let _ = writer.flush();
    }

    /// 对齐 Java: `IoUtil.checksum` — SHA-256 hex。
    pub fn checksum_sha256<R: Read>(reader: &mut R) -> io::Result<String> {
        let mut hasher = Sha256::new();
        let mut buffer = [0_u8; 8 * 1024];
        loop {
            let bytes_read = reader.read(&mut buffer)?;
            if bytes_read == 0 {
                break;
            }
            hasher.update(&buffer[..bytes_read]);
        }
        Ok(hex::encode(hasher.finalize()))
    }

    /// 对齐 Java: `IoUtil.checksumCRC32` / `checksumValue`
    pub fn checksum_crc32<R: Read>(reader: &mut R) -> io::Result<u32> {
        let bytes = Self::read_all(reader)?;
        Ok(crc32_ieee(&bytes))
    }

    /// 对齐 Java: `IoUtil.checksumValue`
    pub fn checksum_value<R: Read>(reader: &mut R) -> io::Result<u64> {
        Ok(u64::from(Self::checksum_crc32(reader)?))
    }

    /// 对齐 Java: `IoUtil.lineIter` — 惰性行迭代收集。
    pub fn line_iter_collect<R: Read>(reader: R) -> io::Result<Vec<String>> {
        Self::read_lines(reader)
    }

    /// 对齐 Java: `IoUtil.toUtf8Stream` / `toStream(String)`
    pub fn to_utf8_bytes(content: &str) -> Vec<u8> {
        content.as_bytes().to_vec()
    }

    /// 对齐 Java: `IoUtil.readHex64Lower` / upper helpers
    pub fn read_hex_upper<R: Read>(reader: &mut R, length: usize) -> io::Result<String> {
        Ok(Self::read_hex(reader, length)?.to_ascii_uppercase())
    }

    /// 对齐 Java: `IoUtil.contentEqualsIgnoreEOL`
    pub fn content_equals_ignore_eol<R1: Read, R2: Read>(a: R1, b: R2) -> io::Result<bool> {
        Ok(Self::read_lines(a)? == Self::read_lines(b)?)
    }
}

fn crc32_ieee(data: &[u8]) -> u32 {
    let mut crc = 0xffff_ffffu32;
    for &byte in data {
        crc ^= u32::from(byte);
        for _ in 0..8 {
            let mask = (!(crc & 1)).wrapping_add(1);
            crc = (crc >> 1) ^ (0xEDB8_8320 & mask);
        }
    }
    !crc
}
