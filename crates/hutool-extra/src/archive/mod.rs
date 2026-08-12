//! ZIP creation and bounded, path-safe extraction.
//!
//! 对齐: `cn.hutool.extra.compress.CompressUtil`
//! 对齐: `cn.hutool.core.util.ZipUtil`（extra 侧安全 ZIP 子集）

use crate::{ExtraError, Result};
use std::{
    fs::{self, File},
    io::{Cursor, Read, Seek, Write},
    path::{Component, Path},
};
use zip::{CompressionMethod, ZipArchive, ZipWriter, write::SimpleFileOptions};

mod compress_util;
mod extraction_limits;
mod zip_util;

pub use compress_util::CompressUtil;
pub use extraction_limits::ExtractionLimits;
pub use zip_util::ZipUtil;

/// Creates an in-memory ZIP archive from named byte entries.
///
/// 对齐 Hutool `ZipUtil.zip`；条目名经过路径安全检查。
pub fn create_zip(entries: &[(&str, &[u8])]) -> Result<Vec<u8>> {
    let mut writer = ZipWriter::new(Cursor::new(Vec::new()));
    let options = SimpleFileOptions::default()
        .compression_method(CompressionMethod::Deflated)
        .unix_permissions(0o644);
    for (name, bytes) in entries {
        validate_relative_path(Path::new(name))?;
        writer.start_file(*name, options)?;
        writer.write_all(bytes)?;
    }
    Ok(writer.finish()?.into_inner())
}

/// Safely extracts a ZIP archive beneath `destination`, enforcing resource limits.
///
/// 对齐 Hutool `ZipUtil.unzip`；拒绝符号链接、目录穿越与超限条目。
pub fn extract_zip<R: Read + Seek>(
    reader: R,
    destination: impl AsRef<Path>,
    limits: ExtractionLimits,
) -> Result<()> {
    let mut archive = ZipArchive::new(reader)?;
    if archive.len() > limits.max_entries {
        return Err(ExtraError::ArchiveLimit("entry count"));
    }

    fs::create_dir_all(destination.as_ref())?;
    let root = destination.as_ref().canonicalize()?;
    let mut declared_total = 0_u64;

    for index in 0..archive.len() {
        let mut entry = archive.by_index(index)?;
        let name = entry.name().to_owned();
        let relative = entry
            .enclosed_name()
            .ok_or_else(|| ExtraError::UnsafeArchivePath(name.clone()))?;
        validate_relative_path(&relative)?;
        if entry
            .unix_mode()
            .is_some_and(|mode| mode & 0o170_000 == 0o120_000)
        {
            return Err(ExtraError::SymlinkEntry(name));
        }

        declared_total = declared_total
            .checked_add(entry.size())
            .ok_or(ExtraError::ArchiveLimit("uncompressed byte count"))?;
        if declared_total > limits.max_uncompressed_bytes {
            return Err(ExtraError::ArchiveLimit("uncompressed byte count"));
        }

        let output_path = root.join(&relative);
        if entry.is_dir() {
            fs::create_dir_all(&output_path)?;
            ensure_beneath_root(&root, &output_path)?;
            continue;
        }
        let parent = output_path
            .parent()
            .ok_or_else(|| ExtraError::UnsafeArchivePath(name.clone()))?;
        fs::create_dir_all(parent)?;
        ensure_beneath_root(&root, parent)?;
        let mut output = File::create(&output_path)?;
        let copied = std::io::copy(&mut entry, &mut output)?;
        if copied > limits.max_uncompressed_bytes {
            return Err(ExtraError::ArchiveLimit("single entry byte count"));
        }
    }
    Ok(())
}

fn validate_relative_path(path: &Path) -> Result<()> {
    if path.as_os_str().is_empty()
        || path.components().any(|component| {
            matches!(
                component,
                Component::ParentDir | Component::RootDir | Component::Prefix(_)
            )
        })
    {
        return Err(ExtraError::UnsafeArchivePath(
            path.to_string_lossy().into_owned(),
        ));
    }
    Ok(())
}

fn ensure_beneath_root(root: &Path, path: &Path) -> Result<()> {
    let canonical = path.canonicalize()?;
    if !canonical.starts_with(root) {
        return Err(ExtraError::UnsafeArchivePath(
            path.to_string_lossy().into_owned(),
        ));
    }
    Ok(())
}
