//! 对齐: `cn.hutool.core.swing.clipboard.ClipboardUtil`
//! 来源: hutool-core/src/main/java/cn/hutool/core/swing/clipboard/ClipboardUtil.java
//!
//! 无 GUI 环境下使用进程内剪贴板模拟，保证 parity 测试可重复执行。

use std::sync::{Mutex, OnceLock};

type Listener = Box<dyn Fn(&str) + Send + Sync>;

static TEXT_CLIPBOARD: OnceLock<Mutex<Option<String>>> = OnceLock::new();
static LISTENERS: OnceLock<Mutex<Vec<Listener>>> = OnceLock::new();

fn text_store() -> &'static Mutex<Option<String>> {
    TEXT_CLIPBOARD.get_or_init(|| Mutex::new(None))
}

fn listeners() -> &'static Mutex<Vec<Listener>> {
    LISTENERS.get_or_init(|| Mutex::new(Vec::new()))
}

/// 对齐 Java 类: `cn.hutool.core.swing.clipboard.ClipboardUtil`
#[derive(Debug, Clone, Copy, Default)]
pub struct ClipboardUtil;

impl ClipboardUtil {
    /// 对齐 Java: `setStr(String)`
    pub fn set_str(value: &str) {
        text_store()
            .lock()
            .expect("clipboard lock")
            .replace(value.to_string());
        let snapshot = Self::get_str().unwrap_or_default();
        for listener in listeners().lock().expect("listener lock").iter() {
            listener(&snapshot);
        }
    }

    /// 对齐 Java: `getStr()`
    pub fn get_str() -> Option<String> {
        text_store().lock().expect("clipboard lock").clone()
    }

    /// 对齐 Java: `listen(ClipboardListener, boolean)` 简化版。
    pub fn listen<F>(listener: F, _sync: bool)
    where
        F: Fn(&str) + Send + Sync + 'static,
    {
        listeners()
            .lock()
            .expect("listener lock")
            .push(Box::new(listener));
    }

    /// 测试辅助：清空剪贴板与监听。
    pub fn reset_for_test() {
        if let Some(store) = TEXT_CLIPBOARD.get() {
            *store.lock().expect("clipboard lock") = None;
        }
        if let Some(list) = LISTENERS.get() {
            list.lock().expect("listener lock").clear();
        }
    }
}
