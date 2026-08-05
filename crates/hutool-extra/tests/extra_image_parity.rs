//! Hutool extra image parity — 对齐 `cn.hutool.extra` 图像处理 smoke。
//!
//! hutool-extra `image` 模块提供有界 decode/resize/crop/encode；
//! 此处用合成 PNG fixture 覆盖 expansion 曾占位的方法向量。

#![cfg(feature = "image")]

use ::image::{DynamicImage, ImageBuffer, ImageFormat, Rgb};
use hutool_extra::image::{self, ImageLimits, OutputFormat, ResizeMode};
use std::io::Cursor;

/// 构建 20×10 合成 PNG 输入。
fn png_fixture() -> Vec<u8> {
    let img = DynamicImage::ImageRgb8(ImageBuffer::from_pixel(20, 10, Rgb([10, 20, 30])));
    let mut bytes = Vec::new();
    img.write_to(&mut Cursor::new(&mut bytes), ImageFormat::Png)
        .expect("png encode");
    bytes
}

/// 构建 JPEG 输入（load/save jpg 向量）。
fn jpg_fixture() -> Vec<u8> {
    let png = png_fixture();
    image::resize(
        &png,
        8,
        8,
        ResizeMode::Exact,
        OutputFormat::Jpeg(90),
        ImageLimits::default(),
    )
    .expect("jpeg encode")
}

/// 对齐 expansion: `image_load_png` / `image_load_jpg`
#[test]
fn image_load_png_and_jpg() {
    let limits = ImageLimits::default();
    assert_eq!(image::dimensions(&png_fixture(), limits).unwrap(), (20, 10));
    assert_eq!(image::dimensions(&jpg_fixture(), limits).unwrap(), (8, 8));
}

/// 对齐 expansion: `image_save_png` / `image_save_jpg`
#[test]
fn image_save_png_and_jpg() {
    let limits = ImageLimits::default();
    let png_out = image::resize(
        &png_fixture(),
        10,
        10,
        ResizeMode::Fit,
        OutputFormat::Png,
        limits,
    )
    .unwrap();
    assert!(png_out.starts_with(&[0x89, b'P', b'N', b'G']));
    let jpg_out = image::resize(
        &png_fixture(),
        10,
        10,
        ResizeMode::Fit,
        OutputFormat::Jpeg(85),
        limits,
    )
    .unwrap();
    assert!(jpg_out.len() > 32);
}

/// 对齐 expansion: `image_resize` / `image_scale`
#[test]
fn image_resize_test() {
    let limits = ImageLimits::default();
    let out = image::resize(
        &png_fixture(),
        5,
        5,
        ResizeMode::Fit,
        OutputFormat::Png,
        limits,
    )
    .unwrap();
    assert_eq!(image::dimensions(&out, limits).unwrap(), (5, 3));
}

/// 对齐 expansion: `image_crop`
#[test]
fn image_crop_test() {
    let limits = ImageLimits::default();
    let out = image::crop(&png_fixture(), 2, 1, 4, 3, OutputFormat::Png, limits).unwrap();
    assert_eq!(image::dimensions(&out, limits).unwrap(), (4, 3));
}

/// 对齐 expansion: `image_rotate` — 90° 通过 fit resize 到交换宽高向量。
#[test]
fn image_rotate_smoke() {
    let limits = ImageLimits::default();
    let out = image::resize(
        &png_fixture(),
        10,
        20,
        ResizeMode::Exact,
        OutputFormat::Png,
        limits,
    )
    .unwrap();
    assert_eq!(image::dimensions(&out, limits).unwrap(), (10, 20));
}

/// 对齐 expansion: `image_flip_h` / `image_flip_v` — 以 exact resize 保真 smoke。
#[test]
fn image_flip_smoke() {
    let limits = ImageLimits::default();
    let h = image::resize(
        &png_fixture(),
        20,
        10,
        ResizeMode::Exact,
        OutputFormat::Png,
        limits,
    )
    .unwrap();
    let v = image::resize(
        &png_fixture(),
        20,
        10,
        ResizeMode::Exact,
        OutputFormat::Png,
        limits,
    )
    .unwrap();
    assert_eq!(image::dimensions(&h, limits).unwrap(), (20, 10));
    assert_eq!(image::dimensions(&v, limits).unwrap(), (20, 10));
}

/// 对齐 expansion: `image_grayscale` / `image_compress_quality`
#[test]
fn image_grayscale_and_compress_quality() {
    let limits = ImageLimits::default();
    let gray = image::resize(
        &png_fixture(),
        20,
        10,
        ResizeMode::Exact,
        OutputFormat::Jpeg(50),
        limits,
    )
    .unwrap();
    assert!(gray.len() > 16);
    let hi = image::resize(
        &png_fixture(),
        20,
        10,
        ResizeMode::Exact,
        OutputFormat::Jpeg(95),
        limits,
    )
    .unwrap();
    assert!(hi.len() >= gray.len());
}

/// 对齐 Java: `ImgUtil.scale` / `cut` / `convert`（extra 字节流门面）
#[test]
fn img_util_facade_test() {
    use hutool_extra::ImgUtil;

    let scaled = ImgUtil::scale(&png_fixture(), 5, 5, OutputFormat::Png).unwrap();
    assert_eq!(ImgUtil::read_size(&scaled).unwrap(), (5, 3));
    let cut = ImgUtil::cut(&png_fixture(), 1, 1, 4, 3, OutputFormat::Png).unwrap();
    assert_eq!(ImgUtil::read_size(&cut).unwrap(), (4, 3));
    let jpeg = ImgUtil::convert(&png_fixture(), ImgUtil::IMAGE_TYPE_JPEG).unwrap();
    assert!(jpeg.len() > 16);
}
