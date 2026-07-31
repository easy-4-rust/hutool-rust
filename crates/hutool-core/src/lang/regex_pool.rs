//! 对齐: `cn.hutool.core.lang.RegexPool`
//! 来源: hutool-core/src/main/java/cn/hutool/core/lang/RegexPool.java
//!
//! 常用正则字符串常量池（不含编译缓存；编译见 [`super::pattern_pool::PatternPool`]）。

/// 对齐 Java: `cn.hutool.core.lang.RegexPool`
pub struct RegexPool;

#[allow(dead_code)] // 对齐 Java 常量池，暂未全部接线，预留
impl RegexPool {
    /// 对齐 Java: `RegexPool.GENERAL`
    pub const GENERAL: &'static str = r"^\w+$";
    /// 对齐 Java: `RegexPool.NUMBERS`
    pub const NUMBERS: &'static str = r"\d+";
    /// 对齐 Java: `RegexPool.WORD`
    pub const WORD: &'static str = r"[a-zA-Z]+";
    /// 对齐 Java: `RegexPool.CHINESE`
    pub const CHINESE: &'static str =
        "[\u{2E80}-\u{2EFF}\u{2F00}-\u{2FDF}\u{31C0}-\u{31EF}\u{3400}-\u{4DBF}\u{4E00}-\u{9FFF}\u{F900}-\u{FAFF}]";
    /// 对齐 Java: `RegexPool.CHINESES`
    pub const CHINESES: &'static str =
        "[\u{2E80}-\u{2EFF}\u{2F00}-\u{2FDF}\u{31C0}-\u{31EF}\u{3400}-\u{4DBF}\u{4E00}-\u{9FFF}\u{F900}-\u{FAFF}]+";
    /// 对齐 Java: `RegexPool.GROUP_VAR`
    pub const GROUP_VAR: &'static str = r"\$(\d+)";
    /// 对齐 Java: `RegexPool.IPV4`
    pub const IPV4: &'static str =
        r"^(25[0-5]|2[0-4]\d|[0-1]?\d?\d)\.(25[0-5]|2[0-4]\d|[0-1]?\d?\d)\.(25[0-5]|2[0-4]\d|[0-1]?\d?\d)\.(25[0-5]|2[0-4]\d|[0-1]?\d?\d)$";
    /// 对齐 Java: `RegexPool.IPV6`
    pub const IPV6: &'static str = concat!(
        "(([0-9a-fA-F]{1,4}:){7}[0-9a-fA-F]{1,4}|([0-9a-fA-F]{1,4}:){1,7}:|",
        "([0-9a-fA-F]{1,4}:){1,6}:[0-9a-fA-F]{1,4}|([0-9a-fA-F]{1,4}:){1,5}(:[0-9a-fA-F]{1,4}){1,2}|",
        "([0-9a-fA-F]{1,4}:){1,4}(:[0-9a-fA-F]{1,4}){1,3}|([0-9a-fA-F]{1,4}:){1,3}(:[0-9a-fA-F]{1,4}){1,4}|",
        "([0-9a-fA-F]{1,4}:){1,2}(:[0-9a-fA-F]{1,4}){1,5}|[0-9a-fA-F]{1,4}:((:[0-9a-fA-F]{1,4}){1,6})|",
        ":((:[0-9a-fA-F]{1,4}){1,7}|:)|fe80:(:[0-9a-fA-F]{0,4}){0,4}%[0-9a-zA-Z]+|",
        "::(ffff(:0{1,4})?:)?((25[0-5]|(2[0-4]|1?[0-9])?[0-9])\\.){3}(25[0-5]|(2[0-4]|1?[0-9])?[0-9])|",
        "([0-9a-fA-F]{1,4}:){1,4}:((25[0-5]|(2[0-4]|1?[0-9])?[0-9])\\.){3}(25[0-5]|(2[0-4]|1?[0-9])?[0-9]))"
    );
    /// 对齐 Java: `RegexPool.MONEY`
    pub const MONEY: &'static str = r"^(\d+(?:\.\d+)?)$";
    /// 对齐 Java: `RegexPool.EMAIL`（RFC5322 宽松版，与 Hutool 同源结构）。
    pub const EMAIL: &'static str = r###"(?:[a-z0-9!#$%&'*+/=?^_`{|}~-]+(?:\.[a-z0-9!#$%&'*+/=?^_`{|}~-]+)*|"(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21\x23-\x5b\x5d-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])*")@(?:(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\.)+[a-z0-9](?:[a-z0-9-]*[a-z0-9])?|\[(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?|[a-z0-9-]*[a-z0-9]:(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21-\x5a\x53-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])+)\])"###;
    /// 对齐 Java: `RegexPool.EMAIL_WITH_CHINESE`
    pub const EMAIL_WITH_CHINESE: &'static str = r###"(?:[a-z0-9\u{4e00}-\u{9fa5}!#$%&'*+/=?^_`{|}~-]+(?:\.[a-z0-9\u{4e00}-\u{9fa5}!#$%&'*+/=?^_`{|}~-]+)*|"(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21\x23-\x5b\x5d-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])*")@(?:(?:[a-z0-9\u{4e00}-\u{9fa5}](?:[a-z0-9\u{4e00}-\u{9fa5}-]*[a-z0-9\u{4e00}-\u{9fa5}])?\.)+[a-z0-9\u{4e00}-\u{9fa5}](?:[a-z0-9\u{4e00}-\u{9fa5}-]*[a-z0-9\u{4e00}-\u{9fa5}])?|\[(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?|[a-z0-9\u{4e00}-\u{9fa5}-]*[a-z0-9\u{4e00}-\u{9fa5}]:(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21-\x5a\x53-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])+)\])"###;
    /// 对齐 Java: `RegexPool.MOBILE`
    pub const MOBILE: &'static str = r"(?:0|86|\+86)?1[3-9]\d{9}";
    /// 对齐 Java: `RegexPool.MOBILE_HK`
    pub const MOBILE_HK: &'static str = r"(?:0|852|\+852)?\d{8}";
    /// 对齐 Java: `RegexPool.MOBILE_TW`
    pub const MOBILE_TW: &'static str = r"(?:0|886|\+886)?(?:|-)?09\d{8}";
    /// 对齐 Java: `RegexPool.MOBILE_MO`
    pub const MOBILE_MO: &'static str = r"(?:0|853|\+853)?(?:|-)?6\d{7}";
    /// 对齐 Java: `RegexPool.TEL`
    pub const TEL: &'static str = r"(010|02\d|0[3-9]\d{2})-?(\d{6,8})";
    /// 对齐 Java: `RegexPool.TEL_400_800`
    pub const TEL_400_800: &'static str =
        r"0\d{2,3}[\- ]?[0-9]\d{6,7}|[48]00[\- ]?[0-9]\d{2}[\- ]?\d{4}";
    /// 对齐 Java: `RegexPool.CITIZEN_ID`
    pub const CITIZEN_ID: &'static str =
        r"[1-9]\d{5}[1-2]\d{3}((0\d)|(1[0-2]))(([012]\d)|3[0-1])\d{3}(\d|X|x)";
    /// 对齐 Java: `RegexPool.ZIP_CODE`
    pub const ZIP_CODE: &'static str =
        r"^(0[1-7]|1[0-356]|2[0-7]|3[0-6]|4[0-7]|5[0-7]|6[0-7]|7[0-5]|8[0-9]|9[0-8])\d{4}|99907[78]$";
    /// 对齐 Java: `RegexPool.BIRTHDAY`
    pub const BIRTHDAY: &'static str =
        r"^(\d{2,4})([/\\-.年]?)(\d{1,2})([/\\-.月]?)(\d{1,2})日?$";
    /// 对齐 Java: `RegexPool.URI`
    pub const URI: &'static str = r"^(([^:/?#]+):)?(//([^/?#]*))?([^?#]*)(\?([^#]*))?(#(.*))?";
    /// 对齐 Java: `RegexPool.URL`
    pub const URL: &'static str = r"[a-zA-Z]+://[\w-+&@#/%?=~_|!:,.;]*[\w-+&@#/%=~_|]";
    /// 对齐 Java: `RegexPool.URL_HTTP`
    pub const URL_HTTP: &'static str = r"(https?|ftp|file)://[\w-+&@#/%?=~_|!:,.;]*[\w-+&@#/%=~_|]";
    /// 对齐 Java: `RegexPool.GENERAL_WITH_CHINESE`
    pub const GENERAL_WITH_CHINESE: &'static str = "^[\u{4E00}-\u{9FFF}\\w]+$";
    /// 对齐 Java: `RegexPool.UUID`
    pub const UUID: &'static str =
        r"^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$";
    /// 对齐 Java: `RegexPool.UUID_SIMPLE`
    pub const UUID_SIMPLE: &'static str = r"^[0-9a-fA-F]{32}$";
    /// 对齐 Java: `RegexPool.HEX`
    pub const HEX: &'static str = r"^[a-fA-F0-9]+$";
    /// 对齐 Java: `RegexPool.MAC_ADDRESS`
    pub const MAC_ADDRESS: &'static str = concat!(
        r"((?:[a-fA-F0-9]{1,2}[:-]){5}[a-fA-F0-9]{1,2})",
        r"|((?:[a-fA-F0-9]{1,4}[.]){2}[a-fA-F0-9]{1,4})",
        r"|[a-fA-F0-9]{12}|0x(\d{12}).+ETHER"
    );
    /// 对齐 Java: `RegexPool.TIME`
    pub const TIME: &'static str = r"\d{1,2}[:时]\d{1,2}([:分]\d{1,2})?秒?";
    /// 对齐 Java: `RegexPool.PLATE_NUMBER`
    pub const PLATE_NUMBER: &'static str = concat!(
        "^(([京津沪渝冀豫云辽黑湘皖鲁新苏浙赣鄂桂甘晋蒙陕吉闽贵粤青藏川宁琼使领]",
        "[A-Z](([0-9]{5}[ABCDEFGHJK])|([ABCDEFGHJKP]([A-HJ-NP-Z0-9])[0-9]{4})))",
        "|([京津沪渝冀豫云辽黑湘皖鲁新苏浙赣鄂桂甘晋蒙陕吉闽贵粤青藏川宁琼使领]\\d{3}\\d{1,3}[领])",
        "|([京津沪渝冀豫云辽黑湘皖鲁新苏浙赣鄂桂甘晋蒙陕吉闽贵粤青藏川宁琼使领][A-Z][A-HJ-NP-Z0-9]{4}[A-HJ-NP-Z0-9挂学警港澳使领]))$"
    );
    /// 对齐 Java: `RegexPool.CREDIT_CODE`
    pub const CREDIT_CODE: &'static str = r"^[0-9A-HJ-NPQRTUWXY]{2}\d{6}[0-9A-HJ-NPQRTUWXY]{10}$";
    /// 对齐 Java: `RegexPool.CAR_VIN`
    pub const CAR_VIN: &'static str =
        r"^[A-HJ-NPR-Z0-9]{8}[X0-9]([A-HJ-NPR-Z0-9]{3}\d{5}|[A-HJ-NPR-Z0-9]{5}\d{3})$";
    /// 对齐 Java: `RegexPool.CAR_DRIVING_LICENCE`
    pub const CAR_DRIVING_LICENCE: &'static str = r"^[0-9]{12}$";
    /// 对齐 Java: `RegexPool.CHINESE_NAME`
    pub const CHINESE_NAME: &'static str = "^[\u{3400}-\u{9FFF}·]{2,60}$";
}

#[cfg(test)]
mod regex_pool_idiomatic_parity {
    use super::*;
    use regex::Regex;

    /// 对齐 Java RegexPool 常量可编译、可匹配的可执行证据。
    #[test]
    fn regex_pool_core_constants_compile_and_match() {
        assert!(Regex::new(RegexPool::GENERAL).unwrap().is_match("ab_1"));
        assert!(Regex::new(RegexPool::NUMBERS).unwrap().is_match("42"));
        assert!(Regex::new(RegexPool::MOBILE).unwrap().is_match("13800138000"));
        assert!(Regex::new(RegexPool::EMAIL).unwrap().is_match("a@b.com"));
        assert!(Regex::new(RegexPool::IPV4).unwrap().is_match("192.168.1.1"));
        assert!(Regex::new(RegexPool::UUID_SIMPLE)
            .unwrap()
            .is_match("0123456789abcdef0123456789abcdef"));
        assert!(Regex::new(RegexPool::CHINESE_NAME).unwrap().is_match("张三"));
    }
}
