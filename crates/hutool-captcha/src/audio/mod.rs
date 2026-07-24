use rand::Rng as _;

use crate::{CaptchaError, CaptchaRenderer, RenderedCaptcha};

mod audio_spec;
mod audio_synthesizer;
mod audio_renderer;

pub use audio_spec::AudioSpec;
pub use audio_synthesizer::AudioSynthesizer;
pub use audio_renderer::AudioRenderer;
