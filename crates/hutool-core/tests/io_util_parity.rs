//! io_util parity tests
//! 对齐: hutool-core IoUtilTest

use hutool_core::IoUtil;
use std::io::Cursor;

// ── 流操作 ──

#[test]
fn copy_basic() {
    let data = b"Hello, World!";
    let mut reader = Cursor::new(data);
    let mut writer = Cursor::new(Vec::new());
    let copied = IoUtil::copy(&mut reader, &mut writer).unwrap();
    assert_eq!(copied, 13);
    assert_eq!(writer.into_inner(), data);
}

#[test]
fn copy_empty() {
    let mut reader = Cursor::new(b"");
    let mut writer = Cursor::new(Vec::new());
    let copied = IoUtil::copy(&mut reader, &mut writer).unwrap();
    assert_eq!(copied, 0);
}

#[test]
fn copy_with_buffer() {
    let data = b"Hello, World!";
    let mut reader = Cursor::new(data);
    let mut writer = Cursor::new(Vec::new());
    let copied = IoUtil::copy_with_buffer(&mut reader, &mut writer, 5).unwrap();
    assert_eq!(copied, 13);
    assert_eq!(writer.into_inner(), data);
}

// ── 读取操作 ──

#[test]
fn read_all_basic() {
    let data = b"Hello, World!";
    let mut reader = Cursor::new(data);
    let result = IoUtil::read_all(&mut reader).unwrap();
    assert_eq!(result, data);
}

#[test]
fn read_to_string_basic() {
    let data = "Hello, World!";
    let mut reader = Cursor::new(data);
    let result = IoUtil::read_to_string(&mut reader).unwrap();
    assert_eq!(result, data);
}

#[test]
fn read_lines_basic() {
    let data = "line1\nline2\nline3";
    let reader = Cursor::new(data);
    let lines = IoUtil::read_lines(reader).unwrap();
    assert_eq!(lines, vec!["line1", "line2", "line3"]);
}

#[test]
fn read_lines_empty() {
    let reader = Cursor::new(b"");
    let lines = IoUtil::read_lines(reader).unwrap();
    assert!(lines.is_empty());
}

// ── 写入操作 ──

#[test]
fn write_all_basic() {
    let mut writer = Cursor::new(Vec::new());
    IoUtil::write_all(&mut writer, b"Hello").unwrap();
    assert_eq!(writer.into_inner(), b"Hello");
}

#[test]
fn write_string_basic() {
    let mut writer = Cursor::new(Vec::new());
    IoUtil::write_string(&mut writer, "Hello").unwrap();
    assert_eq!(writer.into_inner(), b"Hello");
}

// ── 工具方法 ──

#[test]
fn bytes_to_hex_basic() {
    assert_eq!(IoUtil::bytes_to_hex(&[0xCA, 0xFE, 0xBA, 0xBE]), "cafebabe");
    assert_eq!(IoUtil::bytes_to_hex(&[]), "");
}

#[test]
fn hex_to_bytes_basic() {
    let result = IoUtil::hex_to_bytes("cafebabe").unwrap();
    assert_eq!(result, vec![0xCA, 0xFE, 0xBA, 0xBE]);
}

#[test]
fn hex_to_bytes_empty() {
    let result = IoUtil::hex_to_bytes("").unwrap();
    assert!(result.is_empty());
}

#[test]
fn hex_to_bytes_invalid() {
    assert!(IoUtil::hex_to_bytes("xyz").is_err());
    assert!(IoUtil::hex_to_bytes("123").is_err());
}

#[test]
fn hex_roundtrip() {
    let original = vec![0, 1, 2, 127, 128, 255];
    let hex = IoUtil::bytes_to_hex(&original);
    let decoded = IoUtil::hex_to_bytes(&hex).unwrap();
    assert_eq!(decoded, original);
}

// ── 流转换 ──

#[test]
fn read_u8_basic() {
    let data = b"AB";
    let mut reader = Cursor::new(data);
    assert_eq!(IoUtil::read_u8(&mut reader).unwrap(), Some(b'A'));
    assert_eq!(IoUtil::read_u8(&mut reader).unwrap(), Some(b'B'));
    assert_eq!(IoUtil::read_u8(&mut reader).unwrap(), None);
}

#[test]
fn read_exact_basic() {
    let data = b"Hello";
    let mut reader = Cursor::new(data);
    let result = IoUtil::read_exact(&mut reader, 3).unwrap();
    assert_eq!(result, b"Hel");
    let result = IoUtil::read_exact(&mut reader, 2).unwrap();
    assert_eq!(result, b"lo");
}

