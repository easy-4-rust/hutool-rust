//! DEFLATE, GZIP, and path-safe ZIP operations.

use crate::{CoreError, Result};
use flate2::{
    Compression,
    read::{DeflateDecoder, GzDecoder, ZlibDecoder},
    write::{DeflateEncoder, GzEncoder, ZlibEncoder},
};
use std::{
    fs::{self, File},
    io::{Cursor, Read, Seek, Write},
    path::{Component, Path, PathBuf},
};
use zip::{CompressionMethod, ZipArchive, write::SimpleFileOptions};

/// Default maximum expansion ratio for ZIP entries.
pub const DEFAULT_MAX_SIZE_DIFF: u64 = 100;

/// Limits applied while reading or extracting ZIP archives.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ZipLimits {
    /// Maximum number of entries.
    pub max_entries: usize,
    /// Maximum total uncompressed bytes.
    pub max_uncompressed_bytes: u64,
    /// Maximum uncompressed/compressed ratio for a non-empty entry.
    pub max_size_diff: u64,
}

impl Default for ZipLimits {
    fn default() -> Self {
        Self {
            max_entries: 10_000,
            max_uncompressed_bytes: 1_073_741_824,
            max_size_diff: DEFAULT_MAX_SIZE_DIFF,
        }
    }
}

/// Streaming DEFLATE/ZLIB transformer.
pub struct Deflate<R, W> {
    source: R,
    target: W,
    nowrap: bool,
}

impl<R: Read, W: Write> Deflate<R, W> {
    /// Creates a transformer. `nowrap` selects raw DEFLATE instead of ZLIB.
    pub const fn new(source: R, target: W, nowrap: bool) -> Self {
        Self {
            source,
            target,
            nowrap,
        }
    }

    /// Returns the current target.
    pub const fn target(&self) -> &W {
        &self.target
    }

    /// Compresses the source at a level from 0 through 9.
    pub fn deflater(&mut self, level: u32) -> Result<()> {
        if level > 9 {
            return Err(CoreError::InvalidArgument {
                name: "level",
                reason: "must be between 0 and 9",
            });
        }
        deflate_into(&mut self.source, &mut self.target, self.nowrap, level)
    }

    /// Inflates the source according to the configured wrapper mode.
    pub fn inflater(&mut self) -> Result<()> {
        inflate_into(&mut self.source, &mut self.target, self.nowrap)
    }

    /// Returns the source and target after processing.
    pub fn into_inner(self) -> (R, W) {
        (self.source, self.target)
    }
}

/// Streaming GZIP transformer.
pub struct Gzip<R, W> {
    source: R,
    target: W,
}

impl<R: Read, W: Write> Gzip<R, W> {
    /// Creates a GZIP transformer.
    pub const fn new(source: R, target: W) -> Self {
        Self { source, target }
    }

    /// Returns the current target.
    pub const fn target(&self) -> &W {
        &self.target
    }

    /// Compresses the complete source.
    pub fn gzip(&mut self) -> Result<()> {
        gzip_into(&mut self.source, &mut self.target)
    }

    /// Decompresses the complete source.
    pub fn ungzip(&mut self) -> Result<()> {
        ungzip_into(&mut self.source, &mut self.target)
    }

    /// Returns the source and target after processing.
    pub fn into_inner(self) -> (R, W) {
        (self.source, self.target)
    }
}

fn deflate_into(
    source: &mut dyn Read,
    target: &mut dyn Write,
    nowrap: bool,
    level: u32,
) -> Result<()> {
    if nowrap {
        let mut encoder = DeflateEncoder::new(target, Compression::new(level));
        std::io::copy(source, &mut encoder)?;
        encoder.finish()?;
    } else {
        let mut encoder = ZlibEncoder::new(target, Compression::new(level));
        std::io::copy(source, &mut encoder)?;
        encoder.finish()?;
    }
    Ok(())
}

fn inflate_into(source: &mut dyn Read, target: &mut dyn Write, nowrap: bool) -> Result<()> {
    if nowrap {
        std::io::copy(&mut DeflateDecoder::new(source), target)?;
    } else {
        std::io::copy(&mut ZlibDecoder::new(source), target)?;
    }
    Ok(())
}

fn gzip_into(source: &mut dyn Read, target: &mut dyn Write) -> Result<()> {
    let mut encoder = GzEncoder::new(target, Compression::default());
    std::io::copy(source, &mut encoder)?;
    encoder.finish()?;
    Ok(())
}

fn ungzip_into(source: &mut dyn Read, target: &mut dyn Write) -> Result<()> {
    std::io::copy(&mut GzDecoder::new(source), target)?;
    Ok(())
}

