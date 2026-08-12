//! `ShearCaptcha` 对象的独立模块。
//!
//! 对齐: `cn.hutool.captcha.ShearCaptcha`
//! 来源: `hutool-captcha/src/main/java/cn/hutool/captcha/ShearCaptcha.java`
//! 中文说明: 扭曲干扰验证码类型，复用 `AbstractCaptcha` 的共享渲染能力并保留 Hutool 风格构造入口。

use std::ops::{Deref, DerefMut};
use std::sync::Arc;

use image::DynamicImage;

use crate::compat::CaptchaKind;
use crate::generator::RandomGenerator;
use crate::{AbstractCaptcha, CaptchaError, CodeGenerator};

/// Hutool 兼容的扭曲干扰验证码。
///
/// 对齐 Java `ShearCaptcha`：默认 5 位验证码和宽度为 4 的扭曲干扰。
#[derive(Debug)]
pub struct ShearCaptcha(pub(crate) AbstractCaptcha);

impl ShearCaptcha {
    /// 创建默认配置的扭曲干扰验证码。
    pub fn new(width: u16, height: u16) -> Result<Self, CaptchaError> {
        Self::with_code_count(width, height, 5, 4)
    }

    /// 按指定字符数和扭曲干扰宽度创建验证码。
    pub fn with_code_count(
        width: u16,
        height: u16,
        code_count: usize,
        thickness: u16,
    ) -> Result<Self, CaptchaError> {
        let generator = Arc::new(RandomGenerator::new(code_count)?);
        Self::with_generator(width, height, generator, thickness)
    }

    /// 沿用 Java 三参构造习惯，仅指定字符数时使用默认干扰宽度。
    pub fn with_count(width: u16, height: u16, code_count: usize) -> Result<Self, CaptchaError> {
        Self::with_code_count(width, height, code_count, 4)
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
            CaptchaKind::Shear,
        )
        .map(Self)
    }

    /// 使用 Hutool 风格的字体大小倍数创建验证码。
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
            CaptchaKind::Shear,
        )
        .map(Self)
    }

    /// 仅根据传入验证码文本即时生成图像，不修改当前挑战状态。
    pub fn create_image(&self, code: &str) -> Result<DynamicImage, CaptchaError> {
        let rendered = self.0.render(code)?;
        Ok(image::load_from_memory(rendered.bytes())
            .expect("the compatibility renderer emits valid PNG or GIF bytes"))
    }
}

impl Deref for ShearCaptcha {
    type Target = AbstractCaptcha;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ShearCaptcha {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
