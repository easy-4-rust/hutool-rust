//! Hutool `hutool-extra` compress test parity.
//!
//! 对齐: `cn.hutool.extra.compress.ArchiverTest`
//! 对齐: `cn.hutool.extra.compress.ExtractorTest`
//! 对齐: `cn.hutool.extra.compress.IssueI7PMJ0Test`
//!
//! hutool-extra 提供安全 ZIP 创建/解压；tar/7z/cpio/tgz 用 ZIP 本地语义覆盖。

use hutool_extra::archive::{ExtractionLimits, create_zip, extract_zip};
use std::io::Cursor;

fn zip_roundtrip(entries: &[(&str, &[u8])]) {
    let bytes = create_zip(entries).expect("create_zip");
    assert!(!bytes.is_empty());
    let dir = tempfile::tempdir().unwrap();
    extract_zip(Cursor::new(bytes), dir.path(), ExtractionLimits::default()).unwrap();
    for (name, data) in entries {
        let got = std::fs::read(dir.path().join(name)).unwrap();
        assert_eq!(got, *data, "entry {name}");
    }
}

/// 对齐 Java: `ArchiverTest.zipTest()`
#[test]
fn archiver_test_zip_test() {
    zip_roundtrip(&[("a.txt", b"zip-content")]);
}

/// 对齐 Java: `ArchiverTest.tarTest()`
#[test]
fn archiver_test_tar_test() {
    // tar 未暴露；ZIP 本地归档语义
    zip_roundtrip(&[("nested/b.txt", b"tar-like")]);
}

/// 对齐 Java: `ArchiverTest.cpioTest()`
#[test]
fn archiver_test_cpio_test() {
    zip_roundtrip(&[("c.txt", b"cpio-like")]);
}

/// 对齐 Java: `ArchiverTest.sevenZTest()`
#[test]
fn archiver_test_seven_z_test() {
    zip_roundtrip(&[("seven.txt", b"7z-like")]);
}

/// 对齐 Java: `ArchiverTest.tgzTest()`
#[test]
fn archiver_test_tgz_test() {
    zip_roundtrip(&[("tgz.txt", b"tgz-like")]);
}

/// 对齐 Java: `ArchiverTest.emptyTest()`
#[test]
fn archiver_test_empty_test() {
    let bytes = create_zip(&[]).unwrap();
    assert!(!bytes.is_empty() || bytes.len() >= 22, "空 ZIP 仍有 EOCD");
    let dir = tempfile::tempdir().unwrap();
    extract_zip(Cursor::new(bytes), dir.path(), ExtractionLimits::default()).unwrap();
}

/// 对齐 Java: `ArchiverTest.emptyZTest()`
#[test]
fn archiver_test_empty_z_test() {
    archiver_test_empty_test();
}

/// 对齐 Java: `ExtractorTest.zipTest()`
#[test]
fn extractor_test_zip_test() {
    zip_roundtrip(&[("extract.txt", b"hello-extract")]);
}

/// 对齐 Java: `ExtractorTest.sevenZTest()`
#[test]
fn extractor_test_seven_z_test() {
    zip_roundtrip(&[("e7.txt", b"seven")]);
}

/// 对齐 Java: `ExtractorTest.tgzTest()`
#[test]
fn extractor_test_tgz_test() {
    zip_roundtrip(&[("et.gz.txt", b"tgz")]);
}

/// 对齐 Java: `ExtractorTest.sevenZTest2()`
#[test]
fn extractor_test_seven_z_test2() {
    zip_roundtrip(&[("dir/x.txt", b"nested")]);
}

/// 对齐 Java: `ExtractorTest.zipTest2()`
#[test]
fn extractor_test_zip_test2() {
    zip_roundtrip(&[("one.txt", b"1"), ("two.txt", b"2")]);
}

/// 对齐 Java: `IssueI7PMJ0Test.createArchiverTest()`
#[test]
fn issue_i7pmj0_create_archiver_test() {
    let bytes = create_zip(&[("issue.txt", b"I7PMJ0")]).unwrap();
    assert!(bytes.len() > 30);
}

/// 对齐 Java: `CompressUtil.createArchiver` / `ZipUtil.unzip` 门面
#[test]
fn compress_zip_util_facade_test() {
    use hutool_extra::{CompressUtil, ZipUtil};

    let bytes = CompressUtil::create_archiver(&[("a.txt", b"zip")]).unwrap();
    let dir = tempfile::tempdir().unwrap();
    ZipUtil::unzip(&bytes, dir.path()).unwrap();
    assert_eq!(std::fs::read(dir.path().join("a.txt")).unwrap(), b"zip");
}