/// Builder for deterministic ZIP archives.
pub struct ZipWriter<W: Write + Seek> {
    inner: zip::ZipWriter<W>,
    options: SimpleFileOptions,
}

impl<W: Write + Seek> ZipWriter<W> {
    /// Creates a ZIP writer.
    pub fn new(target: W) -> Self {
        Self {
            inner: zip::ZipWriter::new(target),
            options: SimpleFileOptions::default()
                .compression_method(CompressionMethod::Deflated)
                .unix_permissions(0o644),
        }
    }

    /// Sets compression level from 0 through 9.
    pub fn set_level(&mut self, level: i64) -> Result<&mut Self> {
        if !(0..=9).contains(&level) {
            return Err(CoreError::InvalidArgument {
                name: "level",
                reason: "must be between 0 and 9",
            });
        }
        self.options = self.options.compression_level(Some(level));
        Ok(self)
    }

    /// Sets the archive comment.
    pub fn set_comment(&mut self, comment: impl Into<Box<str>>) -> &mut Self {
        let _ = self.inner.set_comment(comment);
        self
    }

    /// Adds one file-like entry.
    pub fn add(&mut self, path: &str, source: &mut dyn Read) -> Result<&mut Self> {
        validate_relative(Path::new(path))?;
        self.inner
            .start_file(path.replace('\\', "/"), self.options)?;
        std::io::copy(source, &mut self.inner)?;
        Ok(self)
    }

    /// Adds one in-memory byte slice.
    pub fn add_bytes(&mut self, path: &str, bytes: &[u8]) -> Result<&mut Self> {
        self.add(path, &mut Cursor::new(bytes))
    }

    /// Adds an empty directory entry.
    pub fn add_directory(&mut self, path: &str) -> Result<&mut Self> {
        validate_relative(Path::new(path))?;
        let normalized = format!("{}/", path.trim_end_matches(['/', '\\']));
        self.inner
            .add_directory(normalized, self.options.unix_permissions(0o755))?;
        Ok(self)
    }

    /// Recursively adds a file or directory.
    pub fn add_path(&mut self, source: &Path, with_source_dir: bool) -> Result<()> {
        let source = source.canonicalize()?;
        let root = if source.is_dir() && !with_source_dir {
            source.clone()
        } else {
            source
                .parent()
                .map(Path::to_path_buf)
                .ok_or(CoreError::InvalidArgument {
                    name: "source",
                    reason: "must have a parent directory",
                })?
        };
        self.add_path_from(&source, &root)
    }

    fn add_path_from(&mut self, path: &Path, root: &Path) -> Result<()> {
        let relative = path
            .strip_prefix(root)
            .map_err(|_| CoreError::Compress("source escaped archive root".into()))?;
        let name = relative.to_string_lossy().replace('\\', "/");
        if path.is_dir() {
            if !name.is_empty() {
                self.add_directory(&name)?;
            }
            let mut entries = read_directory(path)?;
            entries.sort_by_key(std::fs::DirEntry::file_name);
            for entry in entries {
                self.add_path_from(&entry.path(), root)?;
            }
        } else if path.is_file() {
            let mut file = File::open(path)?;
            self.add(&name, &mut file)?;
        }
        Ok(())
    }

    /// Finishes the archive and returns its target.
    pub fn finish(self) -> Result<W> {
        Ok(self.inner.finish()?)
    }
}

/// Metadata exposed while visiting ZIP entries.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ZipEntry {
    /// Entry path.
    pub name: String,
    /// Whether the entry denotes a directory.
    pub is_directory: bool,
    /// Declared uncompressed size.
    pub size: u64,
    /// Declared compressed size.
    pub compressed_size: u64,
}

/// Bounded, path-safe ZIP reader.
pub struct ZipReader<R: Read + Seek> {
    archive: ZipArchive<R>,
    limits: ZipLimits,
    #[cfg(test)]
    validate_index_bias: usize,
    #[cfg(test)]
    read_index_bias: usize,
    #[cfg(test)]
    extract_index_bias: usize,
}

impl<R: Read + Seek> ZipReader<R> {
    /// Opens a ZIP archive from a seekable reader.
    pub fn new(reader: R) -> Result<Self> {
        Ok(Self {
            archive: ZipArchive::new(reader)?,
            limits: ZipLimits::default(),
            #[cfg(test)]
            validate_index_bias: 0,
            #[cfg(test)]
            read_index_bias: 0,
            #[cfg(test)]
            extract_index_bias: 0,
        })
    }

    /// Replaces extraction limits.
    pub const fn set_limits(&mut self, limits: ZipLimits) -> &mut Self {
        self.limits = limits;
        self
    }

    /// Updates only the maximum expansion ratio.
    pub const fn set_max_size_diff(&mut self, max_size_diff: u64) -> &mut Self {
        self.limits.max_size_diff = max_size_diff;
        self
    }

