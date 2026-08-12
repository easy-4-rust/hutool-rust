//! hutool-extra parity tests
use hutool_extra as he;

/// 模块与错误类型可构造（替代 expansion 占位 `extra_module_exists`）。
#[test]
fn extra_module_exists() {
    let err = he::ExtraError::ArchiveLimit("test");
    assert!(err.to_string().contains("archive limit"));
}

#[test]
fn create_zip_test() {
    let entries = vec![("test.txt", b"hello world" as &[u8])];
    let zip_bytes = he::archive::create_zip(&entries).unwrap();
    assert!(!zip_bytes.is_empty(), "ZIP 应非空");
}

#[test]
fn qrcode_svg_test() {
    let svg = he::qrcode::to_svg("hello", 100).unwrap();
    assert!(svg.contains("<svg"), "QR SVG 应含 <svg");
}
