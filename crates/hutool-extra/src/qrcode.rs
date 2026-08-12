//! QR code SVG/PNG/ASCII rendering with Hutool-named facades.
//!
//! 对齐: `cn.hutool.extra.qrcode.QrCodeUtil`
//! 对齐: `cn.hutool.extra.qrcode.QrConfig`
//! 来源: hutool-extra/src/main/java/cn/hutool/extra/qrcode/

use crate::Result;
use ::qrcode::{EcLevel, QrCode, render::svg};

/// QR error-correction strength.
///
/// 对齐 Java: `com.google.zxing.qrcode.decoder.ErrorCorrectionLevel`
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ErrorCorrection {
    /// About 7% restoration capability (`L`).
    Low,
    /// About 15% restoration capability (`M`).
    #[default]
    Medium,
    /// About 25% restoration capability (`Q`).
    Quartile,
    /// About 30% restoration capability (`H`).
    High,
}

impl ErrorCorrection {
    fn to_ec_level(self) -> EcLevel {
        match self {
            Self::Low => EcLevel::L,
            Self::Medium => EcLevel::M,
            Self::Quartile => EcLevel::Q,
            Self::High => EcLevel::H,
        }
    }
}

/// Hutool `QrConfig` — size and error-correction for generation helpers.
///
/// 对齐 Java 类: `cn.hutool.extra.qrcode.QrConfig`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QrConfig {
    /// Width in pixels (SVG min dimension).
    pub width: u32,
    /// Height in pixels (SVG uses `max(width, height)` for square modules).
    pub height: u32,
    /// Error correction level.
    pub error_correction: ErrorCorrection,
    /// Foreground CSS color (SVG dark modules).
    pub fore_color: String,
    /// Background CSS color (SVG light modules).
    pub back_color: String,
}

impl Default for QrConfig {
    fn default() -> Self {
        Self::new(300, 300)
    }
}

impl QrConfig {
    /// Creates a config with Hutool's default 300×300 size (Hutool `QrConfig.create()`).
    #[must_use]
    pub fn create() -> Self {
        Self::default()
    }

    /// Creates a config with explicit dimensions (Hutool `QrConfig(width, height)`).
    #[must_use]
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width: width.max(1),
            height: height.max(1),
            error_correction: ErrorCorrection::Medium,
            fore_color: "#000000".into(),
            back_color: "#ffffff".into(),
        }
    }

    /// Sets width (Hutool `setWidth`).
    #[must_use]
    pub fn set_width(mut self, width: u32) -> Self {
        self.width = width.max(1);
        self
    }

    /// Sets height (Hutool `setHeight`).
    #[must_use]
    pub fn set_height(mut self, height: u32) -> Self {
        self.height = height.max(1);
        self
    }

    /// Sets error correction (Hutool `setErrorCorrection`).
    #[must_use]
    pub fn set_error_correction(mut self, level: ErrorCorrection) -> Self {
        self.error_correction = level;
        self
    }

    /// Sets foreground color hex (Hutool `setForeColor`).
    #[must_use]
    pub fn set_fore_color(mut self, color: impl Into<String>) -> Self {
        self.fore_color = color.into();
        self
    }

    /// Sets background color hex (Hutool `setBackColor`).
    #[must_use]
    pub fn set_back_color(mut self, color: impl Into<String>) -> Self {
        self.back_color = color.into();
        self
    }

    fn min_dimension(&self) -> u32 {
        self.width.min(self.height).max(1)
    }
}

/// Renders data as a standalone SVG document.
pub fn to_svg(data: impl AsRef<[u8]>, minimum_width: u32) -> Result<String> {
    to_svg_with_level(data, minimum_width, ErrorCorrection::Medium)
}

/// Renders data as SVG with an explicit correction level.
pub fn to_svg_with_level(
    data: impl AsRef<[u8]>,
    minimum_width: u32,
    level: ErrorCorrection,
) -> Result<String> {
    let config = QrConfig::new(minimum_width, minimum_width).set_error_correction(level);
    render_svg(data.as_ref(), &config)
}

