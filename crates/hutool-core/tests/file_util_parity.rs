//! file_util parity tests
//! 对齐: hutool-core FileUtilTest

use hutool_core::FileUtil;
use std::path::Path;

// ── 路径操作 ──

#[test]
fn name_basic() {
    assert_eq!(FileUtil::name(Path::new("/path/to/file.txt")), "file.txt");
    assert_eq!(FileUtil::name(Path::new("file.txt")), "file.txt");
}

#[test]
fn suffix_basic() {
    assert_eq!(FileUtil::suffix(Path::new("/path/to/file.txt")), "txt");
    assert_eq!(FileUtil::suffix(Path::new("file.rs")), "rs");
}

#[test]
fn main_name_basic() {
    assert_eq!(FileUtil::main_name(Path::new("/path/to/file.txt")), "file");
    assert_eq!(
        FileUtil::main_name(Path::new("archive.tar.gz")),
        "archive.tar"
    );
}

#[test]
fn name_from_str_basic() {
    assert_eq!(FileUtil::name_from_str("/path/to/file.txt"), "file.txt");
}

#[test]
fn suffix_from_str_basic() {
    assert_eq!(FileUtil::suffix_from_str("/path/to/file.txt"), "txt");
}

// ── 路径构建 ──

#[test]
fn join_paths_basic() {
    let path = FileUtil::join_paths(&["/home", "user", "file.txt"]);
    assert_eq!(path, Path::new("/home/user/file.txt"));
}

#[test]
fn file_basic() {
    let path = FileUtil::file(&["/tmp", "test.txt"]);
    assert_eq!(path, Path::new("/tmp/test.txt"));
}

// ── 文件判断 ──

#[test]
fn exists_tmp_dir() {
    assert!(FileUtil::exists("/tmp"));
}

#[test]
fn exists_nonexistent() {
    assert!(!FileUtil::exists("/nonexistent/path/that/does/not/exist"));
}

#[test]
fn is_directory_tmp() {
    assert!(FileUtil::is_directory("/tmp"));
}

#[test]
fn is_file_tmp() {
    assert!(!FileUtil::is_file("/tmp"));
}

// ── 文件读写 ──

#[test]
fn read_write_utf8_string() {
    let path = "/tmp/test_file_util_utf8.txt";
    let content = "Hello, World!\n你好世界";
    FileUtil::write_utf8_string(path, content).unwrap();
    let read = FileUtil::read_utf8_string(path).unwrap();
    assert_eq!(read, content);
    FileUtil::delete(path).unwrap();
}

#[test]
fn read_write_bytes() {
    let path = "/tmp/test_file_util_bytes.bin";
    let content = b"Hello, Binary!";
    FileUtil::write_bytes(path, content).unwrap();
    let read = FileUtil::read_bytes(path).unwrap();
    assert_eq!(read, content);
    FileUtil::delete(path).unwrap();
}

// ── 文件操作 ──

#[test]
fn copy_basic() {
    let from = "/tmp/test_file_util_copy_src.txt";
    let to = "/tmp/test_file_util_copy_dst.txt";
    FileUtil::write_utf8_string(from, "copy test").unwrap();
    FileUtil::copy(from, to).unwrap();
    let read = FileUtil::read_utf8_string(to).unwrap();
    assert_eq!(read, "copy test");
    FileUtil::delete(from).unwrap();
    FileUtil::delete(to).unwrap();
}

#[test]
fn mkdir_and_delete() {
    let path = "/tmp/test_file_util_dir";
    FileUtil::mkdir(path).unwrap();
    assert!(FileUtil::is_directory(path));
    FileUtil::delete(path).unwrap();
    assert!(!FileUtil::exists(path));
}

#[test]
fn rename_basic() {
    let from = "/tmp/test_file_util_rename_from.txt";
    let to = "/tmp/test_file_util_rename_to.txt";
    FileUtil::write_utf8_string(from, "rename test").unwrap();
    FileUtil::rename(from, to).unwrap();
    assert!(!FileUtil::exists(from));
    assert!(FileUtil::exists(to));
    FileUtil::delete(to).unwrap();
}

// ── 文件列表 ──

#[test]
fn list_file_names_basic() {
    let dir = "/tmp/test_file_util_list";
    FileUtil::mkdir(dir).unwrap();
    FileUtil::write_utf8_string(&format!("{}/a.txt", dir), "a").unwrap();
    FileUtil::write_utf8_string(&format!("{}/b.txt", dir), "b").unwrap();
    let names = FileUtil::list_file_names(dir).unwrap();
    assert_eq!(names.len(), 2);
    assert!(names.contains(&"a.txt".to_string()));
    assert!(names.contains(&"b.txt".to_string()));
    FileUtil::delete(dir).unwrap();
}

// ── 文件大小 ──

