use rand::Rng as _;

use crate::{CaptchaError, CaptchaRenderer, RenderedCaptcha};

use super::audio_spec::AudioSpec;
use super::audio_synthesizer::AudioSynthesizer;

/// WAV renderer that adds bounded low-amplitude noise to injected speech PCM.
#[derive(Debug, Clone)]
pub struct AudioRenderer<S> {
    synthesizer: S,
    spec: AudioSpec,
    noise_amplitude: i16,
}

impl<S> AudioRenderer<S> {
    /// Creates an audio renderer using an application-selected speech engine.
    pub fn new(
        synthesizer: S,
        sample_rate: u32,
        max_duration_seconds: u16,
    ) -> Result<Self, CaptchaError> {
        if !(8_000..=48_000).contains(&sample_rate) || !(1..=600).contains(&max_duration_seconds) {
            return Err(CaptchaError::MediaLimit("audio format"));
        }
        #[allow(clippy::cast_possible_truncation)]
        let max_samples = sample_rate as usize * usize::from(max_duration_seconds);
        Ok(Self {
            synthesizer,
            spec: AudioSpec {
                sample_rate,
                max_samples,
            },
            noise_amplitude: 160,
        })
    }

    /// Sets the absolute random-noise amplitude mixed into each sample.
    #[must_use]
    pub const fn with_noise_amplitude(mut self, amplitude: i16) -> Self {
        self.noise_amplitude = amplitude;
        self
    }
}

impl<S: AudioSynthesizer> CaptchaRenderer for AudioRenderer<S> {
    fn render(&self, code: &str) -> Result<RenderedCaptcha, CaptchaError> {
        if code.is_empty() || code.chars().count() > 32 {
            return Err(CaptchaError::InvalidRenderCode);
        }
        let mut samples = self.synthesizer.synthesize(code, self.spec)?;
        if samples.len() > self.spec.max_samples {
            return Err(CaptchaError::MediaLimit("audio sample count"));
        }
        let amplitude = self.noise_amplitude.unsigned_abs().min(i16::MAX as u16);
        if amplitude > 0 {
            let amplitude = i32::from(amplitude);
            let mut rng = rand::rng();
            for sample in &mut samples {
                let noise = rng.random_range(-amplitude..=amplitude);
                *sample = i16::try_from(
                    i32::from(*sample)
                        .saturating_add(noise)
                        .clamp(i32::from(i16::MIN), i32::from(i16::MAX)),
                )
                .expect("sample is clamped to i16 range");
            }
        }
        Ok(RenderedCaptcha::new(
            "audio/wav",
            encode_wav(&samples, self.spec.sample_rate),
        ))
    }
}

fn encode_wav(samples: &[i16], sample_rate: u32) -> Vec<u8> {
    #[allow(clippy::cast_possible_truncation)]
    let data_size = samples.len().saturating_mul(2) as u32;
    let riff_size = 36_u32 + data_size;
    let mut bytes = Vec::with_capacity(usize::try_from(riff_size).unwrap_or(0).saturating_add(8));
    bytes.extend_from_slice(b"RIFF");
    bytes.extend_from_slice(&riff_size.to_le_bytes());
    bytes.extend_from_slice(b"WAVEfmt ");
    bytes.extend_from_slice(&16_u32.to_le_bytes());
    bytes.extend_from_slice(&1_u16.to_le_bytes());
    bytes.extend_from_slice(&1_u16.to_le_bytes());
    bytes.extend_from_slice(&sample_rate.to_le_bytes());
    bytes.extend_from_slice(&sample_rate.saturating_mul(2).to_le_bytes());
    bytes.extend_from_slice(&2_u16.to_le_bytes());
    bytes.extend_from_slice(&16_u16.to_le_bytes());
    bytes.extend_from_slice(b"data");
    bytes.extend_from_slice(&data_size.to_le_bytes());
    for sample in samples {
        bytes.extend_from_slice(&sample.to_le_bytes());
    }
    bytes
}
