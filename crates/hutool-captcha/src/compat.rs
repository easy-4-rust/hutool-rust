//! Hutool-named raster CAPTCHA facade.
//!
//! 对齐: `cn.hutool.captcha` (Hutool 验证码核心实现)
//! 来源: hutool-captcha/src/main/java/cn/hutool/captcha/
//! 中文说明: Hutool 风格的光栅验证码门面，包含 LineCaptcha、CircleCaptcha、ShearCaptcha、GifCaptcha 等实现

use std::fmt;
use std::fs;
use std::io::{Cursor, Write};
use std::ops::{Deref, DerefMut};
use std::path::Path;
use std::sync::Arc;

use base64::Engine as _;
use font8x8::{BASIC_FONTS, UnicodeFonts as _};
use image::codecs::gif::{GifEncoder, Repeat};
use image::{Delay, DynamicImage, Frame, ImageFormat, Rgba, RgbaImage};
use rand::Rng;

use crate::{CaptchaError, CodeGenerator, RenderedCaptcha, constant_time_ascii_case_eq};

const HUTOOL_ALPHABET: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";

/// Shared state used by Hutool-style random generators.
///
/// 对齐: `cn.hutool.captcha.generator.RandomGenerator` (基类)
/// 中文说明: Hutool 风格随机生成器的共享状态，管理字母表和生成长度
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AbstractGenerator {
    alphabet: Vec<u8>,
    length: usize,
}

impl AbstractGenerator {
    /// Creates a generator configuration using Hutool's alphanumeric alphabet.
    pub fn new(length: usize) -> Result<Self, CaptchaError> {
        Self::with_alphabet(HUTOOL_ALPHABET, length)
    }

    /// Creates a generator configuration with a caller-supplied ASCII alphabet.
    pub fn with_alphabet(alphabet: &[u8], length: usize) -> Result<Self, CaptchaError> {
        if length == 0 {
            return Err(CaptchaError::InvalidLength);
        }
        if alphabet.is_empty() || !alphabet.is_ascii() {
            return Err(CaptchaError::InvalidAlphabet);
        }
        Ok(Self {
            alphabet: alphabet.to_vec(),
            length,
        })
    }

    /// Returns the generated code length.
    #[must_use]
    pub const fn length(&self) -> usize {
        self.length
    }
}

/// Hutool-compatible random-character generator.
///
/// 对齐: `cn.hutool.captcha.generator.RandomGenerator`
/// 中文说明: Hutool 兼容的随机字符验证码生成器
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RandomGenerator(AbstractGenerator);

impl RandomGenerator {
    /// Creates an alphanumeric generator.
    pub fn new(length: usize) -> Result<Self, CaptchaError> {
        AbstractGenerator::new(length).map(Self)
    }

    /// Creates a generator using a custom ASCII alphabet.
    pub fn with_alphabet(alphabet: &[u8], length: usize) -> Result<Self, CaptchaError> {
        AbstractGenerator::with_alphabet(alphabet, length).map(Self)
    }

    /// Returns the generated code length.
    #[must_use]
    pub const fn length(&self) -> usize {
        self.0.length()
    }
}

impl CodeGenerator for RandomGenerator {
    fn generate(&self) -> String {
        let mut rng = rand::rng();
        (0..self.0.length)
            .map(|_| {
                let index = rng.random_range(0..self.0.alphabet.len());
                char::from(self.0.alphabet[index])
            })
            .collect()
    }

    fn verify(&self, generated: &str, input: &str) -> bool {
        !input.trim().is_empty()
            && constant_time_ascii_case_eq(generated.as_bytes(), input.trim().as_bytes())
    }
}

impl Default for RandomGenerator {
    fn default() -> Self {
        Self::new(5).expect("the default CAPTCHA length is valid")
    }
}

/// Arithmetic CAPTCHA generator using addition, subtraction, and multiplication.
///
/// 对齐: `cn.hutool.captcha.generator.MathGenerator`
/// 中文说明: 算术验证码生成器，支持加法、减法和乘法运算
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MathGenerator {
    number_length: u8,
    allow_negative: bool,
}

