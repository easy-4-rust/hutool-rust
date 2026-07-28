//! CAPTCHA code generation and verification primitives.
//!
//! 对齐: `cn.hutool.captcha` (crate root)
//! 来源: hutool-captcha/src/main/java/cn/hutool/captcha/
//! 中文说明: 验证码生成与验证模块，提供多种验证码类型和渲染方式

#![forbid(unsafe_code)]

use rand::RngExt;
use std::fmt::Write as _;
use std::time::{Duration, Instant};
use thiserror::Error;

pub mod prelude;

#[cfg(feature = "audio")]
mod audio;
#[cfg(feature = "raster")]
mod abstract_captcha;
#[cfg(feature = "raster")]
mod captcha_util;
#[cfg(feature = "raster")]
mod circle_captcha;
#[cfg(feature = "raster")]
mod compat;
#[cfg(feature = "raster")]
mod generator;
#[cfg(feature = "raster")]
mod gif_captcha;
#[cfg(feature = "raster")]
mod icaptcha;
#[cfg(feature = "raster")]
mod line_captcha;
#[cfg(feature = "raster")]
mod raster;
#[cfg(feature = "raster")]
mod shear_captcha;

#[cfg(feature = "audio")]
pub use audio::{AudioRenderer, AudioSpec, AudioSynthesizer};
#[cfg(feature = "raster")]
pub use abstract_captcha::AbstractCaptcha;
#[cfg(feature = "raster")]
pub use captcha_util::CaptchaUtil;
#[cfg(feature = "raster")]
pub use circle_captcha::CircleCaptcha;
#[cfg(feature = "raster")]
pub use gif_captcha::GifCaptcha;
#[cfg(feature = "raster")]
pub use compat::{CaptchaColor, CaptchaFont, CaptchaStroke};
#[cfg(feature = "raster")]
pub use generator::{AbstractGenerator, MathGenerator, RandomGenerator};
#[cfg(feature = "raster")]
pub use icaptcha::ICaptcha;
#[cfg(feature = "raster")]
pub use line_captcha::LineCaptcha;
#[cfg(feature = "raster")]
pub use raster::PngRenderer;
#[cfg(feature = "raster")]
pub use shear_captcha::ShearCaptcha;

const DEFAULT_ALPHABET: &[u8] = b"23456789ABCDEFGHJKLMNPQRSTUVWXYZ";

/// A strategy for generating and comparing CAPTCHA codes.
///
/// 对齐: `cn.hutool.captcha.ICodeGenerator`
/// 中文说明: 验证码生成与比对策略接口
pub trait CodeGenerator: Send + Sync {
    /// Generates a new challenge code.
    fn generate(&self) -> String;

    /// Compares a generated code with user input.
    fn verify(&self, generated: &str, input: &str) -> bool;
}

/// Media produced by a CAPTCHA renderer.
///
/// 对齐: Rust 扩展，Java Hutool 无直接对应
/// 中文说明: 验证码渲染器生成的媒体产物，包含 MIME 类型和编码字节
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RenderedCaptcha {
    mime_type: &'static str,
    bytes: Vec<u8>,
}

impl RenderedCaptcha {
    #[cfg(any(feature = "audio", feature = "raster"))]
    fn new(mime_type: &'static str, bytes: Vec<u8>) -> Self {
        Self { mime_type, bytes }
    }

    /// Returns the Internet media type of the payload.
    #[must_use]
    pub const fn mime_type(&self) -> &'static str {
        self.mime_type
    }

    /// Returns the encoded media payload.
    #[must_use]
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Consumes the artifact and returns its media payload.
    #[must_use]
    pub fn into_bytes(self) -> Vec<u8> {
        self.bytes
    }
}

/// Converts a generated code into a client-facing challenge.
///
/// 对齐: `cn.hutool.captcha.ICaptcha`
/// 中文说明: 将生成的验证码转换为客户端可见挑战的渲染器接口
pub trait CaptchaRenderer: Send + Sync {
    /// Renders a code.
    fn render(&self, code: &str) -> Result<RenderedCaptcha, CaptchaError>;
}

