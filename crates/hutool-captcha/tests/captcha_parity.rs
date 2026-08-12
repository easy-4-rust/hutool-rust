//! Captcha parity tests —— 对齐 Hutool `hutool-captcha` 测试。
//!
//! 对齐: `cn.hutool.captcha.CaptchaTest`
//! 对齐: `cn.hutool.captcha.CaptchaUtilTest`
//! 对齐: `cn.hutool.captcha.GeneratorTest`
//! 对齐: `cn.hutool.captcha.GifCaptchaUtilTest`
//! 对齐: `cn.hutool.captcha.ShearCaptchaTest`
//!
//! 来源:
//! - hutool-captcha/src/test/java/cn/hutool/captcha/CaptchaTest.java
//! - hutool-captcha/src/test/java/cn/hutool/captcha/CaptchaUtilTest.java
//! - hutool-captcha/src/test/java/cn/hutool/captcha/GeneratorTest.java
//! - hutool-captcha/src/test/java/cn/hutool/captcha/GifCaptchaUtilTest.java
//! - hutool-captcha/src/test/java/cn/hutool/captcha/ShearCaptchaTest.java
//!
//! ## Image-diff notes（PNG/GIF 字节不对齐说明）
//!
//! Hutool 使用 AWT `BufferedImage` + 系统字体 + 随机干扰；hutool-captcha 使用
//! `image` crate + `font8x8` 位图字体。因此 **PNG/GIF 字节不可字节级对齐**。
//! 本文件对光栅用例统一断言：
//! - 编码魔数（PNG `\x89PNG` / GIF `GIF`）
//! - 宽高与 code 长度 / verify 语义
//! - `verify(code)` 与 generator 行为
//! Shear 的 `shear`/`shearX`/`shearY` 在 Rust 侧以干扰线近似（无 AWT 剪切变换），
//! 通过 `create_image` 结构等价覆盖私有方法路径。

use std::io::Cursor;
use std::sync::Arc;

use hc::CodeGenerator;
use hutool_captcha as hc;
use image::ImageReader;

/// 断言字节流可解码且宽高匹配（结构等价，非字节级对齐）。
fn assert_raster_dims(bytes: &[u8], expect_w: u32, expect_h: u32, kind: &str) {
    let img = ImageReader::new(Cursor::new(bytes))
        .with_guessed_format()
        .expect("guess image format")
        .decode()
        .unwrap_or_else(|e| panic!("{kind} decode failed: {e}"));
    assert_eq!(img.width(), expect_w, "{kind} width");
    assert_eq!(img.height(), expect_h, "{kind} height");
}

// ---------------------------------------------------------------------------
// Existing non-Hutool smoke tests (保留，勿删)
// ---------------------------------------------------------------------------

#[test]
fn alphanumeric_generator_test() {
    let generator = hc::AlphanumericGenerator::new(8);
    let code = generator.generate();
    assert_eq!(code.len(), 8, "AlphanumericGenerator 长度应为 8");
}

#[test]
fn captcha_challenge_generate_test() {
    let generator = hc::AlphanumericGenerator::new(4);
    let challenge = hc::CaptchaChallenge::generate(&generator, std::time::Duration::from_secs(300));
    assert!(!challenge.code().is_empty(), "captcha code 不应为空");
}

// ---------------------------------------------------------------------------
// CaptchaTest
// ---------------------------------------------------------------------------

/// 对齐 Java: `CaptchaTest.lineCaptchaTest1()`
#[test]
fn line_captcha_test1() {
    let mut line = hc::CaptchaUtil::create_line_captcha(200, 100).expect("create_line_captcha");
    let code = line.code().expect("code").to_owned();
    assert!(!code.is_empty(), "对齐 Java: getCode() not null/empty");
    assert!(line.verify(&code), "对齐 Java: verify(getCode()) 应为 true");
    let bytes = line.image_bytes().expect("png bytes");
    assert!(bytes.starts_with(b"\x89PNG"), "PNG magic");
    assert_raster_dims(bytes, 200, 100, "lineCaptchaTest1");
}