impl MathGenerator {
    /// Creates an arithmetic generator.
    pub fn new(number_length: u8, allow_negative: bool) -> Result<Self, CaptchaError> {
        if number_length == 0 || number_length > 8 {
            return Err(CaptchaError::InvalidLength);
        }
        Ok(Self {
            number_length,
            allow_negative,
        })
    }

    /// Creates a generator with Hutool's default operand width.
    pub fn with_negative_results(allow_negative: bool) -> Self {
        Self {
            number_length: 2,
            allow_negative,
        }
    }

    /// Returns the formatted challenge length.
    #[must_use]
    pub const fn length(&self) -> usize {
        self.number_length as usize * 2 + 2
    }

    /// Evaluates an arithmetic CAPTCHA expression produced by [`Self::generate`].
    ///
    /// Mirrors Hutool's `Calculator.conversion(code)` used by `MathGenerator.verify`.
    #[must_use]
    pub fn evaluate(code: &str) -> Option<i64> {
        let expression = code.strip_suffix('=')?;
        for operator in ['+', '-', '*'] {
            if let Some((left, right)) = expression.split_once(operator) {
                let left = left.trim().parse::<i64>().ok()?;
                let right = right.trim().parse::<i64>().ok()?;
                return if operator == '+' {
                    left.checked_add(right)
                } else if operator == '-' {
                    left.checked_sub(right)
                } else {
                    left.checked_mul(right)
                };
            }
        }
        None
    }
}

impl Default for MathGenerator {
    fn default() -> Self {
        Self::with_negative_results(true)
    }
}

impl CodeGenerator for MathGenerator {
    fn generate(&self) -> String {
        let limit = 10_i64.pow(u32::from(self.number_length));
        let mut rng = rand::rng();
        let left = rng.random_range(0..limit);
        let operator = ['+', '-', '*'][rng.random_range(0..3)];
        let right = if !self.allow_negative && operator == '-' {
            rng.random_range(0..=left)
        } else {
            rng.random_range(0..limit)
        };
        format!(
            "{left:<width$}{operator}{right:<width$}=",
            width = usize::from(self.number_length)
        )
    }

    fn verify(&self, generated: &str, input: &str) -> bool {
        input.trim().parse::<i64>().ok() == Self::evaluate(generated)
    }
}

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
enum CaptchaKind {
    Line,
    Circle,
    Shear,
    Gif,
}

/// Common mutable CAPTCHA state corresponding to Hutool's `AbstractCaptcha`.
///
/// 对齐: `cn.hutool.captcha.AbstractCaptcha`
/// 中文说明: Hutool 的 AbstractCaptcha 对应的通用可变验证码状态
pub struct AbstractCaptcha {
    width: u16,
    height: u16,
    generator: Arc<dyn CodeGenerator>,
    interfere_count: u16,
    font: CaptchaFont,
    background: CaptchaColor,
    text_alpha: u8,
    stroke: CaptchaStroke,
    kind: CaptchaKind,
    code: Option<String>,
    rendered: Option<RenderedCaptcha>,
    gif_quality: u8,
    gif_repeat: u16,
    min_color: u8,
    max_color: u8,
}

impl fmt::Debug for AbstractCaptcha {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("AbstractCaptcha")
            .field("width", &self.width)
            .field("height", &self.height)
            .field("interfere_count", &self.interfere_count)
            .field("kind", &self.kind)
            .field("generated", &self.code.is_some())
            .finish_non_exhaustive()
    }
}

impl AbstractCaptcha {
    fn new(
        width: u16,
        height: u16,
        generator: Arc<dyn CodeGenerator>,
        interfere_count: u16,
        font_scale: f32,
        kind: CaptchaKind,
    ) -> Result<Self, CaptchaError> {
        let pixels = u32::from(width).saturating_mul(u32::from(height));
        if width < 32 || height < 16 || pixels > 4_000_000 {
            return Err(CaptchaError::InvalidDimensions);
        }
        let requested_scale = (f32::from(height) * font_scale / 8.0)
            .round()
            .clamp(1.0, 12.0);
        let scale = (1_u8..=12)
            .find(|candidate| f32::from(*candidate) >= requested_scale)
            .unwrap_or(12);
        Ok(Self {
            width,
            height,
            generator,
            interfere_count,
            font: CaptchaFont { scale },
            background: CaptchaColor([255, 255, 255, 255]),
            text_alpha: 255,
            stroke: CaptchaStroke { width: 1 },
            kind,
            code: None,
            rendered: None,
            gif_quality: 10,
            gif_repeat: 0,
            min_color: 0,
            max_color: 255,
        })
    }

