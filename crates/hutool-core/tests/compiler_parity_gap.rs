//! `cn.hutool.core.compiler` 缺口 parity
//!
//! 对齐: `cn.hutool.core.compiler.JavaSourceCompilerTest`
//! Rust 侧对应 `RustSourceCompiler`（rustc 进程编译）。

use hutool_core::{CompilerException, RustSourceCompiler};
use std::env;
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

/// 构造唯一临时输出目录。
fn unique_out(prefix: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let out = env::temp_dir().join(format!("{prefix}_{nanos}"));
    let _ = fs::remove_dir_all(&out);
    fs::create_dir_all(&out).unwrap();
    out
}

/// 对齐 Java: `JavaSourceCompilerTest.testCompile()`
#[test]
fn java_source_compiler_test_compile() {
    let out = unique_out("hutool_compiler_gap_ok");

    let mut compiler = RustSourceCompiler::new();
    compiler
        .add_source("lib.rs", "pub fn answer() -> i32 { 42 }\n")
        .unwrap();
    let result = compiler.compile_to(&out);
    assert!(result.is_ok(), "compile should succeed: {:?}", result.err());
    let output = result.unwrap();
    assert!(
        !output.artifacts.is_empty() || out.read_dir().unwrap().next().is_some(),
        "expected compile artifacts under {}",
        out.display()
    );
}

/// 对齐 Java: `JavaSourceCompilerTest.testErrorCompile()`
#[test]
fn java_source_compiler_test_error_compile() {
    let out = unique_out("hutool_compiler_gap_err");

    let mut compiler = RustSourceCompiler::new();
    compiler
        .add_source("lib.rs", "fn broken( { syntax error\n")
        .unwrap();
    let result = compiler.compile_to(&out);
    assert!(result.is_err());
    let err = result.unwrap_err();
    // 对齐 Java：失败时抛出 CompilerException
    let _: &CompilerException = &err;
    assert!(!err.to_string().is_empty());
}
