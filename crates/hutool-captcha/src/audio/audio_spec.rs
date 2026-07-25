//! 对齐: Rust 扩展，音频规格定义
//! 来源: hutool-captcha/src/main/java/cn/hutool/captcha/ (扩展)
//! 中文说明: 语音合成器请求的 PCM 格式规格

use rand::Rng as _;

use crate::{CaptchaError, CaptchaRenderer, RenderedCaptcha};

/// PCM format requested from a speech synthesizer.
///
/// 对齐: Rust 扩展，Java Hutool 无直接对应
/// 中文说明: 语音合成器请求的 PCM 格式，包含采样率和最大采样数
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AudioSpec {
    /// Mono sample rate in hertz.
    pub sample_rate: u32,
    /// Maximum accepted sample count.
    pub max_samples: usize,
}
