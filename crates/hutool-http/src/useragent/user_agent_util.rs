//! 对齐: `cn.hutool.http.useragent.UserAgentUtil`
//! 来源: hutool-http/src/main/java/cn/hutool/http/useragent/UserAgentUtil.java
//! 中文说明: User-Agent工具门面，提供简化的解析方法和移动端判断

use regex::{Regex, RegexBuilder};
use std::{
    fmt,
    hash::{Hash, Hasher},
    sync::{OnceLock, RwLock},
};
use woothee::parser::Parser as WootheeParser;

use super::user_agent::UserAgent;
use super::user_agent_parser::UserAgentParser;

/// Convenience facade matching Hutool's `UserAgentUtil` role.
#[derive(Debug, Clone, Copy, Default)]
pub struct UserAgentUtil;

impl UserAgentUtil {
    /// Parses a User-Agent string.
    #[must_use]
    pub fn parse(user_agent: &str) -> Option<UserAgent> {
        UserAgentParser::parse(user_agent)
    }
}

use super::{UNKNOWN_NAME, built_in_browsers, built_in_engines, built_in_operating_systems, built_in_platforms, capture, case_insensitive_regex, custom_browsers};
use super::{custom_operating_systems, engine_version, find_browser, find_engine, find_operating_system, find_platform, is_mobile_browser_name, is_mobile_platform_name};
use super::{read_rules, unknown_browser, unknown_engine, unknown_operating_system, unknown_platform, woothee_browser, woothee_operating_system, write_rules};