    /// Generates a new code and encoded image.
    pub fn create_code(&mut self) -> Result<(), CaptchaError> {
        let code = self.generator.generate();
        let rendered = self.render(&code)?;
        self.code = Some(code);
        self.rendered = Some(rendered);
        Ok(())
    }

    /// Returns the generated code, generating it lazily when necessary.
    pub fn code(&mut self) -> Result<&str, CaptchaError> {
        self.ensure_generated()?;
        self.code.as_deref().ok_or(CaptchaError::InvalidRenderCode)
    }

    /// Verifies user input against the current generated code.
    pub fn verify(&self, input: &str) -> bool {
        self.code
            .as_deref()
            .is_some_and(|code| self.generator.verify(code, input))
    }

    /// Writes encoded image bytes to a filesystem path.
    pub fn write_to_path(&mut self, path: &Path) -> Result<(), CaptchaError> {
        self.ensure_generated()?;
        let bytes = self.rendered_bytes_invariant();
        fs::write(path, bytes).map_err(|error| CaptchaError::Io(error.to_string()))
    }

    /// Writes encoded image bytes to a stream.
    pub fn write_to(&mut self, output: &mut dyn Write) -> Result<(), CaptchaError> {
        self.ensure_generated()?;
        let bytes = self.rendered_bytes_invariant();
        output
            .write_all(bytes)
            .map_err(|error| CaptchaError::Io(error.to_string()))
    }

    /// Returns encoded PNG or GIF bytes.
    pub fn image_bytes(&mut self) -> Result<&[u8], CaptchaError> {
        self.ensure_generated()?;
        self.rendered
            .as_ref()
            .map(RenderedCaptcha::bytes)
            .ok_or(CaptchaError::InvalidRenderCode)
    }

    /// Decodes and returns the current raster image.
    pub fn image(&mut self) -> Result<DynamicImage, CaptchaError> {
        image::load_from_memory(self.image_bytes()?).map_err(CaptchaError::from)
    }

    /// Returns the image bytes as standard Base64.
    pub fn image_base64(&mut self) -> Result<String, CaptchaError> {
        Ok(base64::engine::general_purpose::STANDARD.encode(self.image_bytes()?))
    }

    /// Returns a browser-ready image data URI.
    pub fn image_base64_data(&mut self) -> Result<String, CaptchaError> {
        self.ensure_generated()?;
        let mime = if self.kind == CaptchaKind::Gif {
            "image/gif"
        } else {
            "image/png"
        };
        let bytes = self.rendered_bytes_invariant();
        Ok(format!(
            "data:{mime};base64,{}",
            base64::engine::general_purpose::STANDARD.encode(bytes)
        ))
    }

    /// Replaces the bitmap font configuration and invalidates rendered media.
    pub fn set_font(&mut self, font: CaptchaFont) -> &mut Self {
        self.font = CaptchaFont {
            scale: font.scale.clamp(1, 12),
        };
        self.invalidate();
        self
    }

    /// Returns the active code generator.
    #[must_use]
    pub fn generator(&self) -> &dyn CodeGenerator {
        self.generator.as_ref()
    }

    /// Replaces the code generator and invalidates the existing challenge.
    pub fn set_generator(&mut self, generator: Arc<dyn CodeGenerator>) -> &mut Self {
        self.generator = generator;
        self.invalidate();
        self
    }

    /// Sets the background color.
    pub fn set_background(&mut self, background: CaptchaColor) -> &mut Self {
        self.background = background;
        self.invalidate();
        self
    }

