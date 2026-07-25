//! 对齐: Rust 扩展，音频验证码模块
//! 来源: hutool-captcha/src/main/java/cn/hutool/captcha/ (扩展)
//! 中文说明: 音频验证码模块，提供语音合成和 WAV 渲染功能

use rand::Rng as _;

use crate::{CaptchaError, CaptchaRenderer, RenderedCaptcha};

mod audio_spec;
mod audio_synthesizer;
mod audio_renderer;

pub use audio_spec::AudioSpec;
pub use audio_synthesizer::AudioSynthesizer;
pub use audio_renderer::AudioRenderer;
