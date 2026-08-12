//! 对齐: `cn.hutool.http.useragent.UserAgentParser`
//! 来源: hutool-http/src/main/java/cn/hutool/http/useragent/UserAgentParser.java
//! 中文说明: User-Agent解析器，组合浏览器、引擎、操作系统和平台规则进行解析

use super::user_agent::UserAgent;
use super::{find_browser, find_engine, find_operating_system, find_platform};

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