    /// Sets text alpha in the inclusive range 0.0..=1.0.
    pub fn set_text_alpha(&mut self, alpha: f32) -> &mut Self {
        let requested_alpha = (alpha.clamp(0.0, 1.0) * 255.0).round();
        self.text_alpha = (0_u8..=u8::MAX)
            .find(|candidate| f32::from(*candidate) >= requested_alpha)
            .unwrap_or(u8::MAX);
        self.invalidate();
        self
    }

    /// Sets interference stroke width.
    pub fn set_stroke(&mut self, stroke: CaptchaStroke) -> &mut Self {
        self.stroke = CaptchaStroke {
            width: stroke.width.clamp(1, 16),
        };
        self.invalidate();
        self
    }

    fn invalidate(&mut self) {
        self.code = None;
        self.rendered = None;
    }

    fn rendered_bytes_invariant(&self) -> &[u8] {
        self.rendered
            .as_ref()
            .expect("generation stores rendered media")
            .bytes()
    }

    fn ensure_generated(&mut self) -> Result<(), CaptchaError> {
        if self.rendered.is_none() {
            self.create_code()?;
        }
        Ok(())
    }

    fn render(&self, code: &str) -> Result<RenderedCaptcha, CaptchaError> {
        if code.is_empty() || code.chars().count() > 32 {
            return Err(CaptchaError::InvalidRenderCode);
        }
        if self.kind == CaptchaKind::Gif {
            return Ok(self.render_gif(code));
        }
        let image = self.render_frame(code, 255);
        let mut bytes = Vec::new();
        DynamicImage::ImageRgba8(image)
            .write_to(&mut Cursor::new(&mut bytes), ImageFormat::Png)
            .expect("encoding an in-memory RGBA image as PNG is infallible");
        Ok(RenderedCaptcha::new("image/png", bytes))
    }

    fn render_gif(&self, code: &str) -> RenderedCaptcha {
        let mut bytes = Vec::new();
        {
            let mut encoder =
                GifEncoder::new_with_speed(&mut bytes, i32::from(self.gif_quality.clamp(1, 30)));
            let repeat = if self.gif_repeat == 0 {
                Repeat::Infinite
            } else {
                Repeat::Finite(self.gif_repeat)
            };
            encoder
                .set_repeat(repeat)
                .expect("writing GIF metadata to a Vec is infallible");
            let count = code.chars().count().max(1);
            for index in 0..count {
                let alpha = u8::try_from(((index + 1) * 255) / count).unwrap_or(255);
                let frame = Frame::from_parts(
                    self.render_frame(code, alpha),
                    0,
                    0,
                    Delay::from_numer_denom_ms(100, 1),
                );
                encoder
                    .encode_frame(frame)
                    .expect("writing a bounded RGBA frame to a Vec is infallible");
            }
        }
        RenderedCaptcha::new("image/gif", bytes)
    }

    fn render_frame(&self, code: &str, frame_alpha: u8) -> RgbaImage {
        let mut image = RgbaImage::from_pixel(
            u32::from(self.width),
            u32::from(self.height),
            Rgba(self.background.0),
        );
        let mut rng = rand::rng();
        self.draw_interference(&mut image, &mut rng);
        let glyph_count = u32::try_from(code.chars().count()).unwrap_or(1);
        let slot = u32::from(self.width) / (glyph_count + 1);
        for (index, character) in code.chars().enumerate() {
            let bitmap = BASIC_FONTS
                .get(character)
                .or_else(|| BASIC_FONTS.get('?'))
                .unwrap_or([0; 8]);
            let index = u32::try_from(index).unwrap_or(0);
            let scale = u32::from(self.font.scale);
            let x = slot.saturating_mul(index + 1).saturating_sub(4 * scale);
            let y = u32::from(self.height).saturating_sub(8 * scale) / 2;
            let color = self.random_text_color(&mut rng, frame_alpha);
            draw_glyph(&mut image, bitmap, x, y, scale, color);
        }
        image
    }

    fn random_text_color(&self, rng: &mut impl Rng, frame_alpha: u8) -> Rgba<u8> {
        let (min, max) = if self.min_color <= self.max_color {
            (self.min_color, self.max_color)
        } else {
            (0, 255)
        };
        Rgba([
            rng.random_range(min..=max),
            rng.random_range(min..=max),
            rng.random_range(min..=max),
            self.text_alpha.min(frame_alpha),
        ])
    }

