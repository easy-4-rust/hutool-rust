//! 对齐: Rust 扩展，音频验证码模块
//! 来源: hutool-captcha/src/main/java/cn/hutool/captcha/ (扩展)
//! 中文说明: 音频验证码模块，提供语音合成和 WAV 渲染功能


mod audio_renderer;
mod audio_spec;
mod audio_synthesizer;

pub use audio_renderer::AudioRenderer;
pub use audio_spec::AudioSpec;
pub use audio_synthesizer::AudioSynthesizer;
