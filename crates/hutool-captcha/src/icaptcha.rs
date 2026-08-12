//! `ICaptcha` 对象的独立模块。
//!
//! 对齐: `cn.hutool.captcha.ICaptcha`
//! 来源: `hutool-captcha/src/main/java/cn/hutool/captcha/ICaptcha.java`
//! 中文说明: 所有 Hutool 风格验证码变体的通用操作接口，统一暴露生成、取码、校验与导出图片字节能力。

use crate::CaptchaError;

/// Hutool 风格验证码对象的统一接口。
///
/// 对齐 Java `ICaptcha`：保留创建验证码、获取验证码文本、校验输入和写出图片的核心能力。
pub trait ICaptcha {
    /// 创建新的验证码挑战。
    fn create_code(&mut self) -> Result<(), CaptchaError>;

    /// 返回当前验证码文本；尚未生成时应触发懒加载。
    fn code(&mut self) -> Result<&str, CaptchaError>;

    /// 校验用户输入是否与当前验证码一致。
    fn verify(&self, input: &str) -> bool;

    /// 返回已经编码好的图片或 GIF 字节。
    fn image_bytes(&mut self) -> Result<&[u8], CaptchaError>;
}
