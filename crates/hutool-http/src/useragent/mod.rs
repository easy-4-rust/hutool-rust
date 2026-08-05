//! 对齐: `cn.hutool.http.useragent`
//! 来源: hutool-http/src/main/java/cn/hutool/http/useragent/
//! 中文说明: User-Agent解析模块，提供浏览器、引擎、操作系统和平台识别

use regex::{Regex, RegexBuilder};
use std::sync::{OnceLock, RwLock};
use woothee::parser::Parser as WootheeParser;

mod browser;
mod engine;
mod operating_system;
mod platform;
mod rule_error;
mod user_agent;
mod user_agent_info;
mod user_agent_parser;
mod user_agent_util;

pub use browser::Browser;
pub use engine::Engine;
pub use operating_system::OperatingSystem;
pub use platform::Platform;
pub use rule_error::RuleError;
pub use user_agent::UserAgent;
pub use user_agent_info::UserAgentInfo;
pub use user_agent_parser::UserAgentParser;
pub use user_agent_util::UserAgentUtil;

/// 未识别组件的默认名称，对齐 Hutool `UserAgentInfo.UNKNOWN_NAME`。
pub const UNKNOWN_NAME: &str = "Unknown";

fn find_browser(user_agent: &str) -> Browser {
    built_in_browsers()
        .iter()
        .chain(read_rules(custom_browsers()).iter())
        .find(|browser| browser.info.is_match(user_agent))
        .cloned()
        .or_else(|| woothee_browser(user_agent))
        .unwrap_or_else(unknown_browser)
}

fn find_engine(user_agent: &str) -> Engine {
    built_in_engines()
        .iter()
        .find(|engine| engine.info.is_match(user_agent))
        .cloned()
        .unwrap_or_else(unknown_engine)
}

fn find_operating_system(user_agent: &str) -> OperatingSystem {
    built_in_operating_systems()
        .iter()
        .chain(read_rules(custom_operating_systems()).iter())
        .find(|os| os.info.is_match(user_agent))
        .cloned()
        .or_else(|| woothee_operating_system(user_agent))
        .unwrap_or_else(unknown_operating_system)
}

fn find_platform(user_agent: &str) -> Platform {
    built_in_platforms()
        .iter()
        .find(|platform| platform.info.is_match(user_agent))
        .cloned()
        .unwrap_or_else(unknown_platform)
}

fn woothee_browser(user_agent: &str) -> Option<Browser> {
    let result = WootheeParser::new().parse(user_agent)?;
    (result.name != UNKNOWN_NAME)
        .then(|| Browser::new(result.name, None, None).expect("Woothee names are valid"))
}

fn woothee_operating_system(user_agent: &str) -> Option<OperatingSystem> {
    let result = WootheeParser::new().parse(user_agent)?;
    (result.os != UNKNOWN_NAME).then(|| {
        OperatingSystem::new(result.os, None).expect("Woothee operating-system names are valid")
    })
}

fn unknown_browser() -> Browser {
    Browser::new(UNKNOWN_NAME, None, None).expect("empty rule is valid")
}

fn unknown_engine() -> Engine {
    Engine::new(UNKNOWN_NAME, None).expect("empty rule is valid")
}

fn unknown_operating_system() -> OperatingSystem {
    OperatingSystem::new(UNKNOWN_NAME, None).expect("empty rule is valid")
}

fn unknown_platform() -> Platform {
    Platform::new(UNKNOWN_NAME, None).expect("empty rule is valid")
}

