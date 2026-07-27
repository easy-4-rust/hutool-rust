//! Hutool-named raster CAPTCHA facade.
//!
//! 对齐: `cn.hutool.captcha` (Hutool 验证码核心实现)
//! 来源: hutool-captcha/src/main/java/cn/hutool/captcha/
//! 中文说明: Hutool 风格光栅验证码的共享绘图类型与辅助函数，供已拆分对象复用。

use image::{Rgba, RgbaImage};

/// RGBA color used by the compatibility renderer.
///
/// 对齐: Rust 扩展，对应 Hutool 中的颜色配置
/// 中文说明: 兼容渲染器使用的 RGBA 颜色
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CaptchaColor(pub [u8; 4]);

/// Bitmap font scale used by the compatibility renderer.
///
/// 对齐: Rust 扩展，对应 Hutool 中的字体配置
/// 中文说明: 兼容渲染器使用的位图字体缩放
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CaptchaFont {
    /// Integer scale applied to the embedded 8x8 glyphs.
    pub scale: u8,
}

/// Stroke width used for interference elements.
///
/// 对齐: Rust 扩展，对应 Hutool 中的干扰线配置
/// 中文说明: 干扰元素使用的笔触宽度
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CaptchaStroke {
    /// Width in pixels.
    pub width: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CaptchaKind {
    Line,
    Circle,
    Shear,
    Gif,
}

pub(crate) fn draw_glyph(
    image: &mut RgbaImage,
    bitmap: [u8; 8],
    x: u32,
    y: u32,
    scale: u32,
    color: Rgba<u8>,
) {
    for (row, bits) in bitmap.iter().enumerate() {
        for column in 0..8_u32 {
            if bits & (1 << column) == 0 {
                continue;
            }
            let row = u32::try_from(row).expect("bitmap has eight rows");
            for offset_y in 0..scale {
                for offset_x in 0..scale {
                    let pixel_x = x + column * scale + offset_x;
                    let pixel_y = y + row * scale + offset_y;
                    if pixel_x < image.width() && pixel_y < image.height() {
                        image.put_pixel(pixel_x, pixel_y, color);
                    }
                }
            }
        }
    }
}

pub(crate) fn draw_line(
    image: &mut RgbaImage,
    start: (i32, i32),
    end: (i32, i32),
    thickness: u8,
    color: Rgba<u8>,
) {
    let (mut x0, mut y0) = start;
    let (x1, y1) = end;
    let dx = (x1 - x0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let dy = -(y1 - y0).abs();
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut error = dx + dy;
    loop {
        for offset_y in 0..i32::from(thickness) {
            for offset_x in 0..i32::from(thickness) {
                put_pixel_checked(image, x0 + offset_x, y0 + offset_y, color);
            }
        }
        if x0 == x1 && y0 == y1 {
            break;
        }
        let twice = 2 * error;
        if twice >= dy {
            error += dy;
            x0 += sx;
        }
        if twice <= dx {
            error += dx;
            y0 += sy;
        }
    }
}

pub(crate) fn draw_circle(
    image: &mut RgbaImage,
    center_x: i32,
    center_y: i32,
    radius: i32,
    color: Rgba<u8>,
) {
    for offset_x in -radius..=radius {
        let target = radius * radius - offset_x * offset_x;
        let mut offset_y = 0;
        while (offset_y + 1) * (offset_y + 1) <= target {
            offset_y += 1;
        }
        put_pixel_checked(image, center_x + offset_x, center_y + offset_y, color);
        put_pixel_checked(image, center_x + offset_x, center_y - offset_y, color);
    }
}

fn put_pixel_checked(image: &mut RgbaImage, x: i32, y: i32, color: Rgba<u8>) {
    if let (Ok(x), Ok(y)) = (u32::try_from(x), u32::try_from(y)) {
        if x < image.width() && y < image.height() {
            image.put_pixel(x, y, color);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::io::Write;
    use std::sync::Arc;

    use super::*;
    use crate::generator::{AbstractGenerator, MathGenerator, RandomGenerator};
    use crate::icaptcha::ICaptcha;
    use crate::{
        AbstractCaptcha, CaptchaError, CaptchaUtil, CircleCaptcha, CodeGenerator, GifCaptcha,
        LineCaptcha, RenderedCaptcha, ShearCaptcha,
    };

    struct BrokenWriter;

    #[derive(Debug)]
    struct EmptyGenerator;

    impl CodeGenerator for EmptyGenerator {
        fn generate(&self) -> String {
            String::new()
        }

        fn verify(&self, _generated: &str, _input: &str) -> bool {
            false
        }
    }

    impl Write for BrokenWriter {
        fn write(&mut self, _buffer: &[u8]) -> std::io::Result<usize> {
            Err(std::io::Error::other("broken"))
        }

        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    fn exercise_trait(captcha: &mut dyn ICaptcha) {
        captcha.create_code().unwrap();
        let code = captcha.code().unwrap().to_owned();
        assert!(captcha.verify(&code));
        assert!(!captcha.image_bytes().unwrap().is_empty());
    }

    #[test]
    fn generators_cover_validation_and_math() {
        assert_eq!(AbstractGenerator::new(0), Err(CaptchaError::InvalidLength));
        assert_eq!(
            AbstractGenerator::with_alphabet(&[], 2),
            Err(CaptchaError::InvalidAlphabet)
        );
        assert_eq!(
            AbstractGenerator::with_alphabet(&[0xff], 2),
            Err(CaptchaError::InvalidAlphabet)
        );
        let random = RandomGenerator::with_alphabet(b"A", 3).unwrap();
        assert_eq!(random.length(), 3);
        assert_eq!(random.generate(), "AAA");
        assert!(random.verify("AAA", " aaa "));
        assert!(!random.verify("AAA", "   "));

        assert_eq!(
            MathGenerator::new(0, true),
            Err(CaptchaError::InvalidLength)
        );
        assert_eq!(
            MathGenerator::new(9, true),
            Err(CaptchaError::InvalidLength)
        );
        assert_eq!(
            MathGenerator::default(),
            MathGenerator::with_negative_results(true)
        );
        let math = MathGenerator::new(2, false).unwrap();
        assert_eq!(math.length(), 6);
        assert!(math.verify("12+3 =", "15"));
        assert!(math.verify("12-3 =", "9"));
        assert!(math.verify("12*3 =", "36"));
        assert!(!math.verify("bad", "0"));
        assert!(!math.verify("1/2=", "0"));
        assert!(!math.verify("9223372036854775807+1=", "0"));
        assert!(!math.verify("bad+1=", "0"));
        assert!(!math.verify("1+bad=", "0"));
        assert!(!math.verify("1+2=", "bad"));
        for _ in 0..20 {
            let code = math.generate();
            assert_eq!(code.len(), math.length());
            if code.contains('-') {
                assert!(MathGenerator::evaluate(&code).unwrap() >= 0);
            }
        }
    }

    #[test]
    fn raster_variants_generate_real_media_and_common_facade_works() {
        let mut line = CaptchaUtil::create_line_captcha(120, 40).unwrap();
        line.set_font(CaptchaFont { scale: 2 })
            .set_background(CaptchaColor([240, 240, 240, 255]))
            .set_text_alpha(0.8)
            .set_stroke(CaptchaStroke { width: 2 });
        let code = line.code().unwrap().to_owned();
        assert!(line.verify(&code.to_ascii_lowercase()));
        assert!(line.image_bytes().unwrap().starts_with(b"\x89PNG"));
        assert_eq!(line.image().unwrap().width(), 120);
        assert!(line.image_base64().unwrap().len() > 20);
        assert!(
            line.image_base64_data()
                .unwrap()
                .starts_with("data:image/png;base64,")
        );
        assert_eq!(line.create_image("A2").unwrap().height(), 40);
        assert_eq!(line.create_image("🙂").unwrap().width(), 120);
        assert!(format!("{line:?}").contains("AbstractCaptcha"));
        assert!(!line.generator().generate().is_empty());

        let mut output = Vec::new();
        line.write_to(&mut output).unwrap();
        assert_eq!(output, line.image_bytes().unwrap());
        let path = std::env::temp_dir().join(format!("hutool-captcha-{}.png", std::process::id()));
        line.write_to_path(path.as_path()).unwrap();
        assert_eq!(fs::read(&path).unwrap(), output);
        fs::remove_file(path).unwrap();

        line.set_generator(Arc::new(RandomGenerator::with_alphabet(b"Z", 2).unwrap()));
        assert_eq!(line.code().unwrap(), "ZZ");

        let mut circle = CaptchaUtil::create_circle_captcha(120, 40).unwrap();
        circle.create_code().unwrap();
        assert!(circle.image_bytes().unwrap().starts_with(b"\x89PNG"));
        assert_eq!(circle.create_image("C").unwrap().width(), 120);
        let _: &AbstractCaptcha = &circle;
        let mut shear = ShearCaptcha::with_count(120, 40, 4).unwrap();
        shear.create_code().unwrap();
        assert!(shear.image_bytes().unwrap().starts_with(b"\x89PNG"));
        assert_eq!(shear.create_image("S").unwrap().width(), 120);
        let _: &AbstractCaptcha = &shear;

        exercise_trait(&mut *line);
    }

    #[test]
    fn gif_and_constructor_variants_are_usable() {
        let mut gif = CaptchaUtil::create_gif_captcha(120, 40)
            .unwrap()
            .set_quality(0)
            .set_repeat(2)
            .set_min_color(220)
            .set_max_color(20);
        gif.create_code().unwrap();
        assert!(gif.image_bytes().unwrap().starts_with(b"GIF"));
        assert!(
            gif.image_base64_data()
                .unwrap()
                .starts_with("data:image/gif;base64,")
        );
        assert_eq!(gif.image().unwrap().width(), 120);
        assert_eq!(gif.create_image("G").unwrap().width(), 120);
        let _: &AbstractCaptcha = &gif;

        let mut infinite = CaptchaUtil::create_gif_captcha(100, 32).unwrap();
        infinite.create_code().unwrap();
        assert!(infinite.image_bytes().unwrap().starts_with(b"GIF"));

        let generator: Arc<dyn CodeGenerator> = Arc::new(RandomGenerator::default());
        assert!(LineCaptcha::with_generator(100, 32, Arc::clone(&generator), 1).is_ok());
        assert!(LineCaptcha::with_size(100, 32, 3, 1, 0.5).is_ok());
        assert!(CircleCaptcha::with_size(100, 32, 3, 1, 0.5).is_ok());
        assert!(ShearCaptcha::with_code_count(100, 32, 3, 2).is_ok());
        assert!(ShearCaptcha::with_size(100, 32, 3, 2, 0.5).is_ok());
        assert!(GifCaptcha::with_generator(100, 32, generator, 1).is_ok());
        assert!(GifCaptcha::with_size(100, 32, 3, 1, 0.5).is_ok());
        assert!(CaptchaUtil::create_shear_captcha(100, 32).is_ok());
        assert_eq!(
            LineCaptcha::new(20, 10).unwrap_err(),
            CaptchaError::InvalidDimensions
        );
        assert_eq!(
            LineCaptcha::new(120, 40).unwrap().create_image(""),
            Err(CaptchaError::InvalidRenderCode)
        );
        assert_eq!(
            CircleCaptcha::new(120, 40).unwrap().create_image(""),
            Err(CaptchaError::InvalidRenderCode)
        );
        assert_eq!(
            ShearCaptcha::new(120, 40).unwrap().create_image(""),
            Err(CaptchaError::InvalidRenderCode)
        );
        assert_eq!(
            GifCaptcha::new(120, 40).unwrap().create_image(""),
            Err(CaptchaError::InvalidRenderCode)
        );
    }

    #[test]
    fn constructor_and_generation_error_paths_are_structured() {
        let generator = || Arc::new(RandomGenerator::default()) as Arc<dyn CodeGenerator>;
        assert!(LineCaptcha::with_generator(20, 10, generator(), 1).is_err());
        assert!(CircleCaptcha::with_generator(20, 10, generator(), 1).is_err());
        assert!(ShearCaptcha::with_generator(20, 10, generator(), 1).is_err());
        assert!(GifCaptcha::with_generator(20, 10, generator(), 1).is_err());

        assert!(LineCaptcha::with_code_count(100, 32, 0, 1).is_err());
        assert!(CircleCaptcha::with_code_count(100, 32, 0, 1).is_err());
        assert!(ShearCaptcha::with_code_count(100, 32, 0, 1).is_err());
        assert!(GifCaptcha::with_code_count(100, 32, 0, 1).is_err());

        for result in [
            LineCaptcha::with_size(100, 32, 0, 1, 0.5).map(|_| ()),
            CircleCaptcha::with_size(100, 32, 0, 1, 0.5).map(|_| ()),
            ShearCaptcha::with_size(100, 32, 0, 1, 0.5).map(|_| ()),
            GifCaptcha::with_size(100, 32, 0, 1, 0.5).map(|_| ()),
            LineCaptcha::with_size(20, 10, 3, 1, 0.5).map(|_| ()),
            CircleCaptcha::with_size(20, 10, 3, 1, 0.5).map(|_| ()),
            ShearCaptcha::with_size(20, 10, 3, 1, 0.5).map(|_| ()),
            GifCaptcha::with_size(20, 10, 3, 1, 0.5).map(|_| ()),
        ] {
            assert!(result.is_err());
        }

        let mut empty = LineCaptcha::with_generator(100, 32, Arc::new(EmptyGenerator), 1).unwrap();
        assert!(!EmptyGenerator.verify("", ""));
        assert_eq!(empty.create_code(), Err(CaptchaError::InvalidRenderCode));
        assert_eq!(empty.code(), Err(CaptchaError::InvalidRenderCode));
        assert_eq!(empty.image_bytes(), Err(CaptchaError::InvalidRenderCode));
        assert_eq!(empty.image(), Err(CaptchaError::InvalidRenderCode));
        assert_eq!(empty.image_base64(), Err(CaptchaError::InvalidRenderCode));
        assert_eq!(
            empty.image_base64_data(),
            Err(CaptchaError::InvalidRenderCode)
        );
        assert_eq!(
            empty.write_to_path(std::env::temp_dir().as_path()),
            Err(CaptchaError::InvalidRenderCode)
        );
        assert_eq!(
            empty.write_to(&mut Vec::new()),
            Err(CaptchaError::InvalidRenderCode)
        );

        let mut corrupt = LineCaptcha::new(100, 32).unwrap();
        corrupt.0.rendered = Some(RenderedCaptcha::new("image/png", vec![1]));
        assert!(
            corrupt
                .image()
                .unwrap_err()
                .to_string()
                .contains("image encoding failed")
        );
    }

    #[test]
    fn io_errors_are_structured() {
        let mut captcha = LineCaptcha::new(100, 32).unwrap();
        let missing = std::env::temp_dir()
            .join("hutool-missing-dir")
            .join("captcha.png");
        assert!(captcha.write_to_path(missing.as_path()).is_err());

        assert!(captcha.write_to(&mut BrokenWriter).is_err());
        let mut writer = BrokenWriter;
        assert!(writer.flush().is_ok());

        let mut tiny = RgbaImage::new(1, 1);
        draw_glyph(&mut tiny, [u8::MAX; 8], 2, 2, 1, Rgba([1, 2, 3, 4]));
        draw_circle(&mut tiny, 0, 0, 10, Rgba([1, 2, 3, 4]));
    }
}
