//! 对齐: `cn.hutool.http.useragent.Engine`
//! 来源: hutool-http/src/main/java/cn/hutool/http/useragent/Engine.java
//! 中文说明: 渲染引擎识别规则，通过正则匹配识别WebKit、Gecko等引擎

use std::{fmt, hash::{Hash, Hasher}};

use super::rule_error::RuleError;
use super::user_agent_info::UserAgentInfo;
use super::{UNKNOWN_NAME, engine_version};

/// Rendering-engine identification rule.
#[derive(Debug, Clone)]
pub struct Engine {
    pub(crate) info: UserAgentInfo,
    version_name: Option<String>,
}

impl Engine {
    /// Creates an engine rule and its conventional version matcher.
    pub fn new(name: impl Into<String>, regex: Option<&str>) -> Result<Self, RuleError> {
        let name = name.into();
        let version_name = (name != UNKNOWN_NAME).then(|| name.clone());
        Ok(Self {
            info: UserAgentInfo::new(name, regex)?,
            version_name,
        })
    }

    /// Returns the engine name.
    #[must_use]
    pub fn name(&self) -> &str {
        self.info.name()
    }

    /// Extracts the engine version.
    #[must_use]
    pub fn version(&self, user_agent: &str) -> Option<String> {
        engine_version(self.version_name.as_deref()?, user_agent)
    }

    /// Returns whether this engine is unknown.
    #[must_use]
    pub fn is_unknown(&self) -> bool {
        self.info.is_unknown()
    }
}

impl fmt::Display for Engine {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.info.fmt(formatter)
    }
}

impl PartialEq for Engine {
    fn eq(&self, other: &Self) -> bool {
        self.info == other.info
    }
}

impl Eq for Engine {}

impl Hash for Engine {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.info.hash(state);
    }
}