/// Dependency-free SVG renderer with randomized glyph placement and noise.
///
/// SVG challenges are convenient for web applications, but callers facing a
/// high-abuse environment should combine them with rate limits and a stronger
/// proof-of-human mechanism. The plaintext code must never be logged or sent
/// separately from the rendered artifact.
///
/// 对齐: Rust 扩展，Java Hutool 无直接 SVG 渲染器
/// 中文说明: 无依赖 SVG 验证码渲染器，支持随机字形放置和干扰线
#[derive(Debug, Clone)]
pub struct SvgRenderer {
    width: u16,
    height: u16,
    noise_lines: u8,
}

impl SvgRenderer {
    /// Creates an SVG renderer.
    pub fn new(width: u16, height: u16) -> Result<Self, CaptchaError> {
        if width < 80 || height < 30 {
            return Err(CaptchaError::InvalidDimensions);
        }
        Ok(Self {
            width,
            height,
            noise_lines: 6,
        })
    }

    /// Sets the number of randomized interference lines.
    #[must_use]
    pub const fn with_noise_lines(mut self, noise_lines: u8) -> Self {
        self.noise_lines = noise_lines;
        self
    }
}

impl Default for SvgRenderer {
    fn default() -> Self {
        Self {
            width: 160,
            height: 54,
            noise_lines: 6,
        }
    }
}

impl CaptchaRenderer for SvgRenderer {
    fn render(&self, code: &str) -> Result<RenderedCaptcha, CaptchaError> {
        if code.is_empty() || code.chars().count() > 32 {
            return Err(CaptchaError::InvalidRenderCode);
        }

        let mut rng = rand::rng();
        let mut svg = format!(
            r##"<svg xmlns="http://www.w3.org/2000/svg" width="{}" height="{}" viewBox="0 0 {} {}" role="img" aria-label="CAPTCHA"><rect width="100%" height="100%" rx="6" fill="#f4f6f8"/>"##,
            self.width, self.height, self.width, self.height
        );
        for _ in 0..self.noise_lines {
            let (x1, x2) = (
                rng.random_range(0..self.width),
                rng.random_range(0..self.width),
            );
            let (y1, y2) = (
                rng.random_range(0..self.height),
                rng.random_range(0..self.height),
            );
            let hue = rng.random_range(0..360);
            write!(
                svg,
                r#"<line x1="{x1}" y1="{y1}" x2="{x2}" y2="{y2}" stroke="hsl({hue} 55% 45%)" stroke-width="1.5" opacity=".55"/>"#
            )
            .expect("writing to a String cannot fail");
        }

        #[allow(clippy::cast_possible_truncation)]
        let glyph_count = code.chars().count() as u16;
        let glyphs = f32::from(glyph_count);
        let step = f32::from(self.width) / (glyphs + 1.0);
        let baseline = f32::from(self.height) * 0.7;
        let font_size = (f32::from(self.height) * 0.56).clamp(18.0, 42.0);
        for (index, glyph) in code.chars().enumerate() {
            let index =
                u16::try_from(index).expect("render code length is bounded to 32 characters");
            let x = step * (f32::from(index) + 1.0) + rng.random_range(-2.5..2.5);
            let y = baseline + rng.random_range(-4.0..4.0);
            let rotation = rng.random_range(-18..=18);
            let hue = rng.random_range(0..360);
            let glyph = escape_xml_char(glyph);
            write!(
                svg,
                r#"<text x="{x:.1}" y="{y:.1}" text-anchor="middle" transform="rotate({rotation} {x:.1} {y:.1})" font-family="ui-monospace,monospace" font-size="{font_size:.1}" font-weight="700" fill="hsl({hue} 65% 28%)">{glyph}</text>"#
            )
            .expect("writing to a String cannot fail");
        }
        svg.push_str("</svg>");
        Ok(RenderedCaptcha {
            mime_type: "image/svg+xml",
            bytes: svg.into_bytes(),
        })
    }
}

fn escape_xml_char(value: char) -> String {
    match value {
        '&' => "&amp;".to_owned(),
        '<' => "&lt;".to_owned(),
        '>' => "&gt;".to_owned(),
        '\"' => "&quot;".to_owned(),
        '\'' => "&apos;".to_owned(),
        value => value.to_string(),
    }
}

/// Generates unambiguous uppercase ASCII codes.
///
/// 对齐: `cn.hutool.captcha.generator.RandomGenerator`
/// 中文说明: 生成无歧义大写字母数字验证码
#[derive(Debug, Clone)]
pub struct AlphanumericGenerator {
    length: usize,
    alphabet: Vec<u8>,
}