/// 对齐 Java: `CaptchaTest.lineCaptchaTest3()`（Java `@Disabled` 写盘用例）
///
/// 对齐说明: Java `setBackground(null)` → Rust 使用透明背景；写盘改为 temp 路径。
#[test]
fn line_captcha_test3() {
    let mut line =
        hc::CaptchaUtil::create_line_captcha_with_count(200, 70, 4, 15).expect("line 200x70");
    line.set_background(hc::CaptchaColor([0, 0, 0, 0]));
    let dir = std::env::temp_dir().join(format!("hutool-captcha-t3-{}", std::process::id()));
    std::fs::create_dir_all(&dir).expect("tmpdir");
    let path = dir.join("tellow.png");
    line.write_to_path(&path).expect("write");
    let bytes = std::fs::read(&path).expect("read");
    assert!(bytes.starts_with(b"\x89PNG"));
    assert_raster_dims(&bytes, 200, 70, "lineCaptchaTest3");
    let _ = std::fs::remove_dir_all(&dir);
}

/// 对齐 Java: `CaptchaTest.lineCaptchaTestWithSize()`（Java `@Disabled`）
#[test]
fn line_captcha_test_with_size() {
    let mut line =
        hc::CaptchaUtil::create_line_captcha_with_size(200, 70, 4, 15, 0.65).expect("with_size");
    line.set_background(hc::CaptchaColor([255, 255, 0, 255])); // Color.yellow
    let code = line.code().expect("code").to_owned();
    assert_eq!(code.chars().count(), 4);
    assert!(line.verify(&code));
    let bytes = line.image_bytes().expect("png");
    assert!(bytes.starts_with(b"\x89PNG"));
    assert_raster_dims(bytes, 200, 70, "lineCaptchaTestWithSize");
}

/// 对齐 Java: `CaptchaTest.lineCaptchaWithMathTest()`（Java `@Disabled`）
#[test]
fn line_captcha_with_math_test() {
    let mut line = hc::CaptchaUtil::create_line_captcha(200, 80).expect("line");
    line.set_generator(Arc::new(hc::MathGenerator::default()))
        .set_text_alpha(0.8);
    let code = line.code().expect("math code").to_owned();
    assert!(code.ends_with('='), "MathGenerator 表达式应以 '=' 结尾");
    let answer = hc::MathGenerator::evaluate(&code).expect("evaluate");
    assert!(
        line.verify(&answer.to_string()),
        "对齐 Java: MathGenerator verify(answer)"
    );
    let bytes = line.image_bytes().expect("png");
    assert!(bytes.starts_with(b"\x89PNG"));
    assert_raster_dims(bytes, 200, 80, "lineCaptchaWithMathTest");
}

/// 对齐 Java: `CaptchaTest.lineCaptchaTest2()`（Java `@Disabled` 写盘/二次 createCode）
#[test]
fn line_captcha_test2() {
    let mut line = hc::CaptchaUtil::create_line_captcha(200, 100).expect("line");
    let code1 = line.code().expect("code1").to_owned();
    // Java 对固定 "1234" 做 verify（通常为 false）；确保 API 可调用
    let _ = line.verify("1234");
    assert!(line.verify(&code1));
    let bytes1 = line.image_bytes().expect("png1").to_vec();
    assert!(bytes1.starts_with(b"\x89PNG"));

    line.create_code().expect("create_code again");
    let code2 = line.code().expect("code2").to_owned();
    assert!(line.verify(&code2));
    let bytes2 = line.image_bytes().expect("png2");
    assert!(bytes2.starts_with(b"\x89PNG"));
    assert_raster_dims(bytes2, 200, 100, "lineCaptchaTest2");
    // 二次生成后编码字节通常不同（随机干扰）；仅要求结构合法
    assert!(!bytes2.is_empty());
}

/// 对齐 Java: `CaptchaTest.circleCaptchaTest()`（Java `@Disabled`）
#[test]
fn circle_captcha_test() {
    let mut captcha =
        hc::CaptchaUtil::create_circle_captcha_with_count(200, 100, 4, 20).expect("circle");
    let code = captcha.code().expect("code").to_owned();
    assert_eq!(code.chars().count(), 4);
    let _ = captcha.verify("1234");
    assert!(captcha.verify(&code));
    let bytes = captcha.image_bytes().expect("png");
    assert!(bytes.starts_with(b"\x89PNG"));
    assert_raster_dims(bytes, 200, 100, "circleCaptchaTest");
}