    fn draw_interference(&self, image: &mut RgbaImage, rng: &mut impl Rng) {
        let width = i32::from(self.width);
        let height = i32::from(self.height);
        for _ in 0..self.interfere_count {
            let color = Rgba([
                rng.random_range(0..=180),
                rng.random_range(0..=180),
                rng.random_range(0..=180),
                180,
            ]);
            match self.kind {
                CaptchaKind::Circle => {
                    let radius = rng.random_range(1..=(height / 4).max(1));
                    draw_circle(
                        image,
                        rng.random_range(0..width),
                        rng.random_range(0..height),
                        radius,
                        color,
                    );
                }
                CaptchaKind::Line | CaptchaKind::Shear | CaptchaKind::Gif => {
                    let thickness = if self.kind == CaptchaKind::Shear {
                        self.interfere_count.clamp(1, 16) as u8
                    } else {
                        self.stroke.width
                    };
                    draw_line(
                        image,
                        (rng.random_range(0..width), rng.random_range(0..height)),
                        (rng.random_range(0..width), rng.random_range(0..height)),
                        thickness,
                        color,
                    );
                }
            }
        }
    }
}

/// Common operations exposed by all Hutool-style CAPTCHA variants.
///
/// 对齐: `cn.hutool.captcha.ICaptcha`
/// 中文说明: 所有 Hutool 风格验证码变体的通用操作接口
pub trait ICaptcha {
    /// Generates a fresh challenge.
    fn create_code(&mut self) -> Result<(), CaptchaError>;
    /// Returns the generated challenge text.
    fn code(&mut self) -> Result<&str, CaptchaError>;
    /// Verifies user input.
    fn verify(&self, input: &str) -> bool;
    /// Returns encoded image bytes.
    fn image_bytes(&mut self) -> Result<&[u8], CaptchaError>;
}

impl ICaptcha for AbstractCaptcha {
    fn create_code(&mut self) -> Result<(), CaptchaError> {
        AbstractCaptcha::create_code(self)
    }

    fn code(&mut self) -> Result<&str, CaptchaError> {
        AbstractCaptcha::code(self)
    }

    fn verify(&self, input: &str) -> bool {
        AbstractCaptcha::verify(self, input)
    }

    fn image_bytes(&mut self) -> Result<&[u8], CaptchaError> {
        AbstractCaptcha::image_bytes(self)
    }
}

