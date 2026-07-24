use std::sync::LazyLock;

use regex::Regex;

const MOBILE_PATTERN: &str = r"(?:0|86|\+86)?1[3-9][0-9]{9}";
const MOBILE_HK_PATTERN: &str = r"(?:0|852|\+852)?[0-9]{8}";
const MOBILE_TW_PATTERN: &str = r"(?:0|886|\+886)?(?:|-)?09[0-9]{8}";
const MOBILE_MO_PATTERN: &str = r"(?:0|853|\+853)?(?:|-)?6[0-9]{7}";
const TEL_PATTERN: &str = r"(010|02[0-9]|0[3-9][0-9]{2})-?([0-9]{6,8})";
const TEL_400_800_PATTERN: &str =
    r"(?:0\d{2,3}[\- ]?[0-9]\d{6,7}|[48]00[\- ]?[0-9]\d{2}[\- ]?\d{4})";

static MOBILE: LazyLock<Regex> = LazyLock::new(|| full_regex(MOBILE_PATTERN));
static MOBILE_HK: LazyLock<Regex> = LazyLock::new(|| full_regex(MOBILE_HK_PATTERN));
static MOBILE_TW: LazyLock<Regex> = LazyLock::new(|| full_regex(MOBILE_TW_PATTERN));
static MOBILE_MO: LazyLock<Regex> = LazyLock::new(|| full_regex(MOBILE_MO_PATTERN));
static TEL: LazyLock<Regex> = LazyLock::new(|| full_regex(TEL_PATTERN));
static TEL_400_800: LazyLock<Regex> = LazyLock::new(|| full_regex(TEL_400_800_PATTERN));

fn full_regex(pattern: &str) -> Regex {
    Regex::new(&format!(r"^(?:{pattern})$"))
        .expect("HiTool phone patterns are compile-time constants")
}

/// Convenient Hutool-compatible validation and slicing for Chinese phone numbers.

/// 对齐: `cn.hutool.core.util.PhoneUtil`
/// 手机号工具类
pub struct PhoneUtil;

impl PhoneUtil {
    /// Validates a mainland China mobile number.
    #[must_use]
    pub fn is_mobile(value: &str) -> bool {
        MOBILE.is_match(value)
    }

    /// Validates a Hong Kong mobile number.
    #[must_use]
    pub fn is_mobile_hk(value: &str) -> bool {
        MOBILE_HK.is_match(value)
    }

    /// Validates a Taiwan mobile number.
    #[must_use]
    pub fn is_mobile_tw(value: &str) -> bool {
        MOBILE_TW.is_match(value)
    }

    /// Validates a Macao mobile number.
    #[must_use]
    pub fn is_mobile_mo(value: &str) -> bool {
        MOBILE_MO.is_match(value)
    }

    /// Validates a mainland China landline number.
    #[must_use]
    pub fn is_tel(value: &str) -> bool {
        TEL.is_match(value)
    }

    /// Validates a mainland landline, 400 or 800 service number.
    #[must_use]
    pub fn is_tel_400_800(value: &str) -> bool {
        TEL_400_800.is_match(value)
    }

    /// Validates any phone category supported by Hutool's `PhoneUtil`.
    #[must_use]
    pub fn is_phone(value: &str) -> bool {
        Self::is_mobile(value)
            || Self::is_tel_400_800(value)
            || Self::is_mobile_hk(value)
            || Self::is_mobile_tw(value)
            || Self::is_mobile_mo(value)
    }

    /// Hides the first seven characters.
    #[must_use]
    pub fn hide_before(phone: &str) -> String {
        hide(phone, 0, 7)
    }

    /// Hides characters three through six.
    #[must_use]
    pub fn hide_between(phone: &str) -> String {
        hide(phone, 3, 7)
    }

    /// Hides characters seven through ten.
    #[must_use]
    pub fn hide_after(phone: &str) -> String {
        hide(phone, 7, 11)
    }

    /// Returns the first three characters.
    #[must_use]
    pub fn sub_before(phone: &str) -> String {
        substring(phone, 0, 3)
    }

    /// Returns characters three through six.
    #[must_use]
    pub fn sub_between(phone: &str) -> String {
        substring(phone, 3, 7)
    }

