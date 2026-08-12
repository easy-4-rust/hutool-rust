//! ImgUtil / ColorUtil parity（feature `img`）
//!
//! 对齐: `cn.hutool.core.img.ImgUtil` / `ColorUtil`
//!
//! ```text
//! cargo test -p hutool-core --features img --test img_util_parity
//! ```

#![cfg(feature = "img")]

use hutool_core::img::{color_util::ColorUtil, img_util::ImgUtil};
use image::GenericImageView;
use tempfile::tempdir;

/// 对齐 Java: `ImgUtilTest.scaleTest` / `cutTest` / `sliceByRowsAndColsTest`
#[test]
fn scale_cut_slice_roundtrip() {
    let img = ImgUtil::create_image(20, 10, 10, 20, 30);
    let scaled = ImgUtil::scale(&img, 5, 5);
    assert_eq!(scaled.dimensions(), (5, 5));
    let by = ImgUtil::scale_by(&img, 0.5);
    assert_eq!(by.dimensions(), (10, 5));
    let cut = ImgUtil::cut(&img, 2, 1, 4, 3).unwrap();
    assert_eq!(cut.dimensions(), (4, 3));
    let tiles = ImgUtil::slice(&img, 10, 5);
    assert!(tiles.len() >= 2);
    let grid = ImgUtil::slice_by_rows_and_cols(&img, 2, 2);
    assert!(!grid.is_empty());
    assert_eq!(ImgUtil::rotate(&img, 90).dimensions(), (10, 20));
    assert_eq!(ImgUtil::flip(&img).dimensions(), (20, 10));
}

/// 对齐 Java: `ImgUtilTest.writeTest` / `toBase64` / `convert`
#[test]
fn write_base64_convert() {
    let dir = tempdir().unwrap();
    let img = ImgUtil::create_image(8, 8, 255, 0, 0);
    let path = dir.path().join("out.png");
    ImgUtil::write_png(&img, &path).unwrap();
    let back = ImgUtil::read(&path).unwrap();
    assert_eq!(back.dimensions(), (8, 8));
    let b64 = ImgUtil::to_base64(&img, "png").unwrap();
    let from_b64 = ImgUtil::from_base64(&b64).unwrap();
    assert_eq!(from_b64.dimensions(), (8, 8));
    let uri = ImgUtil::to_base64_data_uri(&img, "png").unwrap();
    assert!(uri.starts_with("data:image/png;base64,"));
    let png = ImgUtil::to_bytes(&img, "png").unwrap();
    let jpg = ImgUtil::convert(&png, "jpeg").unwrap();
    assert!(!jpg.is_empty());
}

/// 对齐 Java: `ColorUtil.toHex` / distance / mainColor
#[test]
fn color_util_hex_distance() {
    assert_eq!(ColorUtil::to_hex(10, 20, 30), "#0A141E");
    assert_eq!(ColorUtil::hex_to_color("#0A141E"), Some((10, 20, 30)));
    assert!(ColorUtil::compute_color_distance((0, 0, 0), (255, 0, 0)) > 250.0);
    assert!(ColorUtil::max_distance() > 400.0);
    assert_eq!(
        ColorUtil::get_main_color(&[(10, 20, 30), (30, 20, 10)]),
        Some((20, 20, 20))
    );
    let _ = ColorUtil::random_color();
}

/// 对齐 Java: `ImgUtil.backgroundRemoval` / `getMainColor`
#[test]
fn background_and_main_color() {
    let img = ImgUtil::create_image(4, 4, 255, 255, 255);
    let cleared = ImgUtil::background_removal(&img, (255, 255, 255), 5.0);
    let rgba = cleared.to_rgba8();
    assert_eq!(rgba.get_pixel(0, 0).0[3], 0);
    let main = ImgUtil::get_main_color(&ImgUtil::create_image(2, 2, 10, 20, 30));
    assert_eq!(main, (10, 20, 30));
}
