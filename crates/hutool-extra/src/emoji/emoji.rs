//! Emoji helpers aligned with Hutool `cn.hutool.extra.emoji.EmojiUtil`.
//!
//! Backed by the [`emojis`] crate (GitHub gemoji shortcodes) rather than
//! emoji-java, with the same facade shapes Hutool callers expect.

use std::collections::BTreeSet;

/// Lightweight emoji metadata returned by lookup helpers.
///
/// Java: `com.vdurmont.emoji.Emoji` stand-in (unicode + shortcode only).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Emoji {
    pub(crate) unicode: String,
    pub(crate) shortcode: Option<String>,
    pub(crate) name: String,
}

impl Emoji {
    /// Returns the emoji unicode string.
    #[must_use]
    pub fn unicode(&self) -> &str {
        &self.unicode
    }

    /// Returns the primary GitHub shortcode when present.
    #[must_use]
    pub fn shortcode(&self) -> Option<&str> {
        self.shortcode.as_deref()
    }

    /// Returns the emoji CLDR / gemoji name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }
}

use super::{from_static, match_emoji_prefix, parse_html_codepoint, replace_emojis, skin_tone_type};