fn render_svg(data: &[u8], config: &QrConfig) -> Result<String> {
    let code = QrCode::with_error_correction_level(data, config.error_correction.to_ec_level())?;
    Ok(code
        .render::<svg::Color<'_>>()
        .min_dimensions(config.min_dimension(), config.min_dimension())
        .dark_color(svg::Color(&config.fore_color))
        .light_color(svg::Color(&config.back_color))
        .build())
}

/// Renders a compact Unicode ASCII-art QR (Hutool `generateAsAsciiArt` subset).
pub fn to_ascii_art(data: impl AsRef<[u8]>) -> Result<String> {
    to_ascii_art_with_level(data, ErrorCorrection::Medium)
}

/// Renders ASCII-art with an explicit correction level.
pub fn to_ascii_art_with_level(data: impl AsRef<[u8]>, level: ErrorCorrection) -> Result<String> {
    let code = QrCode::with_error_correction_level(data.as_ref(), level.to_ec_level())?;
    Ok(code
        .render::<char>()
        .quiet_zone(false)
        .module_dimensions(1, 1)
        .build())
}

/// Encodes a PNG QR when the `image` feature is enabled.
#[cfg(feature = "image")]
pub fn to_png(data: impl AsRef<[u8]>, width: u32, height: u32) -> Result<Vec<u8>> {
    to_png_with_config(data, &QrConfig::new(width, height))
}

/// Encodes PNG using [`QrConfig`] (Hutool `generatePng`).
#[cfg(feature = "image")]
pub fn to_png_with_config(data: impl AsRef<[u8]>, config: &QrConfig) -> Result<Vec<u8>> {
    use ::image::{ImageBuffer, ImageFormat, Luma};
    use std::io::Cursor;

    let code =
        QrCode::with_error_correction_level(data.as_ref(), config.error_correction.to_ec_level())?;
    let colors = code.to_colors();
    let modules = code.width();
    let dim = config.min_dimension().max(modules as u32);
    let scale = (dim / modules as u32).max(1);
    let side = modules as u32 * scale;
    let mut buffer = ImageBuffer::from_pixel(side, side, Luma([255_u8]));
    for y in 0..modules {
        for x in 0..modules {
            if colors[y * modules + x] == ::qrcode::Color::Dark {
                for dy in 0..scale {
                    for dx in 0..scale {
                        buffer.put_pixel(x as u32 * scale + dx, y as u32 * scale + dy, Luma([0]));
                    }
                }
            }
        }
    }
    let mut bytes = Vec::new();
    ::image::DynamicImage::ImageLuma8(buffer)
        .write_to(&mut Cursor::new(&mut bytes), ImageFormat::Png)?;
    Ok(bytes)
}

/// Hutool `QrCodeUtil` facade over SVG/ASCII/(optional) PNG helpers.
///
/// 对齐 Java 类: `cn.hutool.extra.qrcode.QrCodeUtil`
pub struct QrCodeUtil;

impl QrCodeUtil {
    /// SVG target type constant (Hutool `QR_TYPE_SVG`).
    pub const QR_TYPE_SVG: &'static str = "svg";
    /// ASCII-art target type constant (Hutool `QR_TYPE_TXT`).
    pub const QR_TYPE_TXT: &'static str = "txt";

    /// Generates SVG (Hutool `generateAsSvg`).
    pub fn generate_as_svg(content: impl AsRef<[u8]>, config: &QrConfig) -> Result<String> {
        render_svg(content.as_ref(), config)
    }

    /// Generates SVG with default 300×300 config (Hutool overload).
    pub fn generate_as_svg_default(content: impl AsRef<[u8]>) -> Result<String> {
        Self::generate_as_svg(content, &QrConfig::create())
    }

