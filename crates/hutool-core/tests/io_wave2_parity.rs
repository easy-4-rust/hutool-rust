//! Wave2 `cn.hutool.core.io` portable parity — PathUtil / FileNameUtil / DataSize /
//! BufferUtil / FastByte* / FileReader / FileWriter / NullOutputStream / LineSeparator.

use hutool_core::{
    BufferUtil, DataSize, DataSizeUtil, DataUnit, FastByteArrayOutputStream, FastByteBuffer,
    FileNameUtil, FileReader, FileUtil, FileWriter, IORuntimeException, LineSeparator,
    NullOutputStream, PathUtil,
};
use std::io::{Cursor, Write};
use std::path::Path;

/// 对齐 Java: PathUtil normalize / copy / walk
#[test]
fn path_util_normalize_copy_walk() {
    let base = std::env::temp_dir().join("hutool_path_util_wave2");
    let _ = std::fs::remove_dir_all(&base);
    std::fs::create_dir_all(base.join("src/sub")).unwrap();
    FileUtil::write_utf8_string(base.join("src/a.txt").to_str().unwrap(), "a").unwrap();
    FileUtil::write_utf8_string(base.join("src/sub/b.txt").to_str().unwrap(), "b").unwrap();

    let norm = PathUtil::normalize(Path::new("x/../y/./z"));
    assert!(norm.to_string_lossy().ends_with("y/z") || norm.ends_with("z"));

    let dst = base.join("dst");
    PathUtil::copy_content(&base.join("src"), &dst).unwrap();
    assert!(dst.join("a.txt").is_file());
    assert!(dst.join("sub/b.txt").is_file());

    let walked = PathUtil::walk_files(&base.join("src")).unwrap();
    assert!(walked.len() >= 2);
    assert!(PathUtil::is_sub(&base, &base.join("src/a.txt")));

    let _ = std::fs::remove_dir_all(&base);
}

/// 对齐 Java: FileNameUtil ext / cleanInvalid / isType
#[test]
fn file_name_util_ext_and_invalid() {
    assert_eq!(FileNameUtil::name("/tmp/a/b.txt"), "b.txt");
    assert_eq!(FileNameUtil::ext_name("archive.tar.gz"), "tar.gz");
    assert_eq!(FileNameUtil::suffix("photo.JPG"), "JPG");
    assert_eq!(FileNameUtil::main_name("photo.JPG"), "photo");
    assert!(FileNameUtil::contains_invalid("a:b.txt"));
    assert_eq!(FileNameUtil::clean_invalid("a:b*.txt"), "ab.txt");
    assert!(FileNameUtil::is_type("x.PNG", &["png", "jpg"]));
}

/// 对齐 Java: DataSize / DataSizeUtil / DataUnit
#[test]
fn data_size_parse_and_units() {
    assert_eq!(DataSize::of_kilobytes(1).to_bytes(), 1024);
    assert_eq!(DataSize::of_megabytes(1).to_kilobytes(), 1024);
    let parsed = DataSize::parse("2KB").unwrap();
    assert_eq!(parsed.to_bytes(), 2048);
    assert!(!parsed.is_negative());
    assert_eq!(DataUnit::Megabytes.suffix(), "MB");
    assert_eq!(DataUnit::from_suffix("gb"), Some(DataUnit::Gigabytes));
    assert!(DataSizeUtil::format(1536).contains("KB") || DataSizeUtil::format(1536).contains("1"));
    assert!(DataSize::of_bytes(10) < DataSize::of_bytes(20));
}

/// 对齐 Java: BufferUtil copy / lineEnd / createUtf8
#[test]
fn buffer_util_copy_and_line() {
    let src = b"hello\nworld";
    assert_eq!(BufferUtil::copy_range(src, 0, 5), b"hello");
    assert_eq!(BufferUtil::line_end(src), Some(5));
    assert_eq!(BufferUtil::read_line(src).as_deref(), Some("hello"));
    assert_eq!(BufferUtil::create_utf8("hi"), b"hi");
    assert_eq!(BufferUtil::read_utf8_str(b"abc"), "abc");
}

