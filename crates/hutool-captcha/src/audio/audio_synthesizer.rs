//! 对齐: Rust 扩展，音频合成器接口
//! 来源: hutool-captcha/src/main/java/cn/hutool/captcha/ (扩展)
//! 中文说明: 文本转语音接口，用于无障碍音频验证码输出

use super::audio_spec::AudioSpec;
use crate::CaptchaError;

/// Injected text-to-speech boundary for accessible audio CAPTCHA output.
///
/// 对齐: Rust 扩展，Java Hutool 无直接对应
/// 中文说明: 注入式文本转语音接口，用于无障碍音频验证码输出
pub trait AudioSynthesizer: Send + Sync {
    /// Synthesizes the supplied code as mono signed 16-bit PCM.
    fn synthesize(&self, code: &str, spec: AudioSpec) -> Result<Vec<i16>, CaptchaError>;
}
