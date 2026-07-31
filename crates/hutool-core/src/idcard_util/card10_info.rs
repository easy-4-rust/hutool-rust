/// Parsed information returned for ten-character Taiwan, Macao, and Hong Kong cards.

/// 对齐: `cn.hutool.core.util.IdcardUtil.Card10Info`
/// 10位身份证信息
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Card10Info {
    pub(crate) region: &'static str,
    pub(crate) gender: char,
    pub(crate) valid: bool,
}

impl Card10Info {
    /// Returns the card's region name.
    #[must_use]
    pub const fn region(self) -> &'static str {
        self.region
    }

    /// Returns `M`, `F`, or `N`, matching Hutool's information array.
    #[must_use]
    pub const fn gender(self) -> char {
        self.gender
    }

    /// Returns whether the regional checksum or syntax is valid.
    #[must_use]
    pub const fn is_valid(self) -> bool {
        self.valid
    }
}

