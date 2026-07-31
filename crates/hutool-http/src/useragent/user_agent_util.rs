//! 对齐: `cn.hutool.http.useragent.UserAgentUtil`
//! 来源: hutool-http/src/main/java/cn/hutool/http/useragent/UserAgentUtil.java
//! 中文说明: User-Agent工具门面，提供简化的解析方法和移动端判断

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
