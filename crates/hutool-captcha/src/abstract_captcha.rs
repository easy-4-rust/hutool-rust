//! `AbstractCaptcha` 对象的独立模块。
//!
//! 对齐: `cn.hutool.captcha.AbstractCaptcha`
//! 来源: `hutool-captcha/src/main/java/cn/hutool/captcha/AbstractCaptcha.java`
//! 中文说明: 验证码抽象基类，统一封装验证码生成、校验、图片编码与懒加载逻辑。

use std::fmt;
use std::fs;
use std::io::{Cursor, Write};
use std::path::Path;
use std::sync::Arc;

use base64::Engine as _;
use font8x8::{BASIC_FONTS, UnicodeFonts as _};
use image::codecs::gif::{GifEncoder, Repeat};
use image::{Delay, DynamicImage, Frame, ImageFormat, Rgba, RgbaImage};
use rand::Rng;

use crate::compat::{
    CaptchaColor, CaptchaFont, CaptchaKind, CaptchaStroke, draw_circle, draw_glyph, draw_line,
};
use crate::icaptcha::ICaptcha;
use crate::{CaptchaError, CodeGenerator, RenderedCaptcha};

/// Hutool `AbstractCaptcha` 的 Rust 对应实现。
///
/// 负责保存验证码状态，并根据验证码类型复用 PNG/GIF 渲染逻辑。
pub struct AbstractCaptcha {
    pub(crate) width: u16,
    pub(crate) height: u16,
    pub(crate) generator: Arc<dyn CodeGenerator>,
    pub(crate) interfere_count: u16,
    pub(crate) font: CaptchaFont,
    pub(crate) background: CaptchaColor,
    pub(crate) text_alpha: u8,
    pub(crate) stroke: CaptchaStroke,
    pub(crate) kind: CaptchaKind,
    pub(crate) code: Option<String>,
    pub(crate) rendered: Option<RenderedCaptcha>,
    pub(crate) gif_quality: u8,
    pub(crate) gif_repeat: u16,
    pub(crate) min_color: u8,
    pub(crate) max_color: u8,
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
    /// 创建抽象验证码基类实例。
    ///
    /// 对齐 Java `AbstractCaptcha(int, int, CodeGenerator, int, float)`。
    pub(crate) fn new(
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

    /// 生成验证码字符串与对应图片字节。
    pub fn create_code(&mut self) -> Result<(), CaptchaError> {
        let code = self.generator.generate();
        let rendered = self.render(&code)?;
        self.code = Some(code);
        self.rendered = Some(rendered);
        Ok(())
    }

    /// 返回当前验证码；如果尚未生成则懒加载创建。
    pub fn code(&mut self) -> Result<&str, CaptchaError> {
        self.ensure_generated()?;
        self.code.as_deref().ok_or(CaptchaError::InvalidRenderCode)
    }

    /// 校验用户输入是否与当前验证码匹配。
    pub fn verify(&self, input: &str) -> bool {
        self.code
            .as_deref()
            .is_some_and(|code| self.generator.verify(code, input))
    }

    /// 将图片字节写入指定路径。
    pub fn write_to_path(&mut self, path: &Path) -> Result<(), CaptchaError> {
        self.ensure_generated()?;
        let bytes = self.rendered_bytes_invariant();
        fs::write(path, bytes).map_err(|error| CaptchaError::Io(error.to_string()))
    }

    /// 将图片字节写入输出流。
    pub fn write_to(&mut self, output: &mut dyn Write) -> Result<(), CaptchaError> {
        self.ensure_generated()?;
        let bytes = self.rendered_bytes_invariant();
        output
            .write_all(bytes)
            .map_err(|error| CaptchaError::Io(error.to_string()))
    }

    /// 返回编码后的 PNG 或 GIF 字节。
    pub fn image_bytes(&mut self) -> Result<&[u8], CaptchaError> {
        self.ensure_generated()?;
        self.rendered
            .as_ref()
            .map(RenderedCaptcha::bytes)
            .ok_or(CaptchaError::InvalidRenderCode)
    }

    /// 解码并返回当前图片。
    pub fn image(&mut self) -> Result<DynamicImage, CaptchaError> {
        image::load_from_memory(self.image_bytes()?).map_err(CaptchaError::from)
    }

    /// 返回标准 Base64 编码的图片字节。
    pub fn image_base64(&mut self) -> Result<String, CaptchaError> {
        Ok(base64::engine::general_purpose::STANDARD.encode(self.image_bytes()?))
    }

    /// 返回可直接嵌入浏览器的 Data URI。
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

    /// 设置字体缩放配置，并清理已生成缓存。
    pub fn set_font(&mut self, font: CaptchaFont) -> &mut Self {
        self.font = CaptchaFont {
            scale: font.scale.clamp(1, 12),
        };
        self.invalidate();
        self
    }

    /// 返回当前验证码生成器。
    #[must_use]
    pub fn generator(&self) -> &dyn CodeGenerator {
        self.generator.as_ref()
    }

    /// 设置验证码生成器，并清理已生成缓存。
    pub fn set_generator(&mut self, generator: Arc<dyn CodeGenerator>) -> &mut Self {
        self.generator = generator;
        self.invalidate();
        self
    }

    /// 设置背景色。
    pub fn set_background(&mut self, background: CaptchaColor) -> &mut Self {
        self.background = background;
        self.invalidate();
        self
    }

    /// 设置文字透明度，取值范围 `0.0..=1.0`。
    pub fn set_text_alpha(&mut self, alpha: f32) -> &mut Self {
        let requested_alpha = (alpha.clamp(0.0, 1.0) * 255.0).round();
        self.text_alpha = (0_u8..=u8::MAX)
            .find(|candidate| f32::from(*candidate) >= requested_alpha)
            .unwrap_or(u8::MAX);
        self.invalidate();
        self
    }

    /// 设置干扰元素笔触宽度。
    pub fn set_stroke(&mut self, stroke: CaptchaStroke) -> &mut Self {
        self.stroke = CaptchaStroke {
            width: stroke.width.clamp(1, 16),
        };
        self.invalidate();
        self
    }

    pub(crate) fn invalidate(&mut self) {
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

    pub(crate) fn render(&self, code: &str) -> Result<RenderedCaptcha, CaptchaError> {
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