    /// Returns characters seven through ten.
    #[must_use]
    pub fn sub_after(phone: &str) -> String {
        substring(phone, 7, 11)
    }

    /// Extracts the area code from a valid mainland landline.
    #[must_use]
    pub fn sub_tel_before(value: &str) -> Option<String> {
        TEL.captures(value)
            .and_then(|captures| captures.get(1))
            .map(|matched| matched.as_str().to_owned())
    }

    /// Extracts the subscriber number from a valid mainland landline.
    #[must_use]
    pub fn sub_tel_after(value: &str) -> Option<String> {
        TEL.captures(value)
            .and_then(|captures| captures.get(2))
            .map(|matched| matched.as_str().to_owned())
    }
}

fn hide(value: &str, start: usize, end: usize) -> String {
    value
        .chars()
        .enumerate()
        .map(|(index, character)| {
            if (start..end).contains(&index) {
                '*'
            } else {
                character
            }
        })
        .collect()
}

fn substring(value: &str, start: usize, end: usize) -> String {
    value
        .chars()
        .skip(start)
        .take(end.saturating_sub(start))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn regional_and_service_patterns_match_hutool_exactly() {
        assert!(PhoneUtil::is_mobile("13612345678"));
        assert!(PhoneUtil::is_mobile("+8613612345678"));
        assert!(!PhoneUtil::is_mobile("136123456781"));
        assert!(!PhoneUtil::is_mobile("١٣٦١٢٣٤٥٦٧٨"));
        assert!(PhoneUtil::is_mobile_hk("+85251234567"));
        assert!(PhoneUtil::is_mobile_hk("051234567"));
        assert!(!PhoneUtil::is_mobile_hk("5123456"));
        assert!(PhoneUtil::is_mobile_tw("+886-0912345678"));
        assert!(PhoneUtil::is_mobile_tw("0912345678"));
        assert!(!PhoneUtil::is_mobile_tw("0812345678"));
        assert!(PhoneUtil::is_mobile_mo("+853-61234567"));
        assert!(PhoneUtil::is_mobile_mo("061234567"));
        assert!(!PhoneUtil::is_mobile_mo("71234567"));

        for valid in ["010-12345678", "01012345678", "020-9999999", "07557654321"] {
            assert!(PhoneUtil::is_tel(valid));
            assert!(PhoneUtil::is_phone(valid));
        }
        for invalid in [
            "010 12345678",
            "A20-9999999",
            "0755-7654.321",
            "13619887123",
        ] {
            assert!(!PhoneUtil::is_tel(invalid));
        }
        for valid in [
            "400-860-8608",
            "400-8608608",
            "800-830-3811",
            "0755-7654321",
        ] {
            assert!(PhoneUtil::is_tel_400_800(valid));
        }
        assert!(!PhoneUtil::is_phone("not-a-phone"));
    }

    #[test]
    fn masking_slicing_and_landline_capture_are_unicode_safe() {
        let mobile = "13612345678";
        assert_eq!(PhoneUtil::hide_before(mobile), "*******5678");
        assert_eq!(PhoneUtil::hide_between(mobile), "136****5678");
        assert_eq!(PhoneUtil::hide_after(mobile), "1361234****");
        assert_eq!(PhoneUtil::sub_before(mobile), "136");
        assert_eq!(PhoneUtil::sub_between(mobile), "1234");
        assert_eq!(PhoneUtil::sub_after(mobile), "5678");
        assert_eq!(PhoneUtil::hide_before("电话123"), "*****");
        assert_eq!(PhoneUtil::sub_before("电话123"), "电话1");
        assert_eq!(PhoneUtil::sub_after("short"), "");

        assert_eq!(
            PhoneUtil::sub_tel_before("010-12345678").as_deref(),
            Some("010")
        );
        assert_eq!(
            PhoneUtil::sub_tel_after("01012345678").as_deref(),
            Some("12345678")
        );
        assert_eq!(
            PhoneUtil::sub_tel_before("07557654321").as_deref(),
            Some("0755")
        );
        assert_eq!(PhoneUtil::sub_tel_after("invalid"), None);
    }
}
