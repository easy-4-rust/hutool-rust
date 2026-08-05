//! `compiler` 子包真实功能测试
//! 对齐: hutool-core/compiler JavaCompilerUtil/FileObject/ClassFileManager
//! 基于 hutool-core compiler/mod.rs 真实实现

use hutool_core::{
    ClassFileManager, ClassFileObject, CompilerException, DEFAULT_MAX_SOURCE_BYTES,
    SourceFileObject, SourceFileObjectUtil, diagnostic_messages,
};
use std::path::Path;

// ── CompilerException ──

#[test]
fn compiler_exception_new() {
    let e = CompilerException::new("something went wrong");
    assert!(e.to_string().contains("something went wrong"));
}

#[test]
fn compiler_exception_formatted() {
    let e = CompilerException::formatted("error at {}:{}", &[&"line", &"42"]);
    assert!(e.to_string().contains("line"));
    assert!(e.to_string().contains("42"));
}

// ── SourceFileObject ──

#[test]
fn source_file_object_new() {
    let sfo = SourceFileObject::new("test.rs", "fn main() {}", DEFAULT_MAX_SOURCE_BYTES);
    assert!(sfo.is_ok());
    let sfo = sfo.unwrap();
    assert_eq!(sfo.name(), Path::new("test.rs"));
    assert_eq!(sfo.char_content(), "fn main() {}");
}

#[test]
fn source_file_object_empty_source() {
    let sfo = SourceFileObject::new("empty.rs", "", DEFAULT_MAX_SOURCE_BYTES);
    assert!(sfo.is_ok());
    assert_eq!(sfo.unwrap().char_content(), "");
}

#[test]
fn source_file_object_exceeds_max_bytes() {
    let result = SourceFileObject::new("big.rs", "a".repeat(20).as_str(), 10);
    assert!(result.is_err());
}

// ── ClassFileObject ──

#[test]
fn class_file_object_new() {
    let cfo = ClassFileObject::new("com.example.Foo");
    assert_eq!(cfo.name(), "com.example.Foo");
    assert!(cfo.open_input().is_empty());
}

#[test]
fn class_file_object_write_and_read() {
    let mut cfo = ClassFileObject::new("output");
    cfo.open_output()
        .extend_from_slice(&[0xCA, 0xFE, 0xBA, 0xBE]);
    assert_eq!(cfo.open_input(), &[0xCA, 0xFE, 0xBA, 0xBE]);
}

// ── ClassFileManager ──

#[test]
fn class_file_manager_output_and_get() {
    let mut cfm = ClassFileManager::default();
    cfm.output("my.Class")
        .open_output()
        .extend_from_slice(&[1, 2, 3]);
    let cfo = cfm.get("my.Class");
    assert!(cfo.is_some());
    assert_eq!(cfo.unwrap().open_input(), &[1, 2, 3]);
}

#[test]
fn class_file_manager_get_missing() {
    let cfm = ClassFileManager::default();
    assert!(cfm.get("nonexistent").is_none());
}

#[test]
fn class_file_manager_multiple_classes() {
    let mut cfm = ClassFileManager::default();
    cfm.output("A").open_output().push(10);
    cfm.output("B").open_output().push(20);
    assert_eq!(cfm.get("A").unwrap().open_input(), &[10]);
    assert_eq!(cfm.get("B").unwrap().open_input(), &[20]);
}

// ── SourceFileObjectUtil ──

#[test]
fn source_file_util_is_source_file() {
    // hutool-rs 只识别 .rs 文件(对齐 Rust 编译器)
    assert!(SourceFileObjectUtil::is_source_file("lib.rs"));
    assert!(SourceFileObjectUtil::is_source_file("main.rs"));
    assert!(!SourceFileObjectUtil::is_source_file("data.bin"));
    assert!(!SourceFileObjectUtil::is_source_file("Main.java"));
}

#[test]
fn source_file_util_is_library_file() {
    assert!(SourceFileObjectUtil::is_library_file("libcore.rlib"));
    assert!(SourceFileObjectUtil::is_library_file("libfoo.so"));
    assert!(SourceFileObjectUtil::is_library_file("bar.dylib"));
    assert!(!SourceFileObjectUtil::is_library_file("main.rs"));
}

// ── diagnostic_messages ──

#[test]
fn diagnostic_messages_format() {
    let result = diagnostic_messages(&["error in line 10", "missing semicolon"]);
    assert!(result.contains("error in line 10"));
    assert!(result.contains("missing semicolon"));
}

#[test]
fn diagnostic_messages_empty() {
    let result: String = diagnostic_messages(&[] as &[&str]);
    assert!(result.is_empty() || result.trim().is_empty());
}

// ── DEFAULT_MAX_SOURCE_BYTES ──

#[test]
fn default_max_source_bytes_is_reasonable() {
    assert!(DEFAULT_MAX_SOURCE_BYTES > 0);
    assert!(
        DEFAULT_MAX_SOURCE_BYTES >= 64 * 1024,
        "should be at least 64KB"
    );
}