    /// Generates ASCII art (Hutool `generateAsAsciiArt`).
    pub fn generate_as_ascii_art(content: impl AsRef<[u8]>) -> Result<String> {
        to_ascii_art(content)
    }

    /// Generates ASCII art with config-driven error correction.
    pub fn generate_as_ascii_art_with_config(
        content: impl AsRef<[u8]>,
        config: &QrConfig,
    ) -> Result<String> {
        to_ascii_art_with_level(content, config.error_correction)
    }

    /// Writes SVG to a file path (Hutool `generate(..., File)` SVG path).
    pub fn generate_to_file(
        content: impl AsRef<[u8]>,
        config: &QrConfig,
        path: impl AsRef<std::path::Path>,
    ) -> Result<()> {
        let svg = Self::generate_as_svg(content, config)?;
        std::fs::write(path, svg)?;
        Ok(())
    }

    /// Generates PNG bytes (Hutool `generatePng`) when `image` is enabled.
    #[cfg(feature = "image")]
    pub fn generate_png(content: impl AsRef<[u8]>, width: u32, height: u32) -> Result<Vec<u8>> {
        to_png(content, width, height)
    }

    /// Generates PNG from config (Hutool `generatePng(content, config)`).
    #[cfg(feature = "image")]
    pub fn generate_png_with_config(
        content: impl AsRef<[u8]>,
        config: &QrConfig,
    ) -> Result<Vec<u8>> {
        to_png_with_config(content, config)
    }

    /// Base64 data-URL for SVG (Hutool `generateAsBase64` SVG subset).
    pub fn generate_as_base64_svg(content: impl AsRef<[u8]>, config: &QrConfig) -> Result<String> {
        let svg = Self::generate_as_svg(content, config)?;
        Ok(format!(
            "data:image/svg+xml;base64,{}",
            base64_encode(svg.as_bytes())
        ))
    }
}

/// RFC 4648 base64 encoder for SVG data URLs (avoids a new dependency).
fn base64_encode(input: &[u8]) -> String {
    const TABLE: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::with_capacity(input.len().div_ceil(3) * 4);
    for chunk in input.chunks(3) {
        let mut buf = [0_u8; 3];
        for (i, b) in chunk.iter().enumerate() {
            buf[i] = *b;
        }
        let n = chunk.len();
        let x = (u32::from(buf[0]) << 16) | (u32::from(buf[1]) << 8) | u32::from(buf[2]);
        out.push(TABLE[((x >> 18) & 63) as usize] as char);
        out.push(TABLE[((x >> 12) & 63) as usize] as char);
        out.push(if n > 1 {
            TABLE[((x >> 6) & 63) as usize] as char
        } else {
            '='
        });
        out.push(if n > 2 {
            TABLE[(x & 63) as usize] as char
        } else {
            '='
        });
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_standalone_svg() {
        let svg = to_svg("https://example.com", 256).unwrap();
        assert!(svg.starts_with("<?xml"));
        assert!(svg.contains("<svg"));
        assert!(svg.contains("width=\""));
    }

    #[test]
    fn qr_code_util_svg_ascii_and_config() {
        let config = QrConfig::create()
            .set_width(128)
            .set_height(128)
            .set_error_correction(ErrorCorrection::High);
        let svg = QrCodeUtil::generate_as_svg("hutool", &config).unwrap();
        assert!(svg.contains("<svg"));
        let art = QrCodeUtil::generate_as_ascii_art("hutool").unwrap();
        assert!(art.len() > 16);
        let data_url = QrCodeUtil::generate_as_base64_svg("hutool", &config).unwrap();
        assert!(data_url.starts_with("data:image/svg+xml;base64,"));
    }

    #[cfg(feature = "image")]
    #[test]
    fn generates_png_bytes() {
        let png = QrCodeUtil::generate_png("png-qr", 128, 128).unwrap();
        assert!(png.starts_with(&[0x89, b'P', b'N', b'G']));
    }
}
