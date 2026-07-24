//! Emoji helpers aligned with Hutool `cn.hutool.extra.emoji.EmojiUtil`.
//!
//! Backed by the [`emojis`] crate (GitHub gemoji shortcodes) rather than
//! emoji-java, with the same facade shapes Hutool callers expect.

use std::collections::BTreeSet;

/// How Fitzpatrick skin-tone modifiers are handled when converting to aliases.
///
/// Java: `com.vdurmont.emoji.EmojiParser.FitzpatrickAction`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FitzpatrickAction {
    /// Append `|type_N` after the shortcode (Hutool default).
    #[default]
    Parse,
    /// Drop the skin-tone modifier from the alias output.
    Remove,
    /// Keep the raw modifier characters after the shortcode.
    Ignore,
}

use super::{from_static, match_emoji_prefix, parse_html_codepoint, replace_emojis, skin_tone_type};
