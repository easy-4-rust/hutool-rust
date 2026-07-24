use rand::Rng as _;

use crate::{CaptchaError, CaptchaRenderer, RenderedCaptcha};

use super::audio_spec::AudioSpec;

/// Injected text-to-speech boundary for accessible audio CAPTCHA output.
pub trait AudioSynthesizer: Send + Sync {
    /// Synthesizes the supplied code as mono signed 16-bit PCM.
    fn synthesize(&self, code: &str, spec: AudioSpec) -> Result<Vec<i16>, CaptchaError>;
}