/// 对齐 Java: `IoUtil.toBuffered(InputStream)` / `IoUtil.toBuffered(OutputStream)`
#[test]
fn buffered_reader_writer_test() {
    let data = b"buffered-data";
    let reader = IoUtil::buffered_reader(Cursor::new(data));
    let mut writer = IoUtil::buffered_writer(Cursor::new(Vec::new()));
    let mut reader = reader;
    let mut buf = Vec::new();
    std::io::Read::read_to_end(&mut reader, &mut buf).unwrap();
    IoUtil::write_all(&mut writer, &buf).unwrap();
    let inner = writer.into_inner().unwrap();
    assert_eq!(inner.into_inner(), data);
}

/// 对齐 Java: `IoUtil.close(AutoCloseable)` → flush
#[test]
fn close_flush_test() {
    let mut writer = Cursor::new(Vec::new());
    IoUtil::write_string(&mut writer, "close-me").unwrap();
    IoUtil::close(&mut writer).unwrap();
    assert_eq!(writer.into_inner(), b"close-me");
}

/// 对齐 Java: `IoUtilTest.readBytesTest()`
#[test]
fn read_bytes_hutool_jpg_test() {
    let bytes = include_bytes!("fixtures/hutool.jpg");
    let mut reader = Cursor::new(bytes.as_slice());
    let read = IoUtil::read_all(&mut reader).unwrap();
    assert_eq!(read.len(), 22807);
}

/// 对齐 Java: `IoUtilTest.readBytesWithLengthTest()`
#[test]
fn read_bytes_with_length_test() {
    let bytes = include_bytes!("fixtures/hutool.jpg");
    let limit = 512;
    let mut reader = Cursor::new(bytes.as_slice());
    let partial = IoUtil::read_exact(&mut reader, limit).unwrap();
    assert_eq!(partial.len(), limit);
    assert_eq!(&partial[..], &bytes[..limit]);
}

/// 对齐 Java: `IoUtilTest.readLinesTest()`
#[test]
fn read_lines_csv_fixture_test() {
    let csv = include_str!("fixtures/test_lines.csv");
    let lines = IoUtil::read_lines(Cursor::new(csv)).unwrap();
    assert!(lines.len() >= 1);
    let _: usize = lines.iter().map(|line| line.as_str().len()).sum();
}

/// 对齐 Java: `IoUtil.copy` 大缓冲
#[test]
fn copy_large_buffer_test() {
    let data = vec![7u8; 10_000];
    let mut reader = Cursor::new(data.clone());
    let mut writer = Cursor::new(Vec::new());
    let n = IoUtil::copy_with_buffer(&mut reader, &mut writer, 4096).unwrap();
    assert_eq!(n, 10_000);
    assert_eq!(writer.into_inner(), data);
}

/// 对齐 Java: `IoUtil.readUtf8` / `writeUtf8` / `contentEquals` / `readHex`
#[test]
fn read_write_utf8_content_equals_hex_test() {
    let mut reader = Cursor::new(b"abc");
    assert_eq!(IoUtil::read_utf8(&mut reader).unwrap(), "abc");
    let mut out = Cursor::new(Vec::new());
    IoUtil::write_utf8(&mut out, "xyz").unwrap();
    assert_eq!(out.into_inner(), b"xyz");
    let mut a = Cursor::new(b"same");
    let mut b = Cursor::new(b"same");
    assert!(IoUtil::content_equals(&mut a, &mut b).unwrap());
    let mut hex_src = Cursor::new([0x0a_u8, 0x0b]);
    assert_eq!(IoUtil::read_hex(&mut hex_src, 2).unwrap(), "0a0b");
}

/// 对齐 Java: IoUtil checksum / lineIter / toUtf8Stream leftovers
#[test]
fn io_util_checksum_and_stream_helpers() {
    let mut reader = Cursor::new(b"hello");
    let sha = IoUtil::checksum_sha256(&mut reader).unwrap();
    assert_eq!(sha.len(), 64);
    let mut reader = Cursor::new(b"hello");
    let crc = IoUtil::checksum_crc32(&mut reader).unwrap();
    assert_ne!(crc, 0);
    let mut reader = Cursor::new(b"hello");
    assert_eq!(IoUtil::checksum_value(&mut reader).unwrap(), u64::from(crc));
    let lines = IoUtil::line_iter_collect(Cursor::new(b"a\nb\n")).unwrap();
    assert_eq!(lines, vec!["a", "b"]);
    assert_eq!(IoUtil::to_utf8_bytes("hi"), b"hi");
    let mut hex = Cursor::new([0xab_u8, 0xcd]);
    assert_eq!(IoUtil::read_hex_upper(&mut hex, 2).unwrap(), "ABCD");
    assert!(IoUtil::content_equals_ignore_eol(Cursor::new(b"a\r\n"), Cursor::new(b"a\n")).unwrap());
}