/// 对齐 Java: `CaptchaTest.circleCaptchaTestWithSize()`（Java `@Disabled`）
#[test]
fn circle_captcha_test_with_size() {
    let mut captcha =
        hc::CaptchaUtil::create_circle_captcha_with_size(200, 70, 4, 15, 0.65).expect("size");
    captcha.set_background(hc::CaptchaColor([255, 255, 0, 255]));
    let code = captcha.code().expect("code").to_owned();
    assert_eq!(code.chars().count(), 4);
    assert!(captcha.verify(&code));
    assert_raster_dims(
        captcha.image_bytes().expect("png"),
        200,
        70,
        "circleCaptchaTestWithSize",
    );
}

/// 对齐 Java: `CaptchaTest.shearCaptchaTest()`（Java `@Disabled`）
#[test]
fn shear_captcha_test() {
    let mut captcha =
        hc::CaptchaUtil::create_shear_captcha_with_count(200, 100, 4, 4).expect("shear");
    let code = captcha.code().expect("code").to_owned();
    assert_eq!(code.chars().count(), 4);
    assert!(captcha.verify(&code));
    let bytes = captcha.image_bytes().expect("png");
    assert!(bytes.starts_with(b"\x89PNG"));
    assert_raster_dims(bytes, 200, 100, "shearCaptchaTest");
}

/// 对齐 Java: `CaptchaTest.shearCaptchaTest2()`（Java `@Disabled`，直接 `new ShearCaptcha`）
#[test]
fn shear_captcha_test2() {
    let mut captcha = hc::ShearCaptcha::with_code_count(200, 100, 4, 4).expect("new ShearCaptcha");
    let code = captcha.code().expect("code").to_owned();
    assert!(captcha.verify(&code));
    assert_raster_dims(
        captcha.image_bytes().expect("png"),
        200,
        100,
        "shearCaptchaTest2",
    );
}

/// 对齐 Java: `CaptchaTest.ShearCaptchaWithMathTest()`（Java `@Disabled`）
#[test]
fn shear_captcha_with_math_test() {
    let mut captcha =
        hc::CaptchaUtil::create_shear_captcha_with_count(200, 45, 4, 4).expect("shear math");
    captcha.set_generator(Arc::new(hc::MathGenerator::default()));
    let code = captcha.code().expect("code").to_owned();
    let answer = hc::MathGenerator::evaluate(&code).expect("evaluate");
    assert!(captcha.verify(&answer.to_string()));
    // Java 仍调用 verify("1234")；此处覆盖可调用性
    let _ = captcha.verify("1234");
    assert_raster_dims(
        captcha.image_bytes().expect("png"),
        200,
        45,
        "ShearCaptchaWithMathTest",
    );
}

/// 对齐 Java: `CaptchaTest.ShearCaptchaTestWithSize()`（Java `@Disabled`）
#[test]
fn shear_captcha_test_with_size() {
    let mut captcha =
        hc::CaptchaUtil::create_shear_captcha_with_size(200, 70, 4, 15, 0.65).expect("size");
    captcha.set_background(hc::CaptchaColor([255, 255, 0, 255]));
    let code = captcha.code().expect("code").to_owned();
    assert_eq!(code.chars().count(), 4);
    assert!(captcha.verify(&code));
    assert_raster_dims(
        captcha.image_bytes().expect("png"),
        200,
        70,
        "ShearCaptchaTestWithSize",
    );
}

/// 对齐 Java: `CaptchaTest.GifCaptchaTest()`（Java `@Disabled`）
#[test]
fn gif_captcha_test() {
    let mut captcha = hc::CaptchaUtil::create_gif_captcha_with_count(200, 100, 4).expect("gif");
    let code = captcha.code().expect("code").to_owned();
    assert!(
        captcha.verify(&code),
        "对齐 Java: assert captcha.verify(captcha.getCode())"
    );
    let bytes = captcha.image_bytes().expect("gif");
    assert!(bytes.starts_with(b"GIF"), "GIF magic（非 PNG）");
    assert_raster_dims(bytes, 200, 100, "GifCaptchaTest");
}

