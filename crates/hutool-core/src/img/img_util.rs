//! 对齐: `cn.hutool.core.img.ImgUtil`
//! 来源: hutool-core/src/main/java/cn/hutool/core/img/ImgUtil.java
//!
//! 基于 `image` crate 的字节流语义（feature `img`）；AWT/`BufferedImage` 无直接等价物标为 planned。

use std::io::Cursor;
use std::path::Path;

use base64::{Engine as _, engine::general_purpose::STANDARD as B64};
use image::{DynamicImage, GenericImageView, ImageFormat, ImageReader, imageops::FilterType};

use crate::{CoreError, Result};

/// 对齐 Java 类: `cn.hutool.core.img.ImgUtil`
#[derive(Debug, Clone, Copy, Default)]
pub struct ImgUtil;

impl ImgUtil {
    /// Hutool `IMAGE_TYPE_PNG`
    pub const IMAGE_TYPE_PNG: &'static str = "png";
    /// Hutool `IMAGE_TYPE_JPG`
    pub const IMAGE_TYPE_JPG: &'static str = "jpg";
    /// Hutool `IMAGE_TYPE_JPEG`
    pub const IMAGE_TYPE_JPEG: &'static str = "jpeg";

    /// 对齐 Java: `ImgUtil.read(File)` — 解码为像素缓冲。
    pub fn read(path: &Path) -> Result<DynamicImage> {
        let bytes = std::fs::read(path)?;
        Self::read_bytes(&bytes)
    }

