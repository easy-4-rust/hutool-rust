//! ZIP creation and bounded, path-safe extraction.
//!
//! 对齐: `cn.hutool.extra.compress.CompressUtil`
//! 对齐: `cn.hutool.core.util.ZipUtil`（extra 侧安全 ZIP 子集）

use crate::Result;
use std::{io::{Read, Seek}, path::Path};

use super::extraction_limits::ExtractionLimits;

/// Hutool `CompressUtil` — ZIP archiver/extractor factory over safe helpers.
///
/// 对齐 Java 类: `cn.hutool.extra.compress.CompressUtil`
///
/// 7z/tar engines remain planned; ZIP maps to [`create_zip`] / [`extract_zip`].
pub struct CompressUtil;

impl CompressUtil {
    /// Creates a ZIP byte archive (Hutool `createArchiver` ZIP path).
    pub fn create_archiver(entries: &[(&str, &[u8])]) -> Result<Vec<u8>> {
        create_zip(entries)
    }

    /// Extracts a ZIP beneath `destination` (Hutool `createExtractor` ZIP path).
    pub fn create_extractor<R: Read + Seek>(
        reader: R,
        destination: impl AsRef<Path>,
        limits: ExtractionLimits,
    ) -> Result<()> {
        extract_zip(reader, destination, limits)
    }
}

use super::{create_zip, extract_zip};