#[test]
fn size_basic() {
    let path = "/tmp/test_file_util_size.txt";
    FileUtil::write_utf8_string(path, "Hello").unwrap();
    let size = FileUtil::size(Path::new(path));
    assert_eq!(size, 5);
    FileUtil::delete(path).unwrap();
}

// ── 临时目录 ──

#[test]
fn tmp_dir_basic() {
    let tmp = FileUtil::tmp_dir();
    assert!(tmp.is_absolute() || tmp.exists() || !tmp.as_os_str().is_empty());
    assert_eq!(FileUtil::tmp_dir_str(), tmp.to_string_lossy());
}

/// 对齐 Java: `FileUtilTest.getNameTest()`
#[test]
fn get_name_test() {
    assert_eq!(FileUtil::name(Path::new("/aaa/bbb/cc/ddd/")), "");
    assert_eq!(FileUtil::name_from_str("/aaa/bbb/cc/ddd.jpg"), "ddd.jpg");
    assert_eq!(FileUtil::name(Path::new("/aaa/bbb/cc/ddd.jpg")), "ddd.jpg");
}

/// 对齐 Java: `FileUtilTest.mainNameTest()`
#[test]
fn main_name_test() {
    assert_eq!(FileUtil::main_name(Path::new("/aaa/bbb/cc/ddd")), "ddd");
    assert_eq!(FileUtil::main_name(Path::new("/aaa/bbb/cc/ddd.jpg")), "ddd");
}

/// 对齐 Java: `FileUtilTest.extNameTest()`（Rust Path 取最后一段扩展名）
#[test]
fn ext_name_test() {
    assert_eq!(FileUtil::suffix(Path::new("/aaa/bbb/cc/ddd")), "");
    assert_eq!(FileUtil::suffix(Path::new("/aaa/bbb/cc/ddd.jpg")), "jpg");
    assert_eq!(FileUtil::suffix(Path::new("/aaa/bbb/cc/fff.xlsx")), "xlsx");
    // Java extName("fff.tar.gz") == "tar.gz"；Rust Path::extension → "gz"
    assert_eq!(FileUtil::suffix(Path::new("/aaa/bbb/cc/fff.tar.gz")), "gz");
}

/// 对齐 Java: `FileUtilTest.listFileNamesTest()`
#[test]
fn list_file_names_test() {
    let dir = "/tmp/test_file_util_list_names";
    let _ = FileUtil::delete(dir);
    FileUtil::mkdir(dir).unwrap();
    FileUtil::write_utf8_string(&format!("{dir}/a.txt"), "a").unwrap();
    FileUtil::write_utf8_string(&format!("{dir}/b.txt"), "b").unwrap();
    FileUtil::mkdir(&format!("{dir}/subdir")).unwrap();
    let names = FileUtil::list_file_names(dir).unwrap();
    assert_eq!(names.len(), 2);
    assert!(names.contains(&"a.txt".to_string()));
    assert!(names.contains(&"b.txt".to_string()));
    FileUtil::delete(dir).unwrap();
}

/// 对齐 Java: `FileUtil.listFileNames` 目录名列表（Rust 扩展 list_dir_names）
#[test]
fn list_dir_names_test() {
    let dir = "/tmp/test_file_util_list_dirs";
    let _ = FileUtil::delete(dir);
    FileUtil::mkdir(dir).unwrap();
    FileUtil::mkdir(&format!("{dir}/d1")).unwrap();
    FileUtil::mkdir(&format!("{dir}/d2")).unwrap();
    FileUtil::write_utf8_string(&format!("{dir}/f.txt"), "f").unwrap();
    let names = FileUtil::list_dir_names(dir).unwrap();
    assert_eq!(names.len(), 2);
    assert!(names.contains(&"d1".to_string()));
    assert!(names.contains(&"d2".to_string()));
    FileUtil::delete(dir).unwrap();
}

/// 对齐 Java: `FileUtilTest.touchTest()` / `FileUtilTest.delTest()`
#[test]
fn touch_and_del_test() {
    let path = "/tmp/test_file_util_touch.txt";
    let _ = FileUtil::delete(path);
    FileUtil::write_utf8_string(path, "").unwrap();
    assert!(FileUtil::is_file(path));
    FileUtil::delete(path).unwrap();
    assert!(!FileUtil::exists(path));
}

/// 对齐 Java: `FileUtilTest.copyTest()` / `PathUtilTest.copyFileTest()` 语义
#[test]
fn copy_overwrite_test() {
    let from = "/tmp/test_file_util_copy_ow_src.txt";
    let to = "/tmp/test_file_util_copy_ow_dst.txt";
    FileUtil::write_utf8_string(from, "v1").unwrap();
    FileUtil::write_utf8_string(to, "old").unwrap();
    FileUtil::copy(from, to).unwrap();
    assert_eq!(FileUtil::read_utf8_string(to).unwrap(), "v1");
    FileUtil::delete(from).unwrap();
    FileUtil::delete(to).unwrap();
}

