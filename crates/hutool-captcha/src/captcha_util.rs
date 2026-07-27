//! `CaptchaUtil` 对象的独立模块。
//!
//! 对齐: `cn.hutool.captcha.CaptchaUtil`
//! 来源: `hutool-captcha/src/main/java/cn/hutool/captcha/CaptchaUtil.java`
//! 中文说明: 验证码工厂工具，提供各类 Hutool 风格验证码的便捷创建方法。

use crate::{CaptchaError, CircleCaptcha, GifCaptcha, LineCaptcha, ShearCaptcha};

/// Hutool 风格验证码工厂门面。
pub struct CaptchaUtil;

impl CaptchaUtil {
    /// 创建线条干扰验证码。
    pub fn create_line_captcha(width: u16, height: u16) -> Result<LineCaptcha, CaptchaError> {
        LineCaptcha::new(width, height)
    }

    /// 按指定字符数和干扰线条数创建线条干扰验证码。
    pub fn create_line_captcha_with_count(
        width: u16,
        height: u16,
        code_count: usize,
        line_count: u16,
    ) -> Result<LineCaptcha, CaptchaError> {
        LineCaptcha::with_code_count(width, height, code_count, line_count)
    }

    /// 按指定字体倍数创建线条干扰验证码。
    pub fn create_line_captcha_with_size(
        width: u16,
        height: u16,
        code_count: usize,
        line_count: u16,
        size: f32,
    ) -> Result<LineCaptcha, CaptchaError> {
        LineCaptcha::with_size(width, height, code_count, line_count, size)
    }

    /// 创建圆圈干扰验证码。
    pub fn create_circle_captcha(width: u16, height: u16) -> Result<CircleCaptcha, CaptchaError> {
        CircleCaptcha::new(width, height)
    }

    /// 按指定字符数和干扰圆圈数量创建圆圈干扰验证码。
    pub fn create_circle_captcha_with_count(
        width: u16,
        height: u16,
        code_count: usize,
        circle_count: u16,
    ) -> Result<CircleCaptcha, CaptchaError> {
        CircleCaptcha::with_code_count(width, height, code_count, circle_count)
    }

    /// 按指定字体倍数创建圆圈干扰验证码。
    pub fn create_circle_captcha_with_size(
        width: u16,
        height: u16,
        code_count: usize,
        circle_count: u16,
        size: f32,
    ) -> Result<CircleCaptcha, CaptchaError> {
        CircleCaptcha::with_size(width, height, code_count, circle_count, size)
    }

    /// 创建扭曲干扰验证码。
    pub fn create_shear_captcha(width: u16, height: u16) -> Result<ShearCaptcha, CaptchaError> {
        ShearCaptcha::new(width, height)
    }

    /// 按指定字符数和干扰宽度创建扭曲干扰验证码。
    pub fn create_shear_captcha_with_count(
        width: u16,
        height: u16,
        code_count: usize,
        thickness: u16,
    ) -> Result<ShearCaptcha, CaptchaError> {
        ShearCaptcha::with_code_count(width, height, code_count, thickness)
    }

    /// 按指定字体倍数创建扭曲干扰验证码。
    pub fn create_shear_captcha_with_size(
        width: u16,
        height: u16,
        code_count: usize,
        thickness: u16,
        size: f32,
    ) -> Result<ShearCaptcha, CaptchaError> {
        ShearCaptcha::with_size(width, height, code_count, thickness, size)
    }

    /// 创建 GIF 验证码。
    pub fn create_gif_captcha(width: u16, height: u16) -> Result<GifCaptcha, CaptchaError> {
        GifCaptcha::new(width, height)
    }

    /// 按指定字符数创建 GIF 验证码。
    pub fn create_gif_captcha_with_count(
        width: u16,
        height: u16,
        code_count: usize,
    ) -> Result<GifCaptcha, CaptchaError> {
        GifCaptcha::with_code_count(width, height, code_count, 10)
    }

    /// 按指定字体倍数创建 GIF 验证码。
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
