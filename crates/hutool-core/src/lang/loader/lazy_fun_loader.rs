//! 对齐: `cn.hutool.core.lang.loader.LazyFunLoader`

#![allow(dead_code)] // 对齐 Java LazyFunLoader，暂未接线，预留

use parking_lot::Mutex;
use std::sync::Arc;

/// 对齐 Java: `LazyFunLoader<T>`
pub struct LazyFunLoader<T> {
    factory: Mutex<Option<Box<dyn FnOnce() -> T + Send>>>,
    value: Mutex<Option<Arc<T>>>,
}

impl<T: Send + Sync + 'static> LazyFunLoader<T> {
    /// 对齐构造 `new LazyFunLoader<>(supplier)`
    pub fn new<F>(factory: F) -> Self
    where
        F: FnOnce() -> T + Send + 'static,
    {
        Self {
            factory: Mutex::new(Some(Box::new(factory))),
            value: Mutex::new(None),
        }
    }

    /// 对齐 `on(supplier)`
    pub fn on<F>(factory: F) -> Self
    where
        F: FnOnce() -> T + Send + 'static,
    {
        Self::new(factory)
    }

    /// 对齐 `get`
    pub fn get(&self) -> Arc<T> {
        {
            let v = self.value.lock();
            if let Some(ref a) = *v {
                return Arc::clone(a);
            }
        }
        let factory = self
            .factory
            .lock()
            .take()
            .expect("factory already consumed");
        let arc = Arc::new(factory());
        *self.value.lock() = Some(Arc::clone(&arc));
        arc
    }

    /// 对齐 `isInitialize`
    pub fn is_initialize(&self) -> bool {
        self.value.lock().is_some()
    }

    /// 对齐 `ifInitialized`
    pub fn if_initialized<F: FnOnce(&T)>(&self, f: F) {
        if let Some(ref a) = *self.value.lock() {
            f(a);
        }
    }
}