/// 对齐 Java: `CaptchaTest.GifCaptchaTestWithSize()`（Java `@Disabled`）
#[test]
fn gif_captcha_test_with_size() {
    let mut captcha =
        hc::CaptchaUtil::create_gif_captcha_with_size(200, 70, 4, 15, 0.65).expect("gif size");
    captcha.set_background(hc::CaptchaColor([255, 255, 0, 255]));
    let code = captcha.code().expect("code").to_owned();
    assert_eq!(code.chars().count(), 4);
    assert!(captcha.verify(&code));
    let bytes = captcha.image_bytes().expect("gif");
    assert!(bytes.starts_with(b"GIF"));
    assert_raster_dims(bytes, 200, 70, "GifCaptchaTestWithSize");
}

/// 对齐 Java: `CaptchaTest.bgTest()`（Java `@Disabled`）
#[test]
fn bg_test() {
    let mut captcha = hc::CaptchaUtil::create_line_captcha_with_count(200, 100, 4, 1).expect("bg");
    captcha.set_background(hc::CaptchaColor([255, 255, 255, 255])); // Color.WHITE
    let code = captcha.code().expect("code").to_owned();
    assert!(captcha.verify(&code));
    let bytes = captcha.image_bytes().expect("png");
    // 对齐说明: Java 写 `test.jpg`，Rust 统一输出 PNG；断言 PNG 结构等价
    assert!(bytes.starts_with(b"\x89PNG"));
    assert_raster_dims(bytes, 200, 100, "bgTest");
}

// ---------------------------------------------------------------------------
// CaptchaUtilTest
// ---------------------------------------------------------------------------

/// 对齐 Java: `CaptchaUtilTest.createTest()`（Java `@Disabled`）
#[test]
fn create_test() {
    for _ in 0..1 {
        let captcha = hc::CaptchaUtil::create_shear_captcha(320, 240).expect("createShearCaptcha");
        assert_eq!(captcha.create_image("ABCD").expect("img").width(), 320);
        assert_eq!(captcha.create_image("ABCD").expect("img").height(), 240);
    }
}

/// 对齐 Java: `CaptchaUtilTest.drawStringColourfulColorDistanceTest()`（Java `@Disabled`）
///
/// 对齐说明: Java 自定义 AbstractCaptcha + GraphicsUtil.drawStringColourful(色差 200)；
/// Rust 无 AWT GraphicsUtil，以 LineCaptcha 彩色文字渲染做结构等价覆盖。
#[test]
fn draw_string_colourful_color_distance_test() {
    for i in 0..10 {
        let mut line = hc::LineCaptcha::with_code_count(200, 100, 5, 10).expect("color distance");
        line.set_background(hc::CaptchaColor([255, 255, 255, 255]));
        let bytes = line.image_bytes().expect("png").to_vec();
        assert!(bytes.starts_with(b"\x89PNG"), "iteration {i}: PNG magic");
        assert_raster_dims(&bytes, 200, 100, "drawStringColourfulColorDistance");
        let code = line.code().expect("code").to_owned();
        assert_eq!(code.chars().count(), 5);
        assert!(line.verify(&code));
    }
}

/// 对齐 Java: `CaptchaUtilTest.drawStringColourfulDefaultColorDistanceTest()`（Java `@Disabled`）
#[test]
fn draw_string_colourful_default_color_distance_test() {
    for i in 0..10 {
        let mut line = hc::LineCaptcha::with_code_count(200, 100, 5, 10).expect("default distance");
        line.set_background(hc::CaptchaColor([255, 255, 255, 255]));
        let bytes = line.image_bytes().expect("png").to_vec();
        assert!(bytes.starts_with(b"\x89PNG"), "iteration {i}");
        assert_raster_dims(&bytes, 200, 100, "drawStringColourfulDefaultColorDistance");
    }
}

// ---------------------------------------------------------------------------
// GeneratorTest
// ---------------------------------------------------------------------------