// ── 扩展 compiler 测试 ──

#[test]
fn compiler_exception_from_error() {
    let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
    let e = CompilerException::from_error(io_err);
    assert!(e.to_string().contains("file not found"));
}

#[test]
fn compiler_exception_with_source() {
    let io_err = std::io::Error::new(std::io::ErrorKind::PermissionDenied, "access denied");
    let e = CompilerException::with_source("compilation failed", io_err);
    assert!(e.to_string().contains("compilation failed"));
}

#[test]
fn compiler_exception_formatted_params() {
    let e = CompilerException::formatted("error at line {}, col {}", &[&42, &10]);
    let msg = e.to_string();
    assert!(msg.contains("42"));
    assert!(msg.contains("10"));
}

#[test]
fn source_file_object_from_path() {
    let temp = std::env::temp_dir().join("test_source.rs");
    std::fs::write(&temp, "fn main() {}").unwrap();
    let sfo = SourceFileObject::from_path(&temp, DEFAULT_MAX_SOURCE_BYTES);
    assert!(sfo.is_ok());
    let sfo = sfo.unwrap();
    assert_eq!(sfo.char_content(), "fn main() {}");
    std::fs::remove_file(&temp).unwrap();
}

#[test]
fn source_file_object_large_content() {
    let content = "fn main() {\n    println!(\"Hello\");\n}\n".repeat(100);
    let sfo = SourceFileObject::new("large.rs", &content, DEFAULT_MAX_SOURCE_BYTES);
    assert!(sfo.is_ok());
    assert_eq!(sfo.unwrap().char_content(), content);
}

#[test]
fn class_file_object_write_multiple() {
    let mut cfo = ClassFileObject::new("multi");
    cfo.open_output().push(1);
    cfo.open_output().push(2);
    cfo.open_output().push(3);
    assert_eq!(cfo.open_input(), &[1, 2, 3]);
}

#[test]
fn class_file_manager_overwrite() {
    let mut cfm = ClassFileManager::default();
    cfm.output("my.Class").open_output().push(1);
    cfm.output("my.Class").open_output().push(2);
    // Both pushes go to same object
    let cfo = cfm.get("my.Class").unwrap();
    assert_eq!(cfo.open_input(), &[1, 2]);
}

#[test]
fn class_file_manager_many_classes() {
    let mut cfm = ClassFileManager::default();
    for i in 0..10 {
        cfm.output(&format!("Class{}", i)).open_output().push(i);
    }
    assert_eq!(cfm.get("Class0").unwrap().open_input(), &[0]);
    assert_eq!(cfm.get("Class9").unwrap().open_input(), &[9]);
}

#[test]
fn source_file_util_is_source_file_various() {
    assert!(SourceFileObjectUtil::is_source_file("main.rs"));
    assert!(SourceFileObjectUtil::is_source_file("lib.rs"));
    assert!(SourceFileObjectUtil::is_source_file("mod.rs"));
    assert!(!SourceFileObjectUtil::is_source_file("data.txt"));
    assert!(!SourceFileObjectUtil::is_source_file("image.png"));
}

#[test]
fn source_file_util_is_library_file_various() {
    assert!(SourceFileObjectUtil::is_library_file("libcore.rlib"));
    assert!(SourceFileObjectUtil::is_library_file("libfoo.a"));
    assert!(SourceFileObjectUtil::is_library_file("libbar.so"));
    assert!(SourceFileObjectUtil::is_library_file("baz.dylib"));
    assert!(SourceFileObjectUtil::is_library_file("qux.dll"));
    assert!(!SourceFileObjectUtil::is_library_file("main.rs"));
    assert!(!SourceFileObjectUtil::is_library_file("data.txt"));
}

#[test]
fn diagnostic_messages_single() {
    let result = diagnostic_messages(&["error: missing semicolon"]);
    assert!(result.contains("missing semicolon"));
}

#[test]
fn diagnostic_messages_multiple() {
    let result = diagnostic_messages(&["error at line 1", "error at line 5", "warning: unused"]);
    assert!(result.contains("line 1"));
    assert!(result.contains("line 5"));
    assert!(result.contains("unused"));
}

#[test]
fn default_max_source_bytes_value() {
    assert!(DEFAULT_MAX_SOURCE_BYTES >= 1024);
}

#[test]
fn source_file_object_unicode() {
    let sfo = SourceFileObject::new(
        "unicode.rs",
        "fn main() { println!(\"你好\"); }",
        DEFAULT_MAX_SOURCE_BYTES,
    );
    assert!(sfo.is_ok());
    assert!(sfo.unwrap().char_content().contains("你好"));
}

#[test]
fn source_file_object_empty() {
    let sfo = SourceFileObject::new("empty.rs", "", DEFAULT_MAX_SOURCE_BYTES);
    assert!(sfo.is_ok());
    assert_eq!(sfo.unwrap().char_content(), "");
}
