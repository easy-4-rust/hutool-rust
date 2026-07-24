//! Emoji helpers aligned with Hutool `cn.hutool.extra.emoji.EmojiUtil`.
//!
//! Backed by the [`emojis`] crate (GitHub gemoji shortcodes) rather than
//! emoji-java, with the same facade shapes Hutool callers expect.

use std::collections::BTreeSet;

mod fitzpatrick_action;
mod emoji;
mod emoji_util;

pub use fitzpatrick_action::FitzpatrickAction;
pub use emoji::Emoji;
pub use emoji_util::EmojiUtil;

fn from_static(emoji: &'static emojis::Emoji) -> Emoji {
    Emoji {
        unicode: emoji.as_str().to_owned(),
        shortcode: emoji.shortcode().map(str::to_owned),
        name: emoji.name().to_owned(),
    }
}

fn parse_html_codepoint(body: &str) -> Option<u32> {
    if let Some(hex) = body
        .strip_prefix('x')
        .or_else(|| body.strip_prefix('X'))
    {
        u32::from_str_radix(hex, 16).ok()
    } else {
        body.parse().ok()
    }
}

fn skin_tone_type(tone: &str) -> Option<u8> {
    match tone {
        "\u{1F3FB}" => Some(1),
        "\u{1F3FC}" => Some(2),
        "\u{1F3FD}" => Some(3),
        "\u{1F3FE}" => Some(4),
        "\u{1F3FF}" => Some(5),
        // emoji-java type_6 is the darkest; gemoji uses the same fifth modifier
        // as type_5 — keep type_5 for the last modifier to stay within 1..5.
        _ => None,
    }
}

fn match_emoji_prefix(s: &str) -> Option<(&'static emojis::Emoji, usize)> {
    let mut best: Option<(&'static emojis::Emoji, usize)> = None;
    for (idx, _) in s.char_indices() {
        let end = idx + s[idx..].chars().next()?.len_utf8();
        let candidate = &s[..end];
        if let Some(emoji) = emojis::get(candidate) {
            best = Some((emoji, end));
        }
    }
    // Also try full string when it is an exact multi-codepoint emoji.
    if let Some(emoji) = emojis::get(s) {
        return Some((emoji, s.len()));
    }
    best
}

fn replace_emojis(s: &str, mut map: impl FnMut(&'static emojis::Emoji, &str) -> String) -> String {
    let mut out = String::with_capacity(s.len());
    let mut rest = s;
    while !rest.is_empty() {
        if let Some((emoji, consumed)) = match_emoji_prefix(rest) {
            let after = &rest[consumed..];
            // Optional single Fitzpatrick modifier after the base emoji.
            let (tone, tone_len) = match after.chars().next() {
                Some(c) if (0x1F3FB..=0x1F3FF).contains(&(c as u32)) => {
                    let len = c.len_utf8();
                    (&after[..len], len)
                }
                _ => ("", 0),
            };
            out.push_str(&map(emoji, tone));
            rest = &rest[consumed + tone_len..];
        } else {
            let mut chars = rest.chars();
            if let Some(c) = chars.next() {
                out.push(c);
            }
            rest = chars.as_str();
        }
    }
    out
}