/// 对齐 Java: `GeneratorTest.mathGeneratorTest()`
#[test]
fn math_generator_test() {
    let math = hc::MathGenerator::default();
    for _ in 0..1000 {
        let code = math.generate();
        // Java: verify(generate(), "0") — 仅确保不抛异常
        let _ = math.verify(&code, "0");
    }

    let math_non_neg = hc::MathGenerator::with_negative_results(false);
    for _ in 0..1000 {
        let code = math_non_neg.generate();
        let value = hc::MathGenerator::evaluate(&code).expect("evaluate");
        assert!(
            value >= 0,
            "对齐 Java: allowNegative=false 时 Calculator.conversion 不得为负: {code} → {value}"
        );
    }
}

// ---------------------------------------------------------------------------
// GifCaptchaUtilTest
// ---------------------------------------------------------------------------

fn new_gif_fixture() -> hc::GifCaptcha {
    hc::GifCaptcha::with_code_count(200, 100, 4, 10).expect("GifCaptcha fixture")
}

/// 对齐 Java: `GifCaptchaUtilTest.testSetQuality()`
#[test]
fn test_set_quality() {
    let captcha = new_gif_fixture().set_quality(20);
    assert_eq!(captcha.quality(), 20, "Quality 应该设置为 20");

    let captcha = captcha.set_quality(0); // <1 → 1
    assert_eq!(captcha.quality(), 1, "Quality 应该设置为 1，如果小于 1");
}

/// 对齐 Java: `GifCaptchaUtilTest.testSetRepeat()`
#[test]
fn test_set_repeat() {
    let captcha = new_gif_fixture().set_repeat(5);
    assert_eq!(captcha.repeat(), 5, "Repeat 应该设置为 5");

    let captcha = captcha.set_repeat(-1);
    assert_eq!(captcha.repeat(), 0, "Repeat 应该设置为 0，如果设置了负值");
}

/// 对齐 Java: `GifCaptchaUtilTest.testSetColorRange()`
#[test]
fn test_set_color_range() {
    let captcha = new_gif_fixture().set_min_color(100).set_max_color(200);
    assert_eq!(captcha.min_color(), 100, "Min color 应该设置为 100");
    assert_eq!(captcha.max_color(), 200, "Max color 应该设置为 200");
}

/// 对齐 Java: `GifCaptchaUtilTest.testCreateCode()`
#[test]
fn test_create_code() {
    let mut captcha = new_gif_fixture();
    captcha.create_code().expect("createCode");
    let image_bytes = captcha.image_bytes().expect("getImageBytes");
    assert!(!image_bytes.is_empty(), "生成的图片字节不应该为空");
    assert!(image_bytes.starts_with(b"GIF"), "应为有效 GIF");
    assert_raster_dims(image_bytes, 200, 100, "testCreateCode");
}

/// 对齐 Java: `GifCaptchaUtilTest.testGraphicsImage()`
///
/// 对齐说明: Java 反射调用私有 `graphicsImage`；Rust 通过 `create_image` 覆盖同路径。
#[test]
fn test_graphics_image() {
    let captcha = new_gif_fixture();
    let image = captcha.create_image("ABCD").expect("graphicsImage 等价");
    assert_eq!(image.width(), 200);
    assert_eq!(image.height(), 100);
}

/// 对齐 Java: `GifCaptchaUtilTest.testRandomColor()`
#[test]
fn test_random_color() {
    let color = hc::GifCaptcha::random_color(0, 255);
    let [r, g, b, a] = color.0;
    let _ = (r, g, b);
    assert_eq!(a, 255);
    // 多次采样均落在区间内
    for _ in 0..20 {
        let c = hc::GifCaptcha::random_color(10, 20);
        assert!((10..=20).contains(&c.0[0]));
        assert!((10..=20).contains(&c.0[1]));
        assert!((10..=20).contains(&c.0[2]));
    }
}

// ---------------------------------------------------------------------------
// ShearCaptchaTest
// ---------------------------------------------------------------------------

/// 对齐 Java: `ShearCaptchaTest.testConstructor()`
#[test]
fn test_constructor() {
    let captcha = hc::ShearCaptcha::new(200, 100).expect("ShearCaptcha");
    // 成功构造即对齐 assertNotNull
    let _ = captcha;
}