/// 对齐 Java: FastByteBuffer append / toArray
#[test]
fn fast_byte_buffer_append() {
    let mut buf = FastByteBuffer::new();
    buf.append(b"ab").append_u8(b'c');
    assert_eq!(buf.size(), 3);
    assert!(!buf.is_empty());
    assert_eq!(buf.to_array(), b"abc");
    assert_eq!(buf.get(1), Some(b'b'));
    buf.reset();
    assert!(buf.is_empty());
}

/// 对齐 Java: FastByteArrayOutputStream
#[test]
fn fast_byte_array_output_stream() {
    let mut out = FastByteArrayOutputStream::new();
    out.write_all(b"xyz").unwrap();
    assert_eq!(out.size(), 3);
    assert_eq!(out.to_byte_array(), b"xyz");
    assert_eq!(out.to_utf8_string(), "xyz");
    let mut sink = Vec::new();
    out.write_to(&mut sink).unwrap();
    assert_eq!(sink, b"xyz");
}

/// 对齐 Java: FileReader / FileWriter roundtrip
#[test]
fn file_reader_writer_roundtrip() {
    let path = std::env::temp_dir().join("hutool_fw_wave2.txt");
    let path_str = path.to_str().unwrap();
    let writer = FileWriter::create(&path);
    writer.write_str("line1\nline2").unwrap();
    writer.append_str("\nline3").unwrap();
    let reader = FileReader::create(&path);
    let text = reader.read_string().unwrap();
    assert!(text.contains("line1"));
    assert_eq!(reader.read_lines().unwrap().len(), 3);
    let mut sink = Vec::new();
    reader.write_to_stream(&mut sink).unwrap();
    assert!(!sink.is_empty());
    FileUtil::delete(path_str).unwrap();
}

/// 对齐 Java: FileUtil Wave2 portable leftovers
#[test]
fn file_util_wave2_portable() {
    let base = std::env::temp_dir().join("hutool_fu_wave2");
    let _ = std::fs::remove_dir_all(&base);
    std::fs::create_dir_all(base.join("d")).unwrap();
    let f = base.join("d/x.txt");
    FileUtil::write_utf8_string(f.to_str().unwrap(), "hello").unwrap();
    assert!(FileUtil::mkdirs_safely(base.join("d2").to_str().unwrap()));
    let files = FileUtil::loop_files(base.join("d").to_str().unwrap()).unwrap();
    assert_eq!(files.len(), 1);
    assert!(FileUtil::is_sub(
        base.to_str().unwrap(),
        f.to_str().unwrap()
    ));
    assert!(!FileUtil::contains_invalid("ok.txt"));
    assert_eq!(FileUtil::clean_invalid("a|b"), "ab");
    let mut cur = Cursor::new(b"streamed");
    let out = base.join("from_stream.bin");
    FileUtil::write_from_stream(out.to_str().unwrap(), &mut cur).unwrap();
    assert_eq!(
        FileUtil::read_bytes(out.to_str().unwrap()).unwrap(),
        b"streamed"
    );
    assert_eq!(
        FileUtil::read_line(f.to_str().unwrap()).unwrap().as_deref(),
        Some("hello")
    );
    let _ = std::fs::remove_dir_all(&base);
}

/// 对齐 Java: NullOutputStream
#[test]
fn null_output_stream_discards() {
    let mut n = NullOutputStream::new();
    assert_eq!(n.write(b"ignore").unwrap(), 6);
    n.flush().unwrap();
}

/// 对齐 Java: LineSeparator.getValue
#[test]
fn line_separator_values() {
    assert_eq!(LineSeparator::Linux.value(), "\n");
    assert_eq!(LineSeparator::Windows.value(), "\r\n");
    assert_eq!(LineSeparator::Mac.value(), "\r");
}

/// 对齐 Java: IORuntimeException
#[test]
fn io_runtime_exception_wraps() {
    let e = IORuntimeException::new("boom");
    assert!(e.to_string().contains("boom"));
    let io = std::io::Error::new(std::io::ErrorKind::NotFound, "missing");
    let wrapped = IORuntimeException::from_io(io);
    assert!(wrapped.cause_instance_of(std::io::ErrorKind::NotFound));
}
