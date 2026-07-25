//! 对齐: `cn.hutool.http.useragent.UserAgent`
//! 来源: hutool-http/src/main/java/cn/hutool/http/useragent/UserAgent.java
//! 中文说明: User-Agent解析结果，包含浏览器、引擎、操作系统和平台信息

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

/// Parsed User-Agent information.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserAgent {
    pub(crate) mobile: bool,
    pub(crate) browser: Browser,
    pub(crate) version: Option<String>,
    pub(crate) platform: Platform,
    pub(crate) os: OperatingSystem,
    pub(crate) os_version: Option<String>,
    pub(crate) engine: Engine,
    pub(crate) engine_version: Option<String>,
}

impl UserAgent {
    /// Returns whether the client is mobile.
    #[must_use]
    pub const fn is_mobile(&self) -> bool {
        self.mobile
    }

    /// Overrides the mobile classification.
    pub const fn set_mobile(&mut self, mobile: bool) {
        self.mobile = mobile;
    }

    /// Returns the detected browser.
    #[must_use]
    pub const fn browser(&self) -> &Browser {
        &self.browser
    }

    /// Replaces the detected browser.
    pub fn set_browser(&mut self, browser: Browser) {
        self.browser = browser;
    }

    /// Returns the detected platform.
    #[must_use]
    pub const fn platform(&self) -> &Platform {
        &self.platform
    }

    /// Replaces the detected platform.
    pub fn set_platform(&mut self, platform: Platform) {
        self.platform = platform;
    }

    /// Returns the detected operating system.
    #[must_use]
    pub const fn os(&self) -> &OperatingSystem {
        &self.os
    }

    /// Replaces the detected operating system.
    pub fn set_os(&mut self, os: OperatingSystem) {
        self.os = os;
    }

    /// Returns the operating-system version.
    #[must_use]
    pub fn os_version(&self) -> Option<&str> {
        self.os_version.as_deref()
    }

    /// Replaces the operating-system version.
    pub fn set_os_version(&mut self, version: Option<String>) {
        self.os_version = version;
    }

    /// Returns the detected rendering engine.
    #[must_use]
    pub const fn engine(&self) -> &Engine {
        &self.engine
    }

    /// Replaces the detected rendering engine.
    pub fn set_engine(&mut self, engine: Engine) {
        self.engine = engine;
    }

    /// Returns the browser version.
    #[must_use]
    pub fn version(&self) -> Option<&str> {
        self.version.as_deref()
    }

    /// Replaces the browser version.
    pub fn set_version(&mut self, version: Option<String>) {
        self.version = version;
    }

    /// Returns the rendering-engine version.
    #[must_use]
    pub fn engine_version(&self) -> Option<&str> {
        self.engine_version.as_deref()
    }

    /// Replaces the rendering-engine version.
    pub fn set_engine_version(&mut self, version: Option<String>) {
        self.engine_version = version;
    }
}

use super::{UNKNOWN_NAME, built_in_browsers, built_in_engines, built_in_operating_systems, built_in_platforms, capture, case_insensitive_regex, custom_browsers};
use super::{custom_operating_systems, engine_version, find_browser, find_engine, find_operating_system, find_platform, is_mobile_browser_name, is_mobile_platform_name};
use super::{read_rules, unknown_browser, unknown_engine, unknown_operating_system, unknown_platform, woothee_browser, woothee_operating_system, write_rules};