    /// Reads an entry fully, subject to configured limits.
    pub fn get(&mut self, path: &str) -> Result<Option<Vec<u8>>> {
        let Ok(mut entry) = self.archive.by_name(path) else {
            return Ok(None);
        };
        check_entry(&entry, self.limits)?;
        let mut bytes = Vec::new();
        entry
            .by_ref()
            .take(self.limits.max_uncompressed_bytes.saturating_add(1))
            .read_to_end(&mut bytes)?;
        if bytes.len() as u64 > self.limits.max_uncompressed_bytes {
            return Err(CoreError::Compress("ZIP entry exceeded byte limit".into()));
        }
        Ok(Some(bytes))
    }

    /// Visits every entry after validating its metadata.
    pub fn read(&mut self, mut visitor: impl FnMut(&ZipEntry)) -> Result<()> {
        self.read_with(&mut visitor)
    }

    fn read_with(&mut self, visitor: &mut dyn FnMut(&ZipEntry)) -> Result<()> {
        self.validate_archive()?;
        for index in 0..self.archive.len() {
            let entry_index = self.read_entry_index(index);
            let entry = self.archive.by_index(entry_index)?;
            visitor(&metadata(&entry));
        }
        Ok(())
    }

    /// Extracts selected entries beneath `destination`.
    pub fn read_to(
        &mut self,
        destination: impl AsRef<Path>,
        mut filter: impl FnMut(&ZipEntry) -> bool,
    ) -> Result<PathBuf> {
        self.read_to_with(destination.as_ref(), &mut filter)
    }

    fn read_to_with(
        &mut self,
        destination: &Path,
        filter: &mut dyn FnMut(&ZipEntry) -> bool,
    ) -> Result<PathBuf> {
        self.validate_archive()?;
        let root = prepare_destination(destination)?;
        for index in 0..self.archive.len() {
            let entry_index = self.extract_entry_index(index);
            let mut entry = self.archive.by_index(entry_index)?;
            let info = metadata(&entry);
            if !filter(&info) {
                continue;
            }
            let relative = entry
                .enclosed_name()
                .ok_or_else(|| CoreError::Compress(format!("unsafe ZIP path: {}", entry.name())))?;
            if entry
                .unix_mode()
                .is_some_and(|mode| mode & 0o170_000 == 0o120_000)
            {
                return Err(CoreError::Compress(format!(
                    "symbolic link ZIP entry: {}",
                    entry.name()
                )));
            }
            let output = root.join(relative);
            if entry.is_dir() {
                fs::create_dir_all(&output)?;
            } else {
                let parent = output.parent().unwrap_or(&root);
                fs::create_dir_all(parent)?;
                let mut file = File::create(output)?;
                std::io::copy(&mut entry, &mut file)?;
            }
        }
        Ok(root)
    }

    fn validate_archive(&mut self) -> Result<()> {
        if self.archive.len() > self.limits.max_entries {
            return Err(CoreError::Compress("ZIP entry count exceeded limit".into()));
        }
        let mut total = 0_u64;
        for index in 0..self.archive.len() {
            let entry_index = self.validate_entry_index(index);
            let entry = self.archive.by_index(entry_index)?;
            check_entry(&entry, self.limits)?;
            total = total.saturating_add(entry.size());
            if total > self.limits.max_uncompressed_bytes {
                return Err(CoreError::Compress("ZIP total size exceeded limit".into()));
            }
        }
        Ok(())
    }

    const fn validate_entry_index(&self, index: usize) -> usize {
        #[cfg(test)]
        {
            index + self.validate_index_bias
        }
        #[cfg(not(test))]
        {
            let _ = self.limits;
            index
        }
    }

    #[cfg(test)]
    const fn set_validate_index_bias(&mut self, bias: usize) {
        self.validate_index_bias = bias;
    }

    const fn read_entry_index(&self, index: usize) -> usize {
        #[cfg(test)]
        {
            index + self.read_index_bias
        }
        #[cfg(not(test))]
        {
            let _ = self.limits;
            index
        }
    }

    const fn extract_entry_index(&self, index: usize) -> usize {
        #[cfg(test)]
        {
            index + self.extract_index_bias
        }
        #[cfg(not(test))]
        {
            let _ = self.limits;
            index
        }
    }

    #[cfg(test)]
    const fn set_read_index_bias(&mut self, bias: usize) {
        self.read_index_bias = bias;
    }

    #[cfg(test)]
    const fn set_extract_index_bias(&mut self, bias: usize) {
        self.extract_index_bias = bias;
    }
}

/// Copies a source tree into an open ZIP writer.
pub struct ZipCopyVisitor {
    source: PathBuf,
}