/// 对齐 Java: `FileUtilTest.renameTest()` / `PathUtilTest.moveTest()` 语义
#[test]
fn rename_to_new_name_test() {
    let from = "/tmp/test_file_util_rename2_from.txt";
    let to = "/tmp/test_file_util_rename2_to.txt";
    let _ = FileUtil::delete(from);
    let _ = FileUtil::delete(to);
    FileUtil::write_utf8_string(from, "move-me").unwrap();
    FileUtil::rename(from, to).unwrap();
    assert!(!FileUtil::exists(from));
    assert_eq!(FileUtil::read_utf8_string(to).unwrap(), "move-me");
    FileUtil::delete(to).unwrap();
}

/// 对齐 Java: `FileUtilTest.fileTest1()` / `fileTest2()`
#[test]
fn file_path_parts_test() {
    let path = FileUtil::file(&["/tmp", "hutool", "a.txt"]);
    assert_eq!(path, Path::new("/tmp/hutool/a.txt"));
    assert_eq!(FileUtil::name(&path), "a.txt");
    assert_eq!(FileUtil::suffix(&path), "txt");
}

/// 对齐 Java: `PathUtilTest.delDirTest()` 语义
#[test]
fn del_dir_test() {
    let dir = "/tmp/test_file_util_del_dir";
    let nested = format!("{dir}/nested");
    let _ = FileUtil::delete(dir);
    FileUtil::mkdir(&nested).unwrap();
    FileUtil::write_utf8_string(&format!("{nested}/x.txt"), "x").unwrap();
    FileUtil::delete(dir).unwrap();
    assert!(!FileUtil::exists(dir));
}

/// 对齐 Java: `FileUtil.size` 空文件
#[test]
fn size_empty_file_test() {
    let path = "/tmp/test_file_util_size_empty.txt";
    FileUtil::write_utf8_string(path, "").unwrap();
    assert_eq!(FileUtil::size(Path::new(path)), 0);
    FileUtil::delete(path).unwrap();
}

/// 对齐 Java: `FileUtil.touch` / `extName` / `ls` / `contentEquals`
#[test]
fn touch_ext_ls_content_equals_test() {
    let dir = "/tmp/test_file_util_touch_ls";
    let a = format!("{dir}/a.txt");
    let b = format!("{dir}/b.txt");
    let _ = FileUtil::delete(dir);
    FileUtil::touch(&a).unwrap();
    FileUtil::write_utf8_string(&a, "hello").unwrap();
    FileUtil::write_utf8_string(&b, "hello").unwrap();
    assert_eq!(FileUtil::ext_name(&a), "txt");
    assert!(FileUtil::content_equals(&a, &b).unwrap());
    let names = FileUtil::ls(dir).unwrap();
    assert!(names.contains(&"a.txt".to_string()));
    FileUtil::clean(dir).unwrap();
    assert!(FileUtil::ls(dir).unwrap().is_empty());
    FileUtil::delete(dir).unwrap();
}

/// 对齐 Java: `FileUtil.isWindows` / `getAbsolutePath`
#[test]
fn is_windows_and_absolute_path_test() {
    assert_eq!(FileUtil::is_windows(), cfg!(windows));
    let abs = FileUtil::absolute_path("relative.txt");
    assert!(abs.is_absolute());
}

/// 对齐 Java: FileUtil leftovers — normalize / lines / checksum / mime / home
#[test]
fn file_util_leftover_helpers() {
    let dir = "/tmp/test_file_util_leftovers";
    let path = format!("{dir}/a.txt");
    let _ = FileUtil::delete(dir);
    FileUtil::mkdir(dir).unwrap();
    FileUtil::write_utf8_lines(&path, &["one", "two"]).unwrap();
    assert_eq!(
        FileUtil::read_utf8_lines(&path).unwrap(),
        vec!["one", "two"]
    );
    assert_eq!(FileUtil::total_lines(&path).unwrap(), 2);
    FileUtil::append_utf8_string(&path, "\nthree").unwrap();
    assert!(FileUtil::is_not_empty(&path));
    assert!(!FileUtil::is_empty(&path));
    assert_eq!(FileUtil::normalize("/a/./b/../c"), "/a/c");
    assert!(FileUtil::is_absolute_path("/tmp"));
    assert!(!FileUtil::user_home_path().as_os_str().is_empty());
    assert_eq!(FileUtil::mime_type("x.png"), "image/png");
    assert!(!FileUtil::checksum_sha256(&path).unwrap().is_empty());
    assert_eq!(FileUtil::readable_file_size(1024), "1.0 KB");
    assert!(FileUtil::path_ends_with(&path, "a.txt"));
    assert!(FileUtil::last_index_of_separator(&path).is_some());
    let moved = format!("{dir}/b.txt");
    FileUtil::move_path(&path, &moved).unwrap();
    assert!(FileUtil::exists(&moved));
    FileUtil::delete(dir).unwrap();
}