    /// 从字节解码图像。
    pub fn read_bytes(bytes: &[u8]) -> Result<DynamicImage> {
        let reader = ImageReader::new(Cursor::new(bytes)).with_guessed_format()?;
        reader
            .decode()
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e.to_string()))
            .map_err(CoreError::from)
    }

    /// 对齐 Java: `ImgUtil.write(Image, File)` — 按扩展名写盘。
    pub fn write(image: &DynamicImage, path: &Path) -> Result<()> {
        let format = format_from_path(path).unwrap_or(ImageFormat::Png);
        image
            .save_with_format(path, format)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;
        Ok(())
    }

    /// 对齐 Java: `ImgUtil.writePng`
    pub fn write_png(image: &DynamicImage, path: &Path) -> Result<()> {
        image
            .save_with_format(path, ImageFormat::Png)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;
        Ok(())
    }

    /// 对齐 Java: `ImgUtil.writeJpg`
    pub fn write_jpg(image: &DynamicImage, path: &Path) -> Result<()> {
        image
            .save_with_format(path, ImageFormat::Jpeg)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;
        Ok(())
    }

    /// 对齐 Java: `ImgUtil.scale(Image, int, int)` — 精确缩放。
    pub fn scale(image: &DynamicImage, width: u32, height: u32) -> DynamicImage {
        image.resize_exact(width.max(1), height.max(1), FilterType::Lanczos3)
    }

    /// 对齐 Java: `ImgUtil.scale(Image, float)` — 按比例缩放。
    pub fn scale_by(image: &DynamicImage, scale: f32) -> DynamicImage {
        let scale = if scale.is_finite() && scale > 0.0 {
            scale
        } else {
            1.0
        };
        let w = ((f64::from(image.width()) * f64::from(scale)).round() as u32).max(1);
        let h = ((f64::from(image.height()) * f64::from(scale)).round() as u32).max(1);
        Self::scale(image, w, h)
    }

    /// 对齐 Java: `ImgUtil.cut(Image, Rectangle)` 矩形裁剪。
    pub fn cut(image: &DynamicImage, x: u32, y: u32, width: u32, height: u32) -> Result<DynamicImage> {
        let w = width.max(1);
        let h = height.max(1);
        if x.saturating_add(w) > image.width() || y.saturating_add(h) > image.height() {
            return Err(CoreError::InvalidArgument {
                name: "cut",
                reason: "rectangle out of bounds",
            });
        }
        Ok(image.crop_imm(x, y, w, h))
    }

    /// 对齐 Java: `ImgUtil.slice(Image, int, int)` — 按固定瓦片宽高切片。
    pub fn slice(image: &DynamicImage, tile_width: u32, tile_height: u32) -> Vec<DynamicImage> {
        let tw = tile_width.max(1);
        let th = tile_height.max(1);
        let mut tiles = Vec::new();
        let mut y = 0u32;
        while y < image.height() {
            let mut x = 0u32;
            while x < image.width() {
                let w = tw.min(image.width() - x);
                let h = th.min(image.height() - y);
                if let Ok(tile) = Self::cut(image, x, y, w, h) {
                    tiles.push(tile);
                }
                x = x.saturating_add(tw);
            }
            y = y.saturating_add(th);
        }
        tiles
    }

    /// 对齐 Java: `ImgUtil.sliceByRowsAndCols(Image, int, int)`。
    pub fn slice_by_rows_and_cols(
        image: &DynamicImage,
        rows: u32,
        cols: u32,
    ) -> Vec<DynamicImage> {
        let rows = rows.max(1);
        let cols = cols.max(1);
        let tile_w = (image.width() / cols).max(1);
        let tile_h = (image.height() / rows).max(1);
        Self::slice(image, tile_w, tile_h)
    }

    /// 对齐 Java: `ImgUtil.rotate(Image, int)`
    pub fn rotate(image: &DynamicImage, degree: i32) -> DynamicImage {
        let normalized = ((degree % 360) + 360) % 360;
        match normalized {
            90 => image.rotate90(),
            180 => image.rotate180(),
            270 => image.rotate270(),
            _ => image.clone(),
        }
    }

    /// 对齐 Java: `ImgUtil.flip(Image)` — 水平翻转。
    pub fn flip(image: &DynamicImage) -> DynamicImage {
        image.fliph()
    }

    /// 对齐 Java: `ImgUtil.gray(Image)`
    pub fn gray(image: &DynamicImage) -> DynamicImage {
        DynamicImage::ImageLuma8(image.to_luma8())
    }

    /// 对齐 Java: `ImgUtil.binary(Image)` — 简单阈值二值化。
    pub fn binary(image: &DynamicImage) -> DynamicImage {
        let gray = image.to_luma8();
        let mut out = gray.clone();
        for p in out.pixels_mut() {
            p.0[0] = if p.0[0] > 127 { 255 } else { 0 };
        }
        DynamicImage::ImageLuma8(out)
    }

    /// 对齐 Java: `ImgUtil.convert` — 重新编码格式。
    pub fn convert(bytes: &[u8], format_name: &str) -> Result<Vec<u8>> {
        let image = Self::read_bytes(bytes)?;
        Self::to_bytes(&image, format_name)
    }

    /// 对齐 Java: `ImgUtil.compress` — JPEG 质量重编码。
    pub fn compress(bytes: &[u8], quality: u8) -> Result<Vec<u8>> {
        let image = Self::read_bytes(bytes)?;
        let q = quality.clamp(1, 100);
        let mut out = Vec::new();
        let mut encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut out, q);
        encoder
            .encode_image(&image)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;
        Ok(out)
    }

    /// 对齐 Java: `ImgUtil.toBytes`
    pub fn to_bytes(image: &DynamicImage, format_name: &str) -> Result<Vec<u8>> {
        let format = parse_format(format_name)?;
        let mut out = Vec::new();
        image
            .write_to(&mut Cursor::new(&mut out), format)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;
        Ok(out)
    }

    /// 对齐 Java: `ImgUtil.toBase64`
    pub fn to_base64(image: &DynamicImage, format_name: &str) -> Result<String> {
        Ok(B64.encode(Self::to_bytes(image, format_name)?))
    }

    /// 对齐 Java: `ImgUtil.toBase64DataUri`
    pub fn to_base64_data_uri(image: &DynamicImage, format_name: &str) -> Result<String> {
        let mime = match format_name.trim().to_ascii_lowercase().as_str() {
            "jpg" | "jpeg" => "image/jpeg",
            "webp" => "image/webp",
            _ => "image/png",
        };
        Ok(format!(
            "data:{mime};base64,{}",
            Self::to_base64(image, format_name)?
        ))
    }

    /// 对齐 Java: `ImgUtil.toImage(String base64)`
    pub fn from_base64(base64: &str) -> Result<DynamicImage> {
        let raw = base64
            .strip_prefix("data:")
            .and_then(|s| s.split(',').nth(1))
            .unwrap_or(base64);
        let bytes = B64
            .decode(raw.trim())
            .map_err(|_| CoreError::InvalidArgument {
                name: "base64",
                reason: "invalid base64 image",
            })?;
        Self::read_bytes(&bytes)
    }

    /// 对齐 Java: `ImgUtil.copyImage`
    pub fn copy_image(image: &DynamicImage) -> DynamicImage {
        image.clone()
    }

    /// 创建纯色 RGB 图。
    pub fn create_image(width: u32, height: u32, r: u8, g: u8, b: u8) -> DynamicImage {
        DynamicImage::ImageRgb8(image::ImageBuffer::from_pixel(
            width.max(1),
            height.max(1),
            image::Rgb([r, g, b]),
        ))
    }

    /// 对齐 Java: `ImgUtil.createTransparentImage`
    pub fn create_transparent_image(width: u32, height: u32) -> DynamicImage {
        DynamicImage::ImageRgba8(image::ImageBuffer::from_pixel(
            width.max(1),
            height.max(1),
            image::Rgba([0, 0, 0, 0]),
        ))
    }

    /// 读取宽高。
    pub fn read_size(bytes: &[u8]) -> Result<(u32, u32)> {
        let image = Self::read_bytes(bytes)?;
        Ok(image.dimensions())
    }

    /// 对齐 Java: `ImgUtil.toHex(Color)` 简化。
    pub fn to_hex(r: u8, g: u8, b: u8) -> String {
        format!("#{r:02X}{g:02X}{b:02X}")
    }

    /// 对齐 Java: `ImgUtil.getColor(String)` — `#RRGGBB`。
    pub fn get_color(color: &str) -> Option<(u8, u8, u8)> {
        let s = color.trim().trim_start_matches('#');
        if s.len() != 6 {
            return None;
        }
        let r = u8::from_str_radix(&s[0..2], 16).ok()?;
        let g = u8::from_str_radix(&s[2..4], 16).ok()?;
        let b = u8::from_str_radix(&s[4..6], 16).ok()?;
        Some((r, g, b))
    }

    /// 对齐 Java: `ImgUtil.randomColor()`
    pub fn random_color() -> (u8, u8, u8) {
        use rand::RngExt;
        let mut rng = rand::rng();
        (rng.random(), rng.random(), rng.random())
    }

    /// 对齐 Java: `ImgUtil.hexToColor(String)`
    pub fn hex_to_color(color: &str) -> Option<(u8, u8, u8)> {
        Self::get_color(color)
    }

    /// 对齐 Java: `ImgUtil.getMainColor` — 采样均值。
    pub fn get_main_color(image: &DynamicImage) -> (u8, u8, u8) {
        let rgba = image.to_rgba8();
        let mut r = 0u64;
        let mut g = 0u64;
        let mut b = 0u64;
        let n = rgba.pixels().len().max(1) as u64;
        for p in rgba.pixels() {
            r += u64::from(p.0[0]);
            g += u64::from(p.0[1]);
            b += u64::from(p.0[2]);
        }
        ((r / n) as u8, (g / n) as u8, (b / n) as u8)
    }

    /// 对齐 Java: `ImgUtil.backgroundRemoval` — 近色透明化。
    pub fn background_removal(
        image: &DynamicImage,
        target: (u8, u8, u8),
        tolerance: f64,
    ) -> DynamicImage {
        let mut rgba = image.to_rgba8();
        for p in rgba.pixels_mut() {
            let dist = {
                let dr = f64::from(p.0[0] as i16 - target.0 as i16);
                let dg = f64::from(p.0[1] as i16 - target.1 as i16);
                let db = f64::from(p.0[2] as i16 - target.2 as i16);
                (dr * dr + dg * dg + db * db).sqrt()
            };
            if dist <= tolerance {
                p.0[3] = 0;
            }
        }
        DynamicImage::ImageRgba8(rgba)
    }
}

