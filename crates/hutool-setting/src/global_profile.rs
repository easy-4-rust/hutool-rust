//! 对齐: `cn.hutool.setting.GlobalProfile` (全局配置档案)
//! 来源: hutool-setting/src/main/java/cn/hutool/setting/GlobalProfile.java
//! 中文说明: 兼容用的进程级全局配置档案，建议优先使用显式持有的 `Profile`。

use super::profile::Profile;
use crate::{Setting, SettingError};
use std::{
    path::Path,
    sync::{OnceLock, RwLock},
};

/// 兼容用的进程级全局配置档案，对应 Hutool 的 `GlobalProfile`。
///
/// 对齐 Java 类: `cn.hutool.setting.GlobalProfile`
/// 来源: hutool-setting/src/main/java/cn/hutool/setting/GlobalProfile.java
///
/// 建议优先使用显式持有的 [`Profile`] 值以避免隐式全局状态。
/// Compatibility-only process profile. Prefer owned [`Profile`] values.
pub struct GlobalProfile;
impl GlobalProfile {
    fn global() -> &'static RwLock<Profile> {
        static GLOBAL: OnceLock<RwLock<Profile>> = OnceLock::new();
        GLOBAL.get_or_init(|| RwLock::new(Profile::default()))
    }
    /// Changes the compatibility profile.
    pub fn set_profile(profile: impl Into<String>) {
        Self::global()
            .write()
            .expect("global profile poisoned")
            .set_profile(profile);
    }
    /// Loads a cloned setting from the compatibility profile.
    pub fn get_setting(name: impl AsRef<Path>) -> Result<Setting, SettingError> {
        Self::global()
            .write()
            .expect("global profile poisoned")
            .get_setting(name)
            .cloned()
    }
    /// Clears compatibility state, primarily for deterministic tests.
    pub fn clear() {
        Self::global()
            .write()
            .expect("global profile poisoned")
            .clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn global_profile_has_explicit_reset() {
        GlobalProfile::clear();
        GlobalProfile::set_profile("definitely-missing");
        assert!(GlobalProfile::get_setting("none").is_err());
        GlobalProfile::clear();
    }
}
