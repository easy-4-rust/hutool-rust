//! 对齐: `cn.hutool.http.useragent.UserAgentParser`
//! 来源: hutool-http/src/main/java/cn/hutool/http/useragent/UserAgentParser.java
//! 中文说明: User-Agent解析器，组合浏览器、引擎、操作系统和平台规则进行解析

use regex::{Regex, RegexBuilder};
use std::{
    fmt,
    hash::{Hash, Hasher},
    sync::{OnceLock, RwLock},
};
use woothee::parser::Parser as WootheeParser;

use super::browser::Browser;
use super::engine::Engine;
use super::operating_system::OperatingSystem;
use super::platform::Platform;
use super::user_agent::UserAgent;

/// Stateless User-Agent parser facade.
#[derive(Debug, Clone, Copy, Default)]
pub struct UserAgentParser;

impl UserAgentParser {
    /// Parses a non-blank User-Agent string.
    #[must_use]
    pub fn parse(user_agent: &str) -> Option<UserAgent> {
        if user_agent.trim().is_empty() {
            return None;
        }

        let browser = find_browser(user_agent);
        let engine = find_engine(user_agent);
        let os = find_operating_system(user_agent);
        let platform = find_platform(user_agent);
        let mobile = (platform.is_mobile() || browser.is_mobile()) && !os.is_macos();
        Some(UserAgent {
            version: browser.version(user_agent),
            engine_version: engine.version(user_agent),
            os_version: os.version(user_agent),
            mobile,
            browser,
            platform,
            os,
            engine,
        })
    }
}

use super::{UNKNOWN_NAME, built_in_browsers, built_in_engines, built_in_operating_systems, built_in_platforms, capture, case_insensitive_regex, custom_browsers};
use super::{custom_operating_systems, engine_version, find_browser, find_engine, find_operating_system, find_platform, is_mobile_browser_name, is_mobile_platform_name};
use super::{read_rules, unknown_browser, unknown_engine, unknown_operating_system, unknown_platform, woothee_browser, woothee_operating_system, write_rules};
