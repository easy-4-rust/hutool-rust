//! 对齐: `cn.hutool.setting.Setting` (自动重载功能)
//! 来源: hutool-setting/src/main/java/cn/hutool/setting/Setting.java
//! 中文说明: 配置文件自动重载句柄，持有文件系统监听器，析构时停止自动重载。

use super::WatchOwner;

/// 配置文件自动重载句柄，对应 Hutool `Setting.autoLoad()` 的返回值。
///
/// 对齐 Java: `cn.hutool.setting.Setting` 的 `autoLoad` 方法
/// 来源: hutool-setting/src/main/java/cn/hutool/setting/Setting.java
///
/// 持有文件系统监听器，析构时停止自动重载。
/// Owns a filesystem watcher. Dropping it stops automatic reload.
pub struct AutoLoadHandle {
    pub(crate) _watcher: Box<dyn WatchOwner>,
}

impl std::fmt::Debug for AutoLoadHandle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AutoLoadHandle").finish_non_exhaustive()
    }
}