/// 对齐 Java: `ShearCaptchaTest.testCreateImage()`
#[test]
fn test_create_image() {
    let captcha = hc::ShearCaptcha::new(200, 100).expect("ShearCaptcha");
    let image = captcha.create_image("ABCD").expect("createImage");
    assert_eq!(image.width(), 200, "图像宽度应该为 200");
    assert_eq!(image.height(), 100, "图像高度应该为 100");
}

/// 对齐 Java: `ShearCaptchaTest.testDrawString()`
///
/// 对齐说明: Java 反射 `drawString` + RenderingHints；Rust 经 `create_image` 绘制字符，
/// 断言输出图像非空且尺寸正确（无 AWT 抗锯齿 hint API）。
#[test]
fn test_draw_string() {
    let captcha = hc::ShearCaptcha::new(200, 100).expect("ShearCaptcha");
    let image = captcha.create_image("ABCD").expect("drawString path");
    assert_eq!(image.width(), 200);
    assert_eq!(image.height(), 100);
    // 前景像素应存在（非纯背景）
    let rgba = image.to_rgba8();
    let non_white = rgba.pixels().any(|p| p.0 != [255, 255, 255, 255]);
    assert!(non_white, "绘制字符串后应有非背景像素");
}

/// 对齐 Java: `ShearCaptchaTest.testShear()`
///
/// 对齐说明: Java 反射 `shear(Graphics,...)`；Rust ShearCaptcha 以干扰线近似剪切，
/// 通过 create_image 结构等价覆盖（PNG 非字节对齐）。
#[test]
fn test_shear() {
    let captcha = hc::ShearCaptcha::new(200, 100).expect("ShearCaptcha");
    let image = captcha.create_image("SHEAR").expect("shear path");
    assert_eq!(image.width(), 200);
    assert_eq!(image.height(), 100);
}

/// 对齐 Java: `ShearCaptchaTest.testShearX()`
#[test]
fn test_shear_x() {
    let captcha = hc::ShearCaptcha::new(200, 100).expect("ShearCaptcha");
    let image = captcha.create_image("SHEARX").expect("shearX path");
    assert_eq!(image.width(), 200);
    assert_eq!(image.height(), 100);
}

/// 对齐 Java: `ShearCaptchaTest.testShearY()`
#[test]
fn test_shear_y() {
    let captcha = hc::ShearCaptcha::new(200, 100).expect("ShearCaptcha");
    let image = captcha.create_image("SHEARY").expect("shearY path");
    assert_eq!(image.width(), 200);
    assert_eq!(image.height(), 100);
}

/// 对齐 Java: `ShearCaptchaTest.testDrawInterfere()`
#[test]
fn test_draw_interfere() {
    let captcha = hc::ShearCaptcha::with_code_count(200, 100, 4, 4).expect("interfere");
    let image = captcha.create_image("ABCD").expect("drawInterfere path");
    assert_eq!(image.width(), 200);
    assert_eq!(image.height(), 100);
}

/// 对齐 Java: `ShearCaptchaTest.testDrawInterfereLines()`
#[test]
fn test_draw_interfere_lines() {
    let captcha = hc::ShearCaptcha::with_count(200, 100, 4).expect("with_count");
    let image = captcha.create_image("ABCD").expect("interfere lines");
    assert!(image.width() > 0, "生成的验证码图片不应该为空");
    assert_eq!(image.width(), 200);
    assert_eq!(image.height(), 100);
}

/// 对齐 Java: `ShearCaptchaTest.testCaptchaSize()`
#[test]
fn test_captcha_size() {
    let captcha = hc::ShearCaptcha::new(300, 150).expect("300x150");
    let image = captcha.create_image("XYZ").expect("createImage");
    assert_eq!(image.width(), 300, "图像宽度应该为 300");
    assert_eq!(image.height(), 150, "图像高度应该为 150");
}

/// 对齐 Java: `ShearCaptchaTest.testRandomGenerator()`
#[test]
fn test_random_generator() {
    let generator = hc::RandomGenerator::new(4).expect("RandomGenerator(4)");
    let code = generator.generate();
    assert!(!code.is_empty(), "生成的验证码字符不应该为 null/empty");
    assert_eq!(code.len(), 4, "验证码字符长度应该为 4");
}
