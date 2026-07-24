//! 对齐: `cn.hutool.core.lang.intern.InternUtil`

use parking_lot::Mutex;
use std::collections::HashMap;
use std::sync::Arc;

/// 弱引用风格字符串驻留（强引用表近似）
pub struct WeakInterner {
    map: Mutex<HashMap<String, Arc<String>>>,
}

impl WeakInterner {
    /// 创建
    pub fn new() -> Self {
        Self {
            map: Mutex::new(HashMap::new()),
        }
    }

    /// 对齐 `intern`
    pub fn intern(&self, s: &str) -> Arc<String> {
        let mut g = self.map.lock();
        if let Some(v) = g.get(s) {
            return Arc::clone(v);
        }
        let a = Arc::new(s.to_string());
        g.insert(s.to_string(), Arc::clone(&a));
        a
    }
}

impl Default for WeakInterner {
    fn default() -> Self {
        Self::new()
    }
}