impl ZipCopyVisitor {
    /// Creates a visitor rooted at `source`.
    pub fn new(source: impl Into<PathBuf>) -> Self {
        Self {
            source: source.into(),
        }
    }

    /// Copies the configured source into `writer`.
    pub fn copy_to<W: Write + Seek>(&self, writer: &mut ZipWriter<W>) -> Result<()> {
        writer.add_path(&self.source, false)
    }
}

fn metadata<R: Read>(entry: &zip::read::ZipFile<'_, R>) -> ZipEntry {
    ZipEntry {
        name: entry.name().to_owned(),
        is_directory: entry.is_dir(),
        size: entry.size(),
        compressed_size: entry.compressed_size(),
    }
}

fn check_entry<R: Read>(entry: &zip::read::ZipFile<'_, R>, limits: ZipLimits) -> Result<()> {
    if entry.is_dir() {
        return Ok(());
    }
    let compressed_size = entry.compressed_size().max(1);
    if compressed_size.saturating_mul(limits.max_size_diff) < entry.size() {
        return Err(CoreError::Compress(format!(
            "ZIP bomb ratio exceeded: {}",
            entry.name()
        )));
    }
    Ok(())
}

fn validate_relative(path: &Path) -> Result<()> {
    if path.as_os_str().is_empty()
        || path.components().any(|component| {
            matches!(
                component,
                Component::ParentDir | Component::RootDir | Component::Prefix(_)
            )
        })
    {
        return Err(CoreError::Compress(format!(
            "unsafe ZIP path: {}",
            path.display()
        )));
    }
    Ok(())
}

fn read_directory(path: &Path) -> Result<Vec<fs::DirEntry>> {
    #[cfg(test)]
    let entries: Box<dyn Iterator<Item = std::io::Result<fs::DirEntry>>> =
        if path.ends_with(".hutool-iteration-failure") {
            Box::new(std::iter::once(Err(std::io::Error::other(
                "injected directory iteration failure",
            ))))
        } else {
            Box::new(fs::read_dir(path)?)
        };
    #[cfg(not(test))]
    let entries = fs::read_dir(path)?;
    Ok(entries.collect::<std::io::Result<Vec<_>>>()?)
}

fn prepare_destination(path: &Path) -> Result<PathBuf> {
    fs::create_dir_all(path)?;
    #[cfg(test)]
    if path.ends_with(".hutool-canonicalize-failure") || path.ends_with(".hutool-remove-failure") {
        fs::remove_dir(path)?;
    }
    Ok(path.canonicalize()?)
}