fn built_in_browsers() -> &'static [Browser] {
    static RULES: OnceLock<Vec<Browser>> = OnceLock::new();
    RULES.get_or_init(|| {
        [
            ("wxwork", "wxwork", r"wxwork/([\d\w.\-]+)"),
            (
                "WindowsWechat",
                "WindowsWechat",
                r"MicroMessenger[/ ]([\d\w.\-]+)",
            ),
            (
                "MicroMessenger",
                "MicroMessenger",
                r"MicroMessenger[/ ]([\d\w.\-]+)",
            ),
            ("miniProgram", "miniProgram", r"miniProgram[/ ]([\d\w.\-]+)"),
            ("QQBrowser", "QQBrowser", r"QQBrowser/([\d\w.\-]+)"),
            ("DingTalk-win", "dingtalk-win", r"DingTalk\(([\d\w.\-]+)\)"),
            ("DingTalk", "DingTalk", r"AliApp\(DingTalk/([\d\w.\-]+)\)"),
            ("Alipay", "AlipayClient", r"AliApp\(AP/([\d\w.\-]+)\)"),
            ("Taobao", "taobao", r"AliApp\(TB/([\d\w.\-]+)\)"),
            ("UCBrowser", "UC?Browser", r"UC?Browser/([\d\w.\-]+)"),
            (
                "MiuiBrowser",
                "MiuiBrowser|mibrowser",
                r"MiuiBrowser/([\d\w.\-]+)",
            ),
            ("Quark", "Quark", r"Quark[/ ]([\d\w.\-]+)"),
            ("Lenovo", "SLBrowser", r"SLBrowser/([\d\w.\-]+)"),
            ("MSEdge", "Edge|Edg", r"(?:edge|Edg|EdgA)/([\d\w.\-]+)"),
            (
                "Chrome",
                r"chrome|(iphone.*crios.*safari)",
                r"(?:Chrome|CriOS)/([\d\w.\-]+)",
            ),
            ("Firefox", "firefox", r"Firefox[/ ]([\d\w.\-]+)"),
            ("IEMobile", "iemobile", r"IEMobile[/ ]([\d\w.\-]+)"),
            ("Android Browser", "android", r"version/([\d\w.\-]+)"),
            ("Safari", "safari", r"version/([\d\w.\-]+)"),
            ("Opera", "opera", r"Opera[/ ]([\d\w.\-]+)"),
            ("Konqueror", "konqueror", r"Konqueror[/ ]([\d\w.\-]+)"),
            ("PS3", "playstation 3", r"([\d\w.\-]+)\)\s*$"),
            ("PSP", "playstation portable", r"([\d\w.\-]+)\)?\s*$"),
            ("Lotus", r"lotus\.notes", r"Lotus-Notes/([\w.]+)"),
            ("Thunderbird", "thunderbird", r"Thunderbird[/ ]([\d\w.\-]+)"),
            ("Netscape", "netscape", r"Netscape[/ ]([\d\w.\-]+)"),
            ("Seamonkey", "seamonkey", r"Seamonkey[/ ]([\d\w.\-]+)"),
            ("Outlook", r"microsoft\.outlook", r"Outlook[/ ]([\d\w.\-]+)"),
            ("Evolution", "evolution", r"Evolution[/ ]([\d\w.\-]+)"),
            ("MSIE", "msie", r"msie ([\d\w.\-]+)"),
            ("MSIE11", "rv:11", r"rv:([\d\w.\-]+)"),
            ("Gabble", "Gabble", r"Gabble[/ ]([\d\w.\-]+)"),
            ("Yammer Desktop", "AdobeAir", r"([\d\w.\-]+)/Yammer"),
            (
                "Yammer Mobile",
                r"Yammer\s+[\d\w.\-]+",
                r"Yammer\s+([\d\w.\-]+)",
            ),
            (
                "Apache HTTP Client",
                r"Apache\-HttpClient",
                r"Apache\-HttpClient/([\d\w.\-]+)",
            ),
            ("BlackBerry", "BlackBerry", r"BlackBerry[\d]+/([\d\w.\-]+)"),
            ("Baidu", "Baidu", r"baiduboxapp/([\d\w.\-]+)"),
        ]
        .into_iter()
        .map(|(name, regex, version)| {
            Browser::new(name, Some(regex), Some(version)).expect("built-in browser rule is valid")
        })
        .collect()
    })
}