impl AlphanumericGenerator {
    /// Creates a generator with an alphabet that excludes ambiguous glyphs.
    ///
    /// # Panics
    ///
    /// Panics when `length` is zero.
    #[must_use]
    pub fn new(length: usize) -> Self {
        assert!(length > 0, "CAPTCHA length must be greater than zero");
        Self {
            length,
            alphabet: DEFAULT_ALPHABET.to_vec(),
        }
    }

    /// Creates a generator with an explicit non-empty ASCII alphabet.
    pub fn with_alphabet(
        length: usize,
        alphabet: impl Into<Vec<u8>>,
    ) -> Result<Self, CaptchaError> {
        let alphabet = alphabet.into();
        if length == 0 {
            return Err(CaptchaError::InvalidLength);
        }
        if alphabet.is_empty() || !alphabet.is_ascii() {
            return Err(CaptchaError::InvalidAlphabet);
        }
        Ok(Self { length, alphabet })
    }
}

impl Default for AlphanumericGenerator {
    fn default() -> Self {
        Self::new(5)
    }
}

impl CodeGenerator for AlphanumericGenerator {
    fn generate(&self) -> String {
        let mut rng = rand::rng();
        (0..self.length)
            .map(|_| {
                let index = rng.random_range(0..self.alphabet.len());
                char::from(self.alphabet[index])
            })
            .collect()
    }

    fn verify(&self, generated: &str, input: &str) -> bool {
        constant_time_ascii_case_eq(generated.as_bytes(), input.trim().as_bytes())
    }
}