/// Creates a seekable in-memory ZIP writer.
pub fn memory_zip_writer() -> ZipWriter<Cursor<Vec<u8>>> {
    ZipWriter::new(Cursor::new(Vec::new()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        cell::{Cell, RefCell},
        rc::Rc,
    };

    fn accept_all(_: &ZipEntry) -> bool {
        true
    }

    fn accept_b(entry: &ZipEntry) -> bool {
        entry.name.ends_with("b.txt")
    }

    fn ignore_entry(_: &ZipEntry) {}

    struct FailIo;

    impl Read for FailIo {
        fn read(&mut self, _: &mut [u8]) -> std::io::Result<usize> {
            Err(std::io::Error::other("injected failure"))
        }
    }

    #[derive(Clone)]
    struct ControlledIo {
        inner: Rc<RefCell<Cursor<Vec<u8>>>>,
        fail: Rc<Cell<bool>>,
    }

    impl ControlledIo {
        fn new() -> Self {
            Self {
                inner: Rc::new(RefCell::new(Cursor::new(Vec::new()))),
                fail: Rc::new(Cell::new(false)),
            }
        }

        fn set_failed(&self, failed: bool) {
            self.fail.set(failed);
        }
    }

    impl Write for ControlledIo {
        fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
            if self.fail.get() {
                return Err(std::io::Error::other("injected write failure"));
            }
            self.inner.borrow_mut().write(bytes)
        }

        fn flush(&mut self) -> std::io::Result<()> {
            if self.fail.get() {
                return Err(std::io::Error::other("injected flush failure"));
            }
            self.inner.borrow_mut().flush()
        }
    }

    impl Seek for ControlledIo {
        fn seek(&mut self, position: std::io::SeekFrom) -> std::io::Result<u64> {
            if self.fail.get() {
                return Err(std::io::Error::other("injected seek failure"));
            }
            self.inner.borrow_mut().seek(position)
        }
    }

    fn zip_bytes(entries: &[(&str, &[u8])]) -> Vec<u8> {
        let mut writer = memory_zip_writer();
        for (name, bytes) in entries {
            writer.add_bytes(name, bytes).expect("add ZIP entry");
        }
        writer.finish().expect("finish ZIP").into_inner()
    }

    #[test]
    fn deflate_and_gzip_round_trip_both_wrapper_modes() {
        for nowrap in [false, true] {
            let mut compressor = Deflate::new(&b"compress me"[..], Vec::new(), nowrap);
            assert!(compressor.target().is_empty());
            compressor.deflater(6).expect("deflate");
            let (_, compressed) = compressor.into_inner();
            let mut decompressor = Deflate::new(compressed.as_slice(), Vec::new(), nowrap);
            decompressor.inflater().expect("inflate");
            let (_, plain) = decompressor.into_inner();
            assert_eq!(plain, b"compress me");
        }
        assert!(
            Deflate::new(&b"x"[..], Vec::new(), false)
                .deflater(10)
                .is_err()
        );

        let mut compressor = Gzip::new(&b"gzip me"[..], Vec::new());
        assert!(compressor.target().is_empty());
        compressor.gzip().expect("gzip");
        let (_, compressed) = compressor.into_inner();
        let mut decompressor = Gzip::new(compressed.as_slice(), Vec::new());
        decompressor.ungzip().expect("ungzip");
        assert_eq!(decompressor.into_inner().1, b"gzip me");
        assert!(Gzip::new(&b"invalid"[..], Vec::new()).ungzip().is_err());
        for nowrap in [false, true] {
            assert!(
                Deflate::new(FailIo, Vec::new(), nowrap)
                    .deflater(1)
                    .is_err()
            );
            assert!(
                Deflate::new(&b"invalid"[..], Vec::new(), nowrap)
                    .inflater()
                    .is_err()
            );
        }
        assert!(
            Deflate::new(FailIo, Vec::new(), false)
                .deflater(10)
                .is_err()
        );
        assert!(
            Deflate::new(&b""[..], ControlledIo::new(), false)
                .deflater(10)
                .is_err()
        );
        assert!(Gzip::new(FailIo, Vec::new()).gzip().is_err());
        for nowrap in [false, true] {
            Deflate::new(&b""[..], ControlledIo::new(), nowrap)
                .deflater(1)
                .expect("empty compression");
            let failed = ControlledIo::new();
            failed.set_failed(true);
            assert!(Deflate::new(&b""[..], failed, nowrap).deflater(1).is_err());
        }
        let failed = ControlledIo::new();
        failed.set_failed(true);
        assert!(Gzip::new(&b""[..], failed).gzip().is_err());
        ignore_entry(&ZipEntry {
            name: String::new(),
            is_directory: false,
            size: 0,
            compressed_size: 0,
        });
    }

    #[test]
    #[expect(
        clippy::too_many_lines,
        reason = "covers one staged ZIP writer lifecycle"
    )]
    fn zip_writer_adds_streams_directories_paths_and_comments() {
        let mut controlled_methods = ControlledIo::new();
        controlled_methods
            .write_all(b"x")
            .expect("controlled write");
        controlled_methods.flush().expect("controlled flush");
        controlled_methods
            .seek(std::io::SeekFrom::Start(0))
            .expect("controlled seek");
        controlled_methods.set_failed(true);
        assert!(controlled_methods.write(b"x").is_err());
        assert!(controlled_methods.flush().is_err());
        assert!(
            controlled_methods
                .seek(std::io::SeekFrom::Start(0))
                .is_err()
        );

        let directory = tempfile::tempdir().expect("temporary directory");
        let tree = directory.path().join("tree");
        fs::create_dir(&tree).expect("create tree");
        fs::write(tree.join("a.txt"), b"a").expect("write file");
        fs::create_dir(tree.join("empty")).expect("create empty directory");

        let mut writer = memory_zip_writer();
        writer
            .set_level(9)
            .expect("compression level")
            .set_comment("HiTool");
        writer
            .add_bytes("manual.txt", b"manual")
            .expect("manual entry");
        writer.add_directory("manual-empty").expect("empty entry");
        writer.add_path(&tree, true).expect("source tree");
        let bytes = writer.finish().expect("finish archive").into_inner();
        let archive = ZipArchive::new(Cursor::new(bytes)).expect("open archive");
        assert_eq!(archive.comment(), b"HiTool");
        assert!(archive.len() >= 4);

        assert!(memory_zip_writer().set_level(-1).is_err());
        assert!(memory_zip_writer().add_bytes("../escape", b"x").is_err());
        assert!(memory_zip_writer().add_directory("").is_err());
        assert!(
            memory_zip_writer()
                .add_path(&directory.path().join("missing"), false)
                .is_err()
        );

        let controlled = ControlledIo::new();
        let mut successful = ZipWriter::new(controlled.clone());
        successful.add_bytes("ok", b"ok").expect("controlled file");
        successful
            .add_directory("manual-empty")
            .expect("controlled directory");
        successful
            .add_path(&tree, false)
            .expect("controlled recursive tree");
        successful.finish().expect("controlled finish");
        let mut controlled_private = ZipWriter::new(ControlledIo::new());
        assert!(
            controlled_private
                .add_path_from(
                    &tree,
                    directory.path().join("controlled-unrelated").as_path()
                )
                .is_err()
        );
        assert!(
            controlled_private
                .add_path(&directory.path().join("controlled-missing"), false)
                .is_err()
        );

        let controlled = ControlledIo::new();
        let mut failed_start = ZipWriter::new(controlled.clone());
        controlled.set_failed(true);
        assert!(failed_start.add_bytes("failed", b"x").is_err());
        let mut invalid_controlled = ZipWriter::new(ControlledIo::new());
        assert!(invalid_controlled.add_bytes("../failed", b"x").is_err());
        assert!(invalid_controlled.add_directory("").is_err());

        let mut failed_source = ZipWriter::new(ControlledIo::new());
        let mut failing_reader = FailIo;
        assert!(failed_source.add("failed", &mut failing_reader).is_err());

        let controlled = ControlledIo::new();
        let mut failed_directory = ZipWriter::new(controlled.clone());
        controlled.set_failed(true);
        assert!(failed_directory.add_directory("failed").is_err());

        let controlled = ControlledIo::new();
        let mut failed_finish = ZipWriter::new(controlled.clone());
        failed_finish
            .add_bytes("ok", b"ok")
            .expect("entry before failure");
        controlled.set_failed(true);
        assert!(failed_finish.finish().is_err());

        let controlled = ControlledIo::new();
        let mut failed_recursive_directory = ZipWriter::new(controlled.clone());
        controlled.set_failed(true);
        assert!(
            failed_recursive_directory
                .add_path(&tree.join("empty"), true)
                .is_err()
        );

        let controlled = ControlledIo::new();
        let mut failed_recursive_file = ZipWriter::new(controlled.clone());
        controlled.set_failed(true);
        assert!(
            failed_recursive_file
                .add_path(&tree.join("a.txt"), false)
                .is_err()
        );

        let iteration_root = directory.path().join("iteration-root");
        let iteration_failure = iteration_root.join(".hutool-iteration-failure");
        fs::create_dir_all(&iteration_failure).expect("iteration failure fixture");
        assert!(
            memory_zip_writer()
                .add_path(&iteration_root, false)
                .is_err()
        );
        let mut controlled_iteration = ZipWriter::new(ControlledIo::new());
        assert!(
            controlled_iteration
                .add_path(&iteration_root, false)
                .is_err()
        );

        #[cfg(unix)]
        {
            use std::os::unix::fs::{PermissionsExt, symlink};

            assert!(memory_zip_writer().add_path(Path::new("/"), true).is_err());
            assert!(
                ZipWriter::new(ControlledIo::new())
                    .add_path(Path::new("/"), true)
                    .is_err()
            );
            let mut private_writer = memory_zip_writer();
            assert!(
                private_writer
                    .add_path_from(&tree, directory.path().join("unrelated").as_path())
                    .is_err()
            );

            let broken = directory.path().join("broken-link");
            symlink(directory.path().join("absent-target"), &broken).expect("broken symlink");
            private_writer
                .add_path_from(&broken, directory.path())
                .expect("unsupported filesystem item is skipped");
            ZipWriter::new(ControlledIo::new())
                .add_path_from(&broken, directory.path())
                .expect("controlled unsupported item is skipped");

            let unreadable_directory = directory.path().join("unreadable-directory");
            fs::create_dir(&unreadable_directory).expect("unreadable directory");
            fs::set_permissions(&unreadable_directory, fs::Permissions::from_mode(0o000))
                .expect("remove directory permissions");
            let directory_result =
                private_writer.add_path_from(&unreadable_directory, directory.path());
            let mut public_directory_writer = memory_zip_writer();
            let public_directory_result =
                public_directory_writer.add_path(&unreadable_directory, true);
            fs::set_permissions(&unreadable_directory, fs::Permissions::from_mode(0o700))
                .expect("restore directory permissions");
            assert!(directory_result.is_err());
            assert!(public_directory_result.is_err());

            let unreadable_file = directory.path().join("unreadable-file");
            fs::write(&unreadable_file, b"secret").expect("unreadable file");
            fs::set_permissions(&unreadable_file, fs::Permissions::from_mode(0o000))
                .expect("remove file permissions");
            let file_result = private_writer.add_path_from(&unreadable_file, directory.path());
            let mut controlled_file_writer = ZipWriter::new(ControlledIo::new());
            let controlled_file_result =
                controlled_file_writer.add_path_from(&unreadable_file, directory.path());
            fs::set_permissions(&unreadable_file, fs::Permissions::from_mode(0o600))
                .expect("restore file permissions");
            assert!(file_result.is_err());
            assert!(controlled_file_result.is_err());
        }

        let mut cursor_failures = memory_zip_writer();
        cursor_failures
            .add_bytes("duplicate", b"first")
            .expect("first duplicate");
        assert!(cursor_failures.add_bytes("duplicate", b"second").is_err());
        let mut failing_reader = FailIo;
        assert!(
            cursor_failures
                .add("read-failure", &mut failing_reader)
                .is_err()
        );

        let mut duplicate_tree = memory_zip_writer();
        duplicate_tree
            .add_directory("empty")
            .expect("first empty directory");
        duplicate_tree
            .add_bytes("a.txt", b"first")
            .expect("first tree file");
        assert!(duplicate_tree.add_path(&tree, false).is_err());
    }

    #[test]
    #[expect(
        clippy::too_many_lines,
        reason = "covers one staged ZIP reader lifecycle"
    )]
    fn zip_reader_gets_visits_filters_extracts_and_limits() {
        let mut archive_writer = memory_zip_writer();
        archive_writer
            .add_bytes("a.txt", b"alpha")
            .expect("first file");
        archive_writer.add_directory("dir").expect("directory");
        archive_writer
            .add_bytes("dir/b.txt", b"beta")
            .expect("second file");
        let bytes = archive_writer.finish().expect("finish").into_inner();
        let mut reader = ZipReader::new(Cursor::new(bytes.clone())).expect("ZIP reader");
        assert_eq!(
            reader.get("a.txt").expect("get entry"),
            Some(b"alpha".to_vec())
        );
        assert_eq!(reader.get("missing").expect("missing entry"), None);
        let mut names = Vec::new();
        reader
            .read(|entry| names.push(entry.name.clone()))
            .expect("visit archive");
        assert_eq!(names, ["a.txt", "dir/", "dir/b.txt"]);

        let destination = tempfile::tempdir().expect("destination");
        reader
            .read_to(destination.path(), accept_b)
            .expect("extract selected entry");
        assert_eq!(
            fs::read(destination.path().join("dir/b.txt")).expect("read extracted"),
            b"beta"
        );
        assert!(!destination.path().join("a.txt").exists());

        let all_destination = tempfile::tempdir().expect("all destination");
        ZipReader::new(Cursor::new(bytes.clone()))
            .expect("reader")
            .read_to(all_destination.path(), accept_all)
            .expect("extract all");
        assert!(all_destination.path().join("dir").is_dir());

        let mut entry_limited = ZipReader::new(Cursor::new(bytes.clone())).expect("reader");
        entry_limited.set_limits(ZipLimits {
            max_entries: 1,
            ..ZipLimits::default()
        });
        assert!(entry_limited.read(ignore_entry).is_err());
        assert!(
            entry_limited
                .read_to(destination.path(), accept_all)
                .is_err()
        );
        let mut byte_limited = ZipReader::new(Cursor::new(bytes.clone())).expect("reader");
        byte_limited.set_limits(ZipLimits {
            max_uncompressed_bytes: 3,
            ..ZipLimits::default()
        });
        assert!(byte_limited.get("a.txt").is_err());
        let mut total_limited = ZipReader::new(Cursor::new(bytes.clone())).expect("reader");
        total_limited.set_limits(ZipLimits {
            max_uncompressed_bytes: 8,
            ..ZipLimits::default()
        });
        assert!(total_limited.read(ignore_entry).is_err());
        assert!(ZipReader::new(Cursor::new(b"not a zip".to_vec())).is_err());

        let mut invalid_index = ZipReader::new(Cursor::new(bytes.clone())).expect("reader");
        invalid_index.set_read_index_bias(100);
        assert!(invalid_index.read(ignore_entry).is_err());
        let mut invalid_index = ZipReader::new(Cursor::new(bytes.clone())).expect("reader");
        invalid_index.set_extract_index_bias(100);
        assert!(
            invalid_index
                .read_to(destination.path(), accept_all)
                .is_err()
        );
        let mut invalid_index = ZipReader::new(Cursor::new(bytes.clone())).expect("reader");
        invalid_index.set_validate_index_bias(100);
        assert!(invalid_index.read(ignore_entry).is_err());

        let destination_parent = tempfile::tempdir().expect("destination parent");
        let destination_file = destination_parent.path().join("destination-file");
        fs::write(&destination_file, b"file").expect("destination file");
        let mut reader = ZipReader::new(Cursor::new(bytes.clone())).expect("reader");
        assert!(reader.read_to(&destination_file, accept_all).is_err());

        let canonicalize_failure = destination_parent
            .path()
            .join(".hutool-canonicalize-failure");
        let mut reader = ZipReader::new(Cursor::new(bytes.clone())).expect("reader");
        assert!(reader.read_to(&canonicalize_failure, accept_all).is_err());

        let remove_failure = destination_parent.path().join(".hutool-remove-failure");
        fs::create_dir(&remove_failure).expect("remove failure directory");
        fs::write(remove_failure.join("child"), b"child").expect("non-empty directory");
        let mut reader = ZipReader::new(Cursor::new(bytes.clone())).expect("reader");
        assert!(reader.read_to(&remove_failure, accept_all).is_err());

        let directory_collision = tempfile::tempdir().expect("directory collision");
        fs::write(directory_collision.path().join("dir"), b"file").expect("collision file");
        let mut reader = ZipReader::new(Cursor::new(bytes.clone())).expect("reader");
        assert!(
            reader
                .read_to(directory_collision.path(), accept_all)
                .is_err()
        );

        let nested_only = zip_bytes(&[("dir/b.txt", b"beta")]);
        let parent_collision = tempfile::tempdir().expect("parent collision");
        fs::write(parent_collision.path().join("dir"), b"file").expect("parent collision file");
        let mut reader = ZipReader::new(Cursor::new(nested_only)).expect("reader");
        assert!(reader.read_to(parent_collision.path(), accept_all).is_err());

        let file_collision = tempfile::tempdir().expect("file collision");
        fs::create_dir(file_collision.path().join("a.txt")).expect("collision directory");
        let mut reader = ZipReader::new(Cursor::new(bytes)).expect("reader");
        assert!(reader.read_to(file_collision.path(), accept_all).is_err());

        let mut corrupt = zip_bytes(&[("corrupt.txt", b"payload payload payload")]);
        let name_offset = corrupt
            .windows(b"corrupt.txt".len())
            .position(|window| window == b"corrupt.txt")
            .expect("local ZIP name");
        corrupt[name_offset + b"corrupt.txt".len()] ^= 0xff;
        let mut reader = ZipReader::new(Cursor::new(corrupt.clone())).expect("corrupt reader");
        assert!(reader.get("corrupt.txt").is_err());
        let corrupt_destination = tempfile::tempdir().expect("corrupt destination");
        let mut reader = ZipReader::new(Cursor::new(corrupt)).expect("corrupt reader");
        assert!(
            reader
                .read_to(corrupt_destination.path(), accept_all)
                .is_err()
        );
    }

    #[test]
    fn zip_reader_rejects_unsafe_bomb_and_symlink_entries() {
        let mut raw = zip::ZipWriter::new(Cursor::new(Vec::new()));
        raw.start_file("../escape", SimpleFileOptions::default())
            .expect("unsafe entry");
        raw.write_all(b"x").expect("write entry");
        let unsafe_bytes = raw.finish().expect("finish").into_inner();
        let mut reader = ZipReader::new(Cursor::new(unsafe_bytes)).expect("reader");
        let destination = tempfile::tempdir().expect("destination");
        assert!(reader.read_to(destination.path(), accept_all).is_err());

        let zeros = vec![0; 20_000];
        let bomb = zip_bytes(&[("zeros", &zeros)]);
        let mut reader = ZipReader::new(Cursor::new(bomb.clone())).expect("reader");
        reader.set_max_size_diff(1);
        assert!(reader.read(ignore_entry).is_err());
        let mut reader = ZipReader::new(Cursor::new(bomb.clone())).expect("reader");
        reader.set_max_size_diff(1);
        assert!(reader.get("zeros").is_err());
        let mut reader = ZipReader::new(Cursor::new(bomb)).expect("reader");
        reader.set_max_size_diff(1);
        assert!(reader.read_to(destination.path(), accept_all).is_err());

        let mut raw = zip::ZipWriter::new(Cursor::new(Vec::new()));
        raw.add_symlink("link", "target", SimpleFileOptions::default())
            .expect("symlink entry");
        let bytes = raw.finish().expect("finish").into_inner();
        let mut reader = ZipReader::new(Cursor::new(bytes)).expect("reader");
        assert!(reader.read_to(destination.path(), accept_all).is_err());
    }

    #[test]
    fn zip_copy_visitor_copies_relative_tree() {
        let directory = tempfile::tempdir().expect("temporary directory");
        fs::write(directory.path().join("copied.txt"), b"copied").expect("write source");
        let visitor = ZipCopyVisitor::new(directory.path());
        let mut writer = memory_zip_writer();
        visitor.copy_to(&mut writer).expect("copy tree");
        let bytes = writer.finish().expect("finish").into_inner();
        let mut reader = ZipReader::new(Cursor::new(bytes)).expect("reader");
        assert_eq!(
            reader.get("copied.txt").expect("get copied"),
            Some(b"copied".to_vec())
        );
    }
}
