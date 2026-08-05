//! `ZipUtil` 对比验证测试 —— 对齐 Hutool `ZipUtilTest`（15 个 @Test）
//!
//! 对齐: `cn.hutool.core.util.ZipUtilTest`
//! 来源: hutool-core/src/test/java/cn/hutool/core/util/ZipUtilTest.java

use hutool_core::ZipUtil;
use std::fs;
use std::path::Path;
use tempfile::tempdir;

fn zip_entry_names(zip_file: &Path) -> Vec<String> {
    ZipUtil::list_entry_names(zip_file).unwrap()
}

fn create_seed_zip(zip_file: &Path, seed_file: &Path) {
    fs::write(seed_file, b"seed").unwrap();
    ZipUtil::zip_files(zip_file, false, &[seed_file]).unwrap();
}

/// 对齐 Java: `ZipUtilTest.gzipTest()`
#[test]
fn gzip_test() {
    let data = "我是一个需要压缩的很长很长的字符串";
    let gzip = ZipUtil::gzip(data.as_bytes()).unwrap();
    assert!(!gzip.is_empty(), "gzip 压缩结果非空 (对齐 Java)");
    let decoded = ZipUtil::un_gzip(&gzip).unwrap();
    assert_eq!(
        data.as_bytes(),
        decoded.as_slice(),
        "gzip 解压后应等于原文 (对齐 Java)"
    );
}

/// 对齐 Java: `ZipUtilTest.zlibTest()`
#[test]
fn zlib_test() {
    let data = "我是一个需要压缩的很长很长的字符串";
    let bytes = data.as_bytes();
    let zlib0 = ZipUtil::zlib(bytes, 0).unwrap();
    assert!(
        zlib0.len() < bytes.len() + 20,
        "zlib level 0 合理大小 (对齐 Java)"
    );
    let zlib9 = ZipUtil::zlib(bytes, 9).unwrap();
    assert!(
        zlib9.len() < zlib0.len(),
        "zlib level 9 应比 level 0 更小 (对齐 Java)"
    );
    assert_eq!(bytes, ZipUtil::un_zlib(&zlib0).unwrap().as_slice());
    assert_eq!(bytes, ZipUtil::un_zlib(&zlib9).unwrap().as_slice());
}

/// 对齐 Java: `ZipUtilTest.appendTest()`
#[test]
fn append_test() {
    let dir = tempdir().unwrap();
    let zip_file = dir.path().join("test.zip");
    let seed_file = dir.path().join("seed.txt");
    create_seed_zip(&zip_file, &seed_file);
    let append_file = dir.path().join("addFile.txt");
    fs::write(&append_file, b"append me").unwrap();

    let before = zip_entry_names(&zip_file);
    ZipUtil::append(&zip_file, &append_file).unwrap();
    let after = zip_entry_names(&zip_file);
    assert_eq!(before.len() + 1, after.len());
    assert!(before.iter().all(|name| after.contains(name)));
    assert!(after.iter().any(|name| name == "addFile.txt"));

    let before = zip_entry_names(&zip_file);
    let add_dir = dir.path().join("test-add");
    fs::create_dir(&add_dir).unwrap();
    fs::write(add_dir.join("test.txt"), b"dir file").unwrap();
    ZipUtil::append(&zip_file, &add_dir).unwrap();
    let after = zip_entry_names(&zip_file);
    assert_eq!(before.len() + 2, after.len());
    assert!(before.iter().all(|name| after.contains(name)));
    assert!(after.iter().any(|name| name == "addFile.txt"));
}

/// 对齐 Java: `ZipUtilTest.zipDirTest()`
#[test]
fn zip_dir_test() {
    let dir = tempdir().unwrap();
    let src = dir.path().join("src");
    fs::create_dir(&src).unwrap();
    fs::write(src.join("a.txt"), b"a").unwrap();
    let zip_file = dir.path().join("dir.zip");
    ZipUtil::zip_dir(&src, &zip_file, false).unwrap();
    assert!(!zip_entry_names(&zip_file).is_empty());
}

/// 对齐 Java: `ZipUtilTest.unzipTest()`
#[test]
fn unzip_test() {
    let dir = tempdir().unwrap();
    let zip_file = dir.path().join("roundtrip.zip");
    let seed_file = dir.path().join("seed.txt");
    create_seed_zip(&zip_file, &seed_file);
    let out = dir.path().join("out");
    ZipUtil::unzip(&zip_file, &out).unwrap();
    assert!(out.join("seed.txt").is_file());
}

/// 对齐 Java: `ZipUtilTest.unzipTest2()`
#[test]
fn unzip_test_2() {
    let dir = tempdir().unwrap();
    let zip_file = dir.path().join("multi.zip");
    let a = dir.path().join("a.txt");
    let b = dir.path().join("b.txt");
    fs::write(&a, b"a").unwrap();
    fs::write(&b, b"b").unwrap();
    ZipUtil::zip_files(&zip_file, false, &[&a, &b]).unwrap();
    let out = dir.path().join("out2");
    ZipUtil::unzip(&zip_file, &out).unwrap();
    assert_eq!("a", fs::read_to_string(out.join("a.txt")).unwrap());
    assert_eq!("b", fs::read_to_string(out.join("b.txt")).unwrap());
}

/// 对齐 Java: `ZipUtilTest.unzipFromStreamTest()`
#[test]
fn unzip_from_stream_test() {
    let dir = tempdir().unwrap();
    let zip_file = dir.path().join("stream.zip");
    let seed_file = dir.path().join("seed.txt");
    create_seed_zip(&zip_file, &seed_file);
    let bytes = fs::read(&zip_file).unwrap();
    let out = dir.path().join("stream-out");
    ZipUtil::unzip_bytes(&bytes, &out).unwrap();
    assert!(out.join("seed.txt").is_file());
}