fn built_in_engines() -> &'static [Engine] {
    static RULES: OnceLock<Vec<Engine>> = OnceLock::new();
    RULES.get_or_init(|| {
        [
            ("Trident", "trident"),
            ("Webkit", "webkit"),
            ("Chrome", "chrome"),
            ("Opera", "opera"),
            ("Presto", "presto"),
            ("Gecko", "gecko"),
            ("KHTML", "khtml"),
            ("Konqueror", "konqueror"),
            ("MIDP", "MIDP"),
        ]
        .into_iter()
        .map(|(name, regex)| Engine::new(name, Some(regex)).expect("built-in engine rule is valid"))
        .collect()
    })
}

fn built_in_operating_systems() -> &'static [OperatingSystem] {
    static RULES: OnceLock<Vec<OperatingSystem>> = OnceLock::new();
    RULES.get_or_init(|| {
        [
            (
                "Windows 10 or Windows Server 2016",
                r"windows nt 10\.0",
                r"windows nt (10\.0)",
            ),
            (
                "Windows 8.1 or Windows Server 2012R2",
                r"windows nt 6\.3",
                r"windows nt (6\.3)",
            ),
            (
                "Windows 8 or Windows Server 2012",
                r"windows nt 6\.2",
                r"windows nt (6\.2)",
            ),
            ("Windows Vista", r"windows nt 6\.0", r"windows nt (6\.0)"),
            (
                "Windows 7 or Windows Server 2008R2",
                r"windows nt 6\.1",
                r"windows nt (6\.1)",
            ),
            ("Windows 2003", r"windows nt 5\.2", r"windows nt (5\.2)"),
            ("Windows XP", r"windows nt 5\.1", r"windows nt (5\.1)"),
            ("Windows 2000", r"windows nt 5\.0", r"windows nt (5\.0)"),
            (
                "Windows Phone",
                r"windows (ce|phone|mobile)( os)?",
                r"windows (?:ce|phone|mobile) (\d+(?:[._]\d+)*)",
            ),
            ("Windows", "windows", r"windows(?: nt)? ([\d._]+)"),
            ("OSX", r"os x \d+[._]\d+", r"os x (\d+(?:[._]\d+)*)"),
            ("Android", "Android", r"Android (\d+(?:[._]\d+)*)"),
            ("Harmony", "OpenHarmony", r"OpenHarmony (\d+(?:[._]\d+)*)"),
            ("Android", r"XiaoMi|MI\s+", r"\(X(\d+(?:[._]\d+)*)"),
            ("Linux", "linux", r"Linux[/ ]([\d._]+)"),
            ("Wii", "wii", r"wii libnup/(\d+(?:[._]\d+)*)"),
            ("PS3", "playstation 3", r"playstation 3; (\d+(?:[._]\d+)*)"),
            (
                "PSP",
                "playstation portable",
                r"Portable\); (\d+(?:[._]\d+)*)",
            ),
            (
                "iPad",
                r"\(iPad.*os \d+[._]\d+",
                r"\(iPad.*os (\d+(?:[._]\d+)*)",
            ),
            (
                "iPhone",
                r"\(iPhone.*os \d+[._]\d+",
                r"\(iPhone.*os (\d+(?:[._]\d+)*)",
            ),
            (
                "YPod",
                r"iPod touch[\s;]+iPhone.*os \d+[._]\d+",
                r"iPod touch[\s;]+iPhone.*os (\d+(?:[._]\d+)*)",
            ),
            (
                "YPad",
                r"iPad[\s;]+iPhone.*os \d+[._]\d+",
                r"iPad[\s;]+iPhone.*os (\d+(?:[._]\d+)*)",
            ),
            (
                "YPhone",
                r"iPhone[\s;]+iPhone.*os \d+[._]\d+",
                r"iPhone[\s;]+iPhone.*os (\d+(?:[._]\d+)*)",
            ),
            ("Symbian", "symbian(os)?", r"Symbian(?:OS)?[/ ]([\d._]+)"),
            ("Darwin", r"Darwin/[\d\w.\-]+", r"Darwin/([\d\w.\-]+)"),
            (
                "Adobe Air",
                r"AdobeAir/[\d\w.\-]+",
                r"AdobeAir/([\d\w.\-]+)",
            ),
            ("Java", r"Java\s+[\d\w.\-]+", r"Java\s+([\d\w.\-]+)"),
        ]
        .into_iter()
        .map(|(name, regex, version)| {
            OperatingSystem::with_version(name, Some(regex), Some(version))
                .expect("built-in operating-system rule is valid")
        })
        .collect()
    })
}