macro_rules! captcha_type {
    ($name:ident, $kind:expr, $default_interference:expr, $java_class:expr) => {
        #[doc = concat!("Hutool-compatible `", stringify!($name), "`.")]
        #[doc = ""]
        #[doc = concat!("对齐: `", $java_class, "`")]
        /// 中文说明: Hutool 兼容的验证码类型
        #[derive(Debug)]
        pub struct $name(AbstractCaptcha);

        impl $name {
            /// Creates a five-character CAPTCHA with default interference.
            pub fn new(width: u16, height: u16) -> Result<Self, CaptchaError> {
                Self::with_code_count(width, height, 5, $default_interference)
            }

            /// Creates a CAPTCHA with an explicit code and interference count.
            pub fn with_code_count(
                width: u16,
                height: u16,
                code_count: usize,
                interference: u16,
            ) -> Result<Self, CaptchaError> {
                let generator = Arc::new(RandomGenerator::new(code_count)?);
                Self::with_generator(width, height, generator, interference)
            }

            /// Creates a CAPTCHA with an injected generator.
            pub fn with_generator(
                width: u16,
                height: u16,
                generator: Arc<dyn CodeGenerator>,
                interference: u16,
            ) -> Result<Self, CaptchaError> {
                AbstractCaptcha::new(width, height, generator, interference, 0.75, $kind).map(Self)
            }

            /// Creates a CAPTCHA with a Hutool-style font-size multiplier.
            pub fn with_size(
                width: u16,
                height: u16,
                code_count: usize,
                interference: u16,
                size: f32,
            ) -> Result<Self, CaptchaError> {
                let generator = Arc::new(RandomGenerator::new(code_count)?);
                AbstractCaptcha::new(width, height, generator, interference, size, $kind).map(Self)
            }

            /// Renders a supplied code without mutating challenge state.
            pub fn create_image(&self, code: &str) -> Result<DynamicImage, CaptchaError> {
                let rendered = self.0.render(code)?;
                Ok(image::load_from_memory(rendered.bytes())
                    .expect("the compatibility renderer emits valid PNG or GIF bytes"))
            }
        }

        impl Deref for $name {
            type Target = AbstractCaptcha;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl DerefMut for $name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
    };
}

captcha_type!(LineCaptcha, CaptchaKind::Line, 150, "cn.hutool.captcha.LineCaptcha");
captcha_type!(CircleCaptcha, CaptchaKind::Circle, 15, "cn.hutool.captcha.CircleCaptcha");
captcha_type!(ShearCaptcha, CaptchaKind::Shear, 4, "cn.hutool.captcha.ShearCaptcha");
captcha_type!(GifCaptcha, CaptchaKind::Gif, 10, "cn.hutool.captcha.GifCaptcha");

impl ShearCaptcha {
    /// Creates a shear CAPTCHA with the default line thickness.
    pub fn with_count(width: u16, height: u16, code_count: usize) -> Result<Self, CaptchaError> {
        Self::with_code_count(width, height, code_count, 4)
    }
}

impl GifCaptcha {
    /// Sets GIF color-quantization speed/quality.
    ///
    /// Values below 1 are clamped to 1 (Hutool `GifCaptcha.setQuality`).
    #[must_use]
    pub fn set_quality(mut self, quality: u8) -> Self {
        self.0.gif_quality = quality.clamp(1, 30);
        self.0.invalidate();
        self
    }

    /// Returns the GIF quantization quality (Hutool `quality` field).
    #[must_use]
    pub const fn quality(&self) -> u8 {
        self.0.gif_quality
    }

    /// Sets GIF repetition count; zero means infinite.
    ///
    /// Negative values are clamped to `0` (Hutool `GifCaptcha.setRepeat` / `Math.max`).
    #[must_use]
    pub fn set_repeat(mut self, repeat: i32) -> Self {
        self.0.gif_repeat = if repeat < 0 {
            0
        } else {
            u16::try_from(repeat).unwrap_or(u16::MAX)
        };
        self.0.invalidate();
        self
    }

    /// Returns the GIF frame repeat count (Hutool `repeat` field).
    #[must_use]
    pub const fn repeat(&self) -> u16 {
        self.0.gif_repeat
    }

    /// Sets the maximum random text color component.
    #[must_use]
    pub fn set_max_color(mut self, maximum: u8) -> Self {
        self.0.max_color = maximum;
        self.0.invalidate();
        self
    }

    /// Returns the maximum random text color component (Hutool `maxColor`).
    #[must_use]
    pub const fn max_color(&self) -> u8 {
        self.0.max_color
    }

    /// Sets the minimum random text color component.
    #[must_use]
    pub fn set_min_color(mut self, minimum: u8) -> Self {
        self.0.min_color = minimum;
        self.0.invalidate();
        self
    }

    /// Returns the minimum random text color component (Hutool `minColor`).
    #[must_use]
    pub const fn min_color(&self) -> u8 {
        self.0.min_color
    }

    /// Samples a random RGB color in `[min, max]` (Hutool `GifCaptcha.getRandomColor`).
    #[must_use]
    pub fn random_color(min: u8, max: u8) -> CaptchaColor {
        let (lo, hi) = if min <= max { (min, max) } else { (0, 255) };
        let mut rng = rand::rng();
        CaptchaColor([
            rng.random_range(lo..=hi),
            rng.random_range(lo..=hi),
            rng.random_range(lo..=hi),
            255,
        ])
    }
}

/// Hutool-named factory facade.
///
/// 对齐: `cn.hutool.captcha.CaptchaUtil`
/// 中文说明: Hutool 风格的验证码工厂门面类，提供各种验证码的便捷创建方法
pub struct CaptchaUtil;

impl CaptchaUtil {
    /// Creates a line CAPTCHA.
    pub fn create_line_captcha(width: u16, height: u16) -> Result<LineCaptcha, CaptchaError> {
        LineCaptcha::new(width, height)
    }

    /// Creates a line CAPTCHA with explicit code and interference counts.
    pub fn create_line_captcha_with_count(
        width: u16,
        height: u16,
        code_count: usize,
        line_count: u16,
    ) -> Result<LineCaptcha, CaptchaError> {
        LineCaptcha::with_code_count(width, height, code_count, line_count)
    }

    /// Creates a line CAPTCHA with a Hutool-style font-size multiplier.
    pub fn create_line_captcha_with_size(
        width: u16,
        height: u16,
        code_count: usize,
        line_count: u16,
        size: f32,
    ) -> Result<LineCaptcha, CaptchaError> {
        LineCaptcha::with_size(width, height, code_count, line_count, size)
    }

    /// Creates a circle CAPTCHA.
    pub fn create_circle_captcha(width: u16, height: u16) -> Result<CircleCaptcha, CaptchaError> {
        CircleCaptcha::new(width, height)
    }

    /// Creates a circle CAPTCHA with explicit code and interference counts.
    pub fn create_circle_captcha_with_count(
        width: u16,
        height: u16,
        code_count: usize,
        circle_count: u16,
    ) -> Result<CircleCaptcha, CaptchaError> {
        CircleCaptcha::with_code_count(width, height, code_count, circle_count)
    }

    /// Creates a circle CAPTCHA with a Hutool-style font-size multiplier.
    pub fn create_circle_captcha_with_size(
        width: u16,
        height: u16,
        code_count: usize,
        circle_count: u16,
        size: f32,
    ) -> Result<CircleCaptcha, CaptchaError> {
        CircleCaptcha::with_size(width, height, code_count, circle_count, size)
    }

    /// Creates a shear CAPTCHA.
    pub fn create_shear_captcha(width: u16, height: u16) -> Result<ShearCaptcha, CaptchaError> {
        ShearCaptcha::new(width, height)
    }

    /// Creates a shear CAPTCHA with explicit code and thickness.
    pub fn create_shear_captcha_with_count(
        width: u16,
        height: u16,
        code_count: usize,
        thickness: u16,
    ) -> Result<ShearCaptcha, CaptchaError> {
        ShearCaptcha::with_code_count(width, height, code_count, thickness)
    }

    /// Creates a shear CAPTCHA with a Hutool-style font-size multiplier.
    pub fn create_shear_captcha_with_size(
        width: u16,
        height: u16,
        code_count: usize,
        thickness: u16,
        size: f32,
    ) -> Result<ShearCaptcha, CaptchaError> {
        ShearCaptcha::with_size(width, height, code_count, thickness, size)
    }

    /// Creates an animated GIF CAPTCHA.
    pub fn create_gif_captcha(width: u16, height: u16) -> Result<GifCaptcha, CaptchaError> {
        GifCaptcha::new(width, height)
    }

    /// Creates a GIF CAPTCHA with an explicit code count (default interference 10).
    pub fn create_gif_captcha_with_count(
        width: u16,
        height: u16,
        code_count: usize,
    ) -> Result<GifCaptcha, CaptchaError> {
        GifCaptcha::with_code_count(width, height, code_count, 10)
    }

    /// Creates a GIF CAPTCHA with a Hutool-style font-size multiplier.
    pub fn create_gif_captcha_with_size(
        width: u16,
        height: u16,
        code_count: usize,
        thickness: u16,
        size: f32,
    ) -> Result<GifCaptcha, CaptchaError> {
        GifCaptcha::with_size(width, height, code_count, thickness, size)
    }
}

fn draw_glyph(image: &mut RgbaImage, bitmap: [u8; 8], x: u32, y: u32, scale: u32, color: Rgba<u8>) {
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

fn draw_line(
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

fn draw_circle(image: &mut RgbaImage, center_x: i32, center_y: i32, radius: i32, color: Rgba<u8>) {
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
    use super::*;

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
