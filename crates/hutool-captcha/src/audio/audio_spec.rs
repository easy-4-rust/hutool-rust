use rand::Rng as _;

use crate::{CaptchaError, CaptchaRenderer, RenderedCaptcha};

/// PCM format requested from a speech synthesizer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AudioSpec {
    /// Mono sample rate in hertz.
    pub sample_rate: u32,
    /// Maximum accepted sample count.
    pub max_samples: usize,
}
