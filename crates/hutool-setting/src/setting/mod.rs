//! 对齐: `cn.hutool.setting` (Setting 子模块)
//! 来源: hutool-setting/src/main/java/cn/hutool/setting/Setting.java
//! 中文说明: Hutool Setting 配置文档子模块，包含核心 Setting 结构体、自动重载句柄及文件监听机制。

use crate::{GroupedMap, SettingLoader};
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use std::{
    path::{Path, PathBuf},
    sync::{Arc, RwLock},
};

mod auto_load_handle;
mod setting;

pub use auto_load_handle::AutoLoadHandle;
pub use setting::Setting;

pub(crate) fn fix_extension(path: &Path, extension: &str) -> PathBuf {
    if path.extension().is_none() {
        path.with_extension(extension)
    } else {
        path.to_path_buf()
    }
}

fn reload_event(
    event: Result<notify::Event, notify::Error>,
    path: &Path,
    loader: &SettingLoader,
    data: &RwLock<GroupedMap>,
    callback: &dyn Fn(bool),
) {
    let success = event.is_ok_and(|event| event.paths.iter().any(|candidate| candidate == path));
    if success {
        if let Ok(bytes) = std::fs::read(path) {
            let mut parsed = GroupedMap::new();
            let mut reader = bytes.as_slice();
            if loader.load(&mut reader, &mut parsed).is_ok() {
                *data.write().expect("setting poisoned") = parsed;
                callback(true);
                return;
            }
        }
    }
    callback(false);
}

struct ReloadHandler {
    path: PathBuf,
    loader: SettingLoader,
    data: Arc<RwLock<GroupedMap>>,
    callback: Arc<dyn Fn(bool) + Send + Sync>,
}

fn create_recommended_watcher(
    handler: ReloadHandler,
) -> Result<Box<dyn WatchOwner>, notify::Error> {
    notify::recommended_watcher(handler).map(box_watcher)
}

fn box_watcher(watcher: RecommendedWatcher) -> Box<dyn WatchOwner> {
    Box::new(watcher)
}

pub(crate) trait WatchOwner: Send {
    fn watch_path(&mut self, path: &Path) -> Result<(), notify::Error>;
}

impl WatchOwner for RecommendedWatcher {
    fn watch_path(&mut self, path: &Path) -> Result<(), notify::Error> {
        self.watch(path, RecursiveMode::NonRecursive)
    }
}