fn format_from_path(path: &Path) -> Option<ImageFormat> {
    path.extension()
        .and_then(|e| e.to_str())
        .and_then(|ext| match ext.to_ascii_lowercase().as_str() {
            "png" => Some(ImageFormat::Png),
            "jpg" | "jpeg" => Some(ImageFormat::Jpeg),
            "webp" => Some(ImageFormat::WebP),
            "gif" => Some(ImageFormat::Gif),
            _ => None,
        })
}

fn parse_format(name: &str) -> Result<ImageFormat> {
    match name.trim().to_ascii_lowercase().as_str() {
        "png" => Ok(ImageFormat::Png),
        "jpg" | "jpeg" => Ok(ImageFormat::Jpeg),
        "webp" => Ok(ImageFormat::WebP),
        "gif" => Ok(ImageFormat::Gif),
        _ => Err(CoreError::InvalidArgument {
            name: "format",
            reason: "unsupported image format",
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scale_cut_rotate_roundtrip() {
        let img = ImgUtil::create_image(20, 10, 10, 20, 30);
        let scaled = ImgUtil::scale(&img, 5, 5);
        assert_eq!(scaled.dimensions(), (5, 5));
        let cut = ImgUtil::cut(&img, 2, 1, 4, 3).unwrap();
        assert_eq!(cut.dimensions(), (4, 3));
        assert_eq!(ImgUtil::rotate(&img, 90).dimensions(), (10, 20));
        let b64 = ImgUtil::to_base64(&img, "png").unwrap();
        let back = ImgUtil::from_base64(&b64).unwrap();
        assert_eq!(back.dimensions(), (20, 10));
        assert_eq!(ImgUtil::get_color("#aabbcc"), Some((0xaa, 0xbb, 0xcc)));
    }
}