fn built_in_platforms() -> &'static [Platform] {
    static RULES: OnceLock<Vec<Platform>> = OnceLock::new();
    RULES.get_or_init(|| {
        [
            ("Windows Phone", r"windows (ce|phone|mobile)( os)?"),
            ("iPad", "ipad"),
            ("iPod", "ipod"),
            ("iPhone", "iphone"),
            ("Android", r"XiaoMi|MI\s+"),
            ("Android", "android"),
            ("GoogleTV", "googletv"),
            ("htcFlyer", "htc_flyer"),
            ("Symbian", "symbian(os)?"),
            ("Blackberry", "blackberry"),
            ("Harmony", "OpenHarmony"),
            ("Windows", "windows"),
            ("Mac", "(macintosh|darwin)"),
            ("Linux", "linux"),
            ("Wii", "wii"),
            ("Playstation", "playstation"),
            ("Java", "java"),
        ]
        .into_iter()
        .map(|(name, regex)| {
            Platform::new(name, Some(regex)).expect("built-in platform rule is valid")
        })
        .collect()
    })
}

fn custom_browsers() -> &'static RwLock<Vec<Browser>> {
    static RULES: OnceLock<RwLock<Vec<Browser>>> = OnceLock::new();
    RULES.get_or_init(|| RwLock::new(Vec::new()))
}

fn custom_operating_systems() -> &'static RwLock<Vec<OperatingSystem>> {
    static RULES: OnceLock<RwLock<Vec<OperatingSystem>>> = OnceLock::new();
    RULES.get_or_init(|| RwLock::new(Vec::new()))
}

fn read_rules<T>(rules: &'static RwLock<Vec<T>>) -> std::sync::RwLockReadGuard<'static, Vec<T>> {
    rules
        .read()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
}

fn write_rules<T>(rules: &'static RwLock<Vec<T>>) -> std::sync::RwLockWriteGuard<'static, Vec<T>> {
    rules
        .write()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
}

fn capture(pattern: Option<&Regex>, content: &str) -> Option<String> {
    pattern?
        .captures(content)?
        .get(1)
        .map(|value| value.as_str().to_owned())
}

fn engine_version(name: &str, user_agent: &str) -> Option<String> {
    let start = user_agent
        .to_ascii_lowercase()
        .find(&name.to_ascii_lowercase())?
        + name.len();
    let value = user_agent[start..].strip_prefix(['/', '-', ' '])?;
    let end = value
        .find(|character: char| {
            !character.is_ascii_alphanumeric() && !matches!(character, '.' | '-')
        })
        .unwrap_or(value.len());
    (end > 0).then(|| value[..end].to_owned())
}

fn case_insensitive_regex(pattern: &str) -> Result<Regex, RuleError> {
    RegexBuilder::new(pattern).case_insensitive(true).build()
}

fn is_mobile_browser_name(name: &str) -> bool {
    matches!(
        name,
        "PSP"
            | "Yammer Mobile"
            | "Android Browser"
            | "IEMobile"
            | "MicroMessenger"
            | "miniProgram"
            | "DingTalk"
    )
}

fn is_mobile_platform_name(name: &str) -> bool {
    matches!(
        name,
        "Windows Phone"
            | "iPad"
            | "iPod"
            | "iPhone"
            | "Android"
            | "GoogleTV"
            | "htcFlyer"
            | "Symbian"
            | "Blackberry"
            | "Harmony"
    )
}
