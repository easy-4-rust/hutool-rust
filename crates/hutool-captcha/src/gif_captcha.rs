//! `GifCaptcha` 对象的独立模块。
//!
//! 对齐: `cn.hutool.captcha.GifCaptcha`
//! 来源: `hutool-captcha/src/main/java/cn/hutool/captcha/GifCaptcha.java`
//! 中文说明: GIF 动态验证码类型，复用 `AbstractCaptcha` 的帧渲染与 GIF 编码能力，并保留 Hutool 风格配置项。

use std::ops::{Deref, DerefMut};
use std::sync::Arc;

use image::DynamicImage;
use rand::Rng;

use crate::compat::{CaptchaColor, CaptchaKind};
use crate::generator::RandomGenerator;
use crate::{AbstractCaptcha, CaptchaError, CodeGenerator};

/// Hutool 兼容的 GIF 动态验证码。
///
/// 对齐 Java `GifCaptcha`：默认 5 位验证码和 10 个干扰元素。
#[derive(Debug)]
pub struct GifCaptcha(pub(crate) AbstractCaptcha);

impl GifCaptcha {
    /// 创建默认配置的 GIF 验证码。
    pub fn new(width: u16, height: u16) -> Result<Self, CaptchaError> {
        Self::with_code_count(width, height, 5, 10)
    }

    /// 按指定字符数和干扰元素数量创建 GIF 验证码。
    pub fn with_code_count(
        width: u16,
        height: u16,
        code_count: usize,
        interfere_count: u16,
    ) -> Result<Self, CaptchaError> {
        let generator = Arc::new(RandomGenerator::new(code_count)?);
        Self::with_generator(width, height, generator, interfere_count)
    }

    /// 注入自定义验证码生成器。
    pub fn with_generator(
        width: u16,
        height: u16,
        generator: Arc<dyn CodeGenerator>,
        interfere_count: u16,
    ) -> Result<Self, CaptchaError> {
        AbstractCaptcha::new(
            width,
            height,
            generator,
            interfere_count,
            0.75,
            CaptchaKind::Gif,
        )
        .map(Self)
    }

    /// 使用 Hutool 风格的字体大小倍数创建 GIF 验证码。
    pub fn with_size(
        width: u16,
        height: u16,
        code_count: usize,
        interfere_count: u16,
        size: f32,
    ) -> Result<Self, CaptchaError> {
        let generator = Arc::new(RandomGenerator::new(code_count)?);
        AbstractCaptcha::new(
            width,
            height,
            generator,
            interfere_count,
            size,
            CaptchaKind::Gif,
        )
        .map(Self)
    }

    /// 仅根据传入验证码文本即时生成图像，不修改当前挑战状态。
    pub fn create_image(&self, code: &str) -> Result<DynamicImage, CaptchaError> {
        let rendered = self.0.render(code)?;
        Ok(image::load_from_memory(rendered.bytes())
            .expect("the compatibility renderer emits valid PNG or GIF bytes"))
    }

    /// 设置 GIF 颜色量化采样间隔。
    ///
    /// 对齐 Java `GifCaptcha.setQuality`，小于 1 时钳制为 1。
    #[must_use]
    pub fn set_quality(mut self, quality: u8) -> Self {
        self.0.gif_quality = quality.clamp(1, 30);
        self.0.invalidate();
        self
    }

    /// 返回 GIF 颜色量化质量配置。
    #[must_use]
    pub const fn quality(&self) -> u8 {
        self.0.gif_quality
    }

    /// 设置 GIF 帧循环次数，`0` 表示无限循环。
    ///
    /// 对齐 Java `GifCaptcha.setRepeat`，负数在 Java 中会被提升为 `0`。
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

    /// 返回 GIF 帧循环次数。
    #[must_use]
    pub const fn repeat(&self) -> u16 {
        self.0.gif_repeat
    }

    /// 设置随机文字颜色的上界。
    #[must_use]
    pub fn set_max_color(mut self, maximum: u8) -> Self {
        self.0.max_color = maximum;
        self.0.invalidate();
        self
    }

    /// 返回随机文字颜色的上界。
    #[must_use]
    pub const fn max_color(&self) -> u8 {
        self.0.max_color
    }

    /// 设置随机文字颜色的下界。
    #[must_use]
    pub fn set_min_color(mut self, minimum: u8) -> Self {
        self.0.min_color = minimum;
        self.0.invalidate();
        self
    }

    /// 返回随机文字颜色的下界。
    #[must_use]
    pub const fn min_color(&self) -> u8 {
        self.0.min_color
    }

    /// 在给定范围内采样随机 RGB 颜色。
    ///
    /// 对齐 Java `GifCaptcha.getRandomColor`：上下界越界时会回落到有效范围。
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

impl Deref for GifCaptcha {
    type Target = AbstractCaptcha;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for GifCaptcha {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
