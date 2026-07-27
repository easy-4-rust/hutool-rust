//! `LineCaptcha` 对象的独立模块。
//!
//! 对齐: `cn.hutool.captcha.LineCaptcha`
//! 来源: `hutool-captcha/src/main/java/cn/hutool/captcha/LineCaptcha.java`
//! 中文说明: 线条干扰验证码类型，对应 Hutool `LineCaptcha` 的构造和渲染入口。

use std::ops::{Deref, DerefMut};
use std::sync::Arc;

use image::DynamicImage;

use crate::compat::CaptchaKind;
use crate::generator::RandomGenerator;
use crate::{AbstractCaptcha, CaptchaError, CodeGenerator};

/// Hutool 兼容的线条干扰验证码。
///
/// 对齐 Java `LineCaptcha`：默认 5 位验证码和 150 条干扰线。
#[derive(Debug)]
pub struct LineCaptcha(pub(crate) AbstractCaptcha);

impl LineCaptcha {
    /// 创建默认配置的线条干扰验证码。
    pub fn new(width: u16, height: u16) -> Result<Self, CaptchaError> {
        Self::with_code_count(width, height, 5, 150)
    }

    /// 按指定字符数和干扰线条数创建验证码。
    pub fn with_code_count(
        width: u16,
        height: u16,
        code_count: usize,
        line_count: u16,
    ) -> Result<Self, CaptchaError> {
        let generator = Arc::new(RandomGenerator::new(code_count)?);
        Self::with_generator(width, height, generator, line_count)
    }

    /// 注入自定义验证码生成器。
    pub fn with_generator(
        width: u16,
        height: u16,
        generator: Arc<dyn CodeGenerator>,
        interference: u16,
    ) -> Result<Self, CaptchaError> {
        AbstractCaptcha::new(
            width,
            height,
            generator,
            interference,
            0.75,
            CaptchaKind::Line,
        )
        .map(Self)
    }

    /// 使用 Hutool 风格的字体大小倍数创建验证码。
    pub fn with_size(
        width: u16,
        height: u16,
        code_count: usize,
        line_count: u16,
        size: f32,
    ) -> Result<Self, CaptchaError> {
        let generator = Arc::new(RandomGenerator::new(code_count)?);
        AbstractCaptcha::new(
            width,
            height,
            generator,
            line_count,
            size,
            CaptchaKind::Line,
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

impl Deref for LineCaptcha {
    type Target = AbstractCaptcha;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for LineCaptcha {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