/// CAPTCHA creation and verification errors.
///
/// 对齐: Rust 扩展，Java Hutool 无直接对应
/// 中文说明: 验证码创建和验证过程中的错误类型
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum CaptchaError {
    /// Code length must be positive.
    #[error("CAPTCHA length must be greater than zero")]
    InvalidLength,
    /// Alphabet must contain at least one ASCII byte.
    #[error("CAPTCHA alphabet must be non-empty ASCII")]
    InvalidAlphabet,
    /// Render dimensions are too small to produce a usable challenge.
    #[error("CAPTCHA dimensions must be at least 80x30")]
    InvalidDimensions,
    /// Render input must contain between one and 32 Unicode scalar values.
    #[error("CAPTCHA render code must contain between 1 and 32 characters")]
    InvalidRenderCode,
    /// Raster image encoding failed.
    #[cfg(feature = "raster")]
    #[error("CAPTCHA image encoding failed: {0}")]
    Image(String),
    /// Raster or audio output exceeded defensive resource limits.
    #[error("CAPTCHA media limit exceeded: {0}")]
    MediaLimit(&'static str),
    /// An injected speech synthesizer rejected the challenge.
    #[cfg(feature = "audio")]
    #[error("CAPTCHA speech synthesis failed: {0}")]
    SpeechSynthesis(String),
    /// The challenge has expired.
    #[error("CAPTCHA challenge has expired")]
    Expired,
    /// User input does not match the challenge.
    #[error("CAPTCHA code does not match")]
    Mismatch,
    /// Filesystem or stream output failed.
    #[cfg(feature = "raster")]
    #[error("CAPTCHA I/O failed: {0}")]
    Io(String),
}

#[cfg(feature = "raster")]
impl From<image::ImageError> for CaptchaError {
    fn from(error: image::ImageError) -> Self {
        Self::Image(error.to_string())
    }
}

/// A generated code paired with an expiration deadline.
///
/// 对齐: Rust 扩展，Java Hutool 无直接对应
/// 中文说明: 带过期时间的验证码挑战，封装生成码和验证逻辑
pub struct CaptchaChallenge {
    code: String,
    expires_at: Instant,
}

impl CaptchaChallenge {
    /// Generates a challenge with a fixed validity duration.
    #[must_use]
    pub fn generate(generator: &dyn CodeGenerator, valid_for: Duration) -> Self {
        Self {
            code: generator.generate(),
            expires_at: Instant::now() + valid_for,
        }
    }

    /// Returns the code for an image/audio renderer.
    #[must_use]
    pub fn code(&self) -> &str {
        &self.code
    }

    /// Verifies user input and expiration.
    pub fn verify(&self, generator: &dyn CodeGenerator, input: &str) -> Result<(), CaptchaError> {
        if Instant::now() >= self.expires_at {
            return Err(CaptchaError::Expired);
        }
        if !generator.verify(&self.code, input) {
            return Err(CaptchaError::Mismatch);
        }
        Ok(())
    }
}

fn constant_time_ascii_case_eq(left: &[u8], right: &[u8]) -> bool {
    if left.len() != right.len() {
        return false;
    }
    left.iter()
        .zip(right)
        .fold(0_u8, |difference, (left, right)| {
            difference | left.to_ascii_uppercase() ^ right.to_ascii_uppercase()
        })
        == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_requested_length_and_verifies_case_insensitively() {
        let generator = AlphanumericGenerator::new(6);
        let code = generator.generate();
        assert_eq!(code.len(), 6);
        assert!(generator.verify(&code, &code.to_ascii_lowercase()));
        assert!(!generator.verify(&code, "wrong"));
        assert_eq!(
            AlphanumericGenerator::with_alphabet(0, b"A".to_vec()).unwrap_err(),
            CaptchaError::InvalidLength
        );
        assert_eq!(
            AlphanumericGenerator::with_alphabet(2, Vec::new()).unwrap_err(),
            CaptchaError::InvalidAlphabet
        );
        assert_eq!(
            AlphanumericGenerator::with_alphabet(2, vec![0xff]).unwrap_err(),
            CaptchaError::InvalidAlphabet
        );
        let fixed = AlphanumericGenerator::with_alphabet(2, b"Z".to_vec()).unwrap();
        assert_eq!(fixed.generate(), "ZZ");
    }

    #[test]
    fn challenge_enforces_deadline() {
        let generator = AlphanumericGenerator::default();
        let challenge = CaptchaChallenge::generate(&generator, Duration::ZERO);
        assert_eq!(
            challenge.verify(&generator, challenge.code()),
            Err(CaptchaError::Expired)
        );
    }

    #[test]
    fn challenge_covers_success_and_mismatch() {
        let generator = AlphanumericGenerator::with_alphabet(2, b"Z".to_vec()).unwrap();
        let challenge = CaptchaChallenge::generate(&generator, Duration::from_secs(60));
        assert_eq!(challenge.verify(&generator, "zz"), Ok(()));
        assert_eq!(
            challenge.verify(&generator, "no"),
            Err(CaptchaError::Mismatch)
        );
    }

    #[test]
    fn svg_renderer_escapes_code_and_returns_media_metadata() {
        let artifact = SvgRenderer::default().render("A<&").unwrap();
        let svg = std::str::from_utf8(artifact.bytes()).unwrap();
        assert_eq!(artifact.mime_type(), "image/svg+xml");
        assert!(svg.starts_with("<svg"));
        assert!(svg.contains("&lt;"));
        assert!(svg.contains("&amp;"));
        assert!(!svg.contains("><</text>"));
        let escaped = SvgRenderer::new(100, 30)
            .unwrap()
            .with_noise_lines(0)
            .render(">\"'")
            .unwrap();
        let escaped = std::str::from_utf8(escaped.bytes()).unwrap();
        assert!(escaped.contains("&gt;"));
        assert!(escaped.contains("&quot;"));
        assert!(escaped.contains("&apos;"));
    }

    #[test]
    fn svg_renderer_rejects_unusable_inputs() {
        assert_eq!(
            SvgRenderer::new(79, 30).unwrap_err(),
            CaptchaError::InvalidDimensions
        );
        assert_eq!(
            SvgRenderer::default().render(""),
            Err(CaptchaError::InvalidRenderCode)
        );
        assert_eq!(
            SvgRenderer::default().render(&"A".repeat(33)),
            Err(CaptchaError::InvalidRenderCode)
        );
        let bytes = SvgRenderer::default().render("A").unwrap().into_bytes();
        assert!(bytes.starts_with(b"<svg"));

        #[cfg(feature = "raster")]
        {
            let image_error = image::load_from_memory(b"not-an-image")
                .map_err(CaptchaError::from)
                .unwrap_err();
            assert!(image_error.to_string().contains("image encoding failed"));
        }
    }
}
