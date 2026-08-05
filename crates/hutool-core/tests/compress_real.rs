//! compress module real functional tests
//! 对齐: hutool-core ZipUtilTest/GzipTest

use hutool_core::{Deflate, Gzip, ZipLimits, ZipWriter};
use std::io::Cursor;

#[test]
fn deflate_inflate_roundtrip() {
    let input = b"Hello, World! This is a test of deflate compression.";
    let mut deflater = Deflate::new(input.as_slice(), Vec::new(), false);
    deflater.deflater(6).unwrap();
    let (_, compressed) = deflater.into_inner();
    assert!(!compressed.is_empty());
    let mut inflater = Deflate::new(compressed.as_slice(), Vec::new(), false);
    inflater.inflater().unwrap();
    let (_, decompressed) = inflater.into_inner();
    assert_eq!(decompressed, input);
}

#[test]
fn deflate_empty_input() {
    let mut deflater = Deflate::new(&b""[..], Vec::new(), false);
    deflater.deflater(6).unwrap();
    let (_, compressed) = deflater.into_inner();
    let mut inflater = Deflate::new(compressed.as_slice(), Vec::new(), false);
    inflater.inflater().unwrap();
    let (_, decompressed) = inflater.into_inner();
    assert!(decompressed.is_empty());
}

#[test]
fn deflate_large_input() {
    let input = vec![0u8; 10000];
    let mut deflater = Deflate::new(input.as_slice(), Vec::new(), false);
    deflater.deflater(6).unwrap();
    let (_, compressed) = deflater.into_inner();
    assert!(compressed.len() < input.len());
    let mut inflater = Deflate::new(compressed.as_slice(), Vec::new(), false);
    inflater.inflater().unwrap();
    let (_, decompressed) = inflater.into_inner();
    assert_eq!(decompressed, input);
}

#[test]
fn deflate_nowrap_roundtrip() {
    let input = b"Hello, World!";
    let mut deflater = Deflate::new(input.as_slice(), Vec::new(), true);
    deflater.deflater(6).unwrap();
    let (_, compressed) = deflater.into_inner();
    let mut inflater = Deflate::new(compressed.as_slice(), Vec::new(), true);
    inflater.inflater().unwrap();
    let (_, decompressed) = inflater.into_inner();
    assert_eq!(decompressed, input);
}

#[test]
fn deflate_level_comparison() {
    let input = vec![0u8; 1000];
    let mut low = Deflate::new(input.as_slice(), Vec::new(), false);
    low.deflater(1).unwrap();
    let (_, compressed_low) = low.into_inner();
    let mut high = Deflate::new(input.as_slice(), Vec::new(), false);
    high.deflater(9).unwrap();
    let (_, compressed_high) = high.into_inner();
    assert!(compressed_high.len() <= compressed_low.len());
}

#[test]
fn gzip_ungzip_roundtrip() {
    let input = b"Hello, World! This is a test of gzip compression.";
    let mut gz = Gzip::new(input.as_slice(), Vec::new());
    gz.gzip().unwrap();
    let (_, compressed) = gz.into_inner();
    assert!(!compressed.is_empty());
    let mut ungz = Gzip::new(compressed.as_slice(), Vec::new());
    ungz.ungzip().unwrap();
    let (_, decompressed) = ungz.into_inner();
    assert_eq!(decompressed, input);
}

#[test]
fn gzip_empty_input() {
    let mut gz = Gzip::new(&b""[..], Vec::new());
    gz.gzip().unwrap();
    let (_, compressed) = gz.into_inner();
    let mut ungz = Gzip::new(compressed.as_slice(), Vec::new());
    ungz.ungzip().unwrap();
    let (_, decompressed) = ungz.into_inner();
    assert!(decompressed.is_empty());
}

#[test]
fn gzip_large_input() {
    let input = vec![42u8; 10000];
    let mut gz = Gzip::new(input.as_slice(), Vec::new());
    gz.gzip().unwrap();
    let (_, compressed) = gz.into_inner();
    assert!(compressed.len() < input.len());
    let mut ungz = Gzip::new(compressed.as_slice(), Vec::new());
    ungz.ungzip().unwrap();
    let (_, decompressed) = ungz.into_inner();
    assert_eq!(decompressed, input);
}

#[test]
fn zip_writer_add_bytes() {
    let mut writer = ZipWriter::new(Cursor::new(Vec::new()));
    writer.add_bytes("test.txt", b"Hello, World!").unwrap();
    writer.add_bytes("data.bin", &[0, 1, 2, 3]).unwrap();
    let cursor = writer.finish().unwrap();
    assert!(!cursor.into_inner().is_empty());
}

#[test]
fn zip_limits_default() {
    let limits = ZipLimits::default();
    assert_eq!(limits.max_entries, 10_000);
    assert!(limits.max_uncompressed_bytes > 0);
}