/// 对齐 Java: `ZipUtilTest.unzipChineseTest()`
#[test]
fn unzip_chinese_test() {
    let dir = tempdir().unwrap();
    let zip_file = dir.path().join("中文.zip");
    let data = ZipUtil::zip_streams(&["中文.txt"], &["你好".as_bytes()]).unwrap();
    fs::write(&zip_file, data).unwrap();
    let out = dir.path().join("中文-out");
    ZipUtil::unzip(&zip_file, &out).unwrap();
    assert_eq!("你好", fs::read_to_string(out.join("中文.txt")).unwrap());
}

/// 对齐 Java: `ZipUtilTest.unzipFileBytesTest()`
#[test]
fn unzip_file_bytes_test() {
    let dir = tempdir().unwrap();
    let zip_file = dir.path().join("entry.zip");
    let data = ZipUtil::zip_streams(&["images/picture.jpg"], &[b"fake-image"]).unwrap();
    fs::write(&zip_file, data).unwrap();
    let bytes = ZipUtil::unzip_file_bytes(&zip_file, "images/picture.jpg")
        .unwrap()
        .unwrap();
    assert_eq!(b"fake-image", bytes.as_slice());
}

/// 对齐 Java: `ZipUtilTest.zipStreamTest()`
#[test]
fn zip_stream_test() {
    let dir = tempdir().unwrap();
    let src = dir.path().join("src-stream");
    fs::create_dir(&src).unwrap();
    fs::write(src.join("a.txt"), b"a").unwrap();
    let zip_file = dir.path().join("stream.zip");
    ZipUtil::zip_dir(&src, &zip_file, false).unwrap();
    assert!(!zip_entry_names(&zip_file).is_empty());
}

/// 对齐 Java: `ZipUtilTest.zipStreamTest2()`
#[test]
fn zip_stream_test_2() {
    let dir = tempdir().unwrap();
    let file1 = dir.path().join("a.txt");
    let file2 = dir.path().join("b.txt");
    fs::write(&file1, b"1").unwrap();
    fs::write(&file2, b"2").unwrap();
    let zip_file = dir.path().join("multi-stream.zip");
    ZipUtil::zip_files(&zip_file, false, &[&file1, &file2]).unwrap();
    assert_eq!(2, zip_entry_names(&zip_file).len());
}

/// 对齐 Java: `ZipUtilTest.zipToStreamTest()`
#[test]
fn zip_to_stream_test() {
    let bytes = ZipUtil::zip_streams(&["sm1_alias.txt"], &[b"stream-content"]).unwrap();
    assert!(!bytes.is_empty());
}

/// 对齐 Java: `ZipUtilTest.zipMultiFileTest()`
#[test]
fn zip_multi_file_test() {
    let dir = tempdir().unwrap();
    let a = dir.path().join("qr_a.jpg");
    let b = dir.path().join("qr_b.jpg");
    fs::write(&a, b"a").unwrap();
    fs::write(&b, b"b").unwrap();
    let zip_file = dir.path().join("qr.zip");
    ZipUtil::zip_files(&zip_file, false, &[&a, &b]).unwrap();
    assert_eq!(2, zip_entry_names(&zip_file).len());
}

/// 对齐 Java: `ZipUtilTest.sizeUnzipTest()`
#[test]
fn size_unzip_test() {
    let dir = tempdir().unwrap();
    let zip_file = dir.path().join("large.zip");
    let payload = vec![b'x'; 1024 * 700];
    let data = ZipUtil::zip_streams(&["large.bin"], &[&payload]).unwrap();
    fs::write(&zip_file, data).unwrap();
    let out = dir.path().join("limited-out");
    assert!(ZipUtil::unzip_with_limit(&zip_file, &out, 637 * 1024).is_err());
}

/// 对齐 Java: `ZipUtilTest.issue3018Test()`
#[test]
fn issue_3018_test() {
    let dir = tempdir().unwrap();
    let bytes = ZipUtil::zip_streams(&["default.txt"], &[b"default"]).unwrap();
    let out = dir.path().join("default-out");
    ZipUtil::unzip_bytes(&bytes, &out).unwrap();
    assert_eq!(
        b"default",
        fs::read(out.join("default.txt")).unwrap().as_slice()
    );
}

/// 对齐 Java: ZipUtil gzip_str / listFileNames / get / read
#[test]
fn zip_util_extra_helpers() {
    let dir = tempdir().unwrap();
    let gz = ZipUtil::gzip_str("你好").unwrap();
    assert_eq!(ZipUtil::un_gzip_str(&gz).unwrap(), "你好");
    let zl = ZipUtil::zlib_str("zlib", 6).unwrap();
    assert_eq!(ZipUtil::un_zlib_str(&zl).unwrap(), "zlib");
    let zip_file = dir.path().join("extra.zip");
    ZipUtil::zip_bytes_entry(&zip_file, "dir/a.txt", b"a").unwrap();
    let append_file = dir.path().join("b.txt");
    fs::write(&append_file, b"b").unwrap();
    ZipUtil::append(&zip_file, &append_file).unwrap();
    let names = ZipUtil::list_file_names(&zip_file, "dir").unwrap();
    assert!(names.iter().any(|n| n.ends_with("a.txt")));
    assert_eq!(
        ZipUtil::get(&zip_file, "dir/a.txt").unwrap().as_deref(),
        Some(b"a".as_slice())
    );
    let mut seen = Vec::new();
    ZipUtil::read_names(&zip_file, |n| seen.push(n.to_string())).unwrap();
    assert!(!seen.is_empty());
}
