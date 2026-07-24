use super::WatchOwner;
use crate::{DEFAULT_ENCODING, GroupedMap, Props, SettingError, SettingLoader, checked_path};
use encoding_rs::Encoding;
use indexmap::IndexMap;
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use serde::de::DeserializeOwned;
use std::{
    path::{Path, PathBuf},
    sync::{Arc, RwLock},
};

/// Owns a filesystem watcher. Dropping it stops automatic reload.
pub struct AutoLoadHandle {
    pub(crate) _watcher: Box<dyn WatchOwner>,
}

impl std::fmt::Debug for AutoLoadHandle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AutoLoadHandle").finish_non_exhaustive()
    }
}

use super::{ReloadHandler, box_watcher, create_recommended_watcher, fix_extension, reload_event};
