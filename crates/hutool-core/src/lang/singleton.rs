//! 对齐: `cn.hutool.core.lang.Singleton`
//! `ReentrantMutex` + `RefCell`：同线程构造嵌套 `get` 不死锁，且同类型只创建一次。

#![allow(dead_code)] // 对齐 Java Singleton，暂未接线，预留

use parking_lot::ReentrantMutex;
use std::any::{Any, TypeId};
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::Arc;

type StoreMap = HashMap<TypeId, Arc<dyn Any + Send + Sync>>;

static STORE: std::sync::OnceLock<ReentrantMutex<RefCell<StoreMap>>> = std::sync::OnceLock::new();

fn store() -> &'static ReentrantMutex<RefCell<StoreMap>> {
    STORE.get_or_init(|| ReentrantMutex::new(RefCell::new(HashMap::new())))
}

/// 对齐 Java: `Singleton`
pub struct Singleton;

impl Singleton {
    /// 对齐 `Singleton.get(Class)` — 按类型懒创建（可重入）
    pub fn get<T, F>(factory: F) -> Arc<T>
    where
        T: Send + Sync + 'static,
        F: FnOnce() -> T,
    {
        let tid = TypeId::of::<T>();
        let guard = store().lock();
        if let Some(v) = guard.borrow().get(&tid) {
            return Arc::clone(v).downcast::<T>().expect("type mismatch");
        }
        // 持有可重入锁调用 factory；嵌套 get 其他 TypeId 可再次 lock
        let created = Arc::new(factory());
        let mut map = guard.borrow_mut();
        if let Some(v) = map.get(&tid) {
            return Arc::clone(v).downcast::<T>().expect("type mismatch");
        }
        map.insert(tid, created.clone());
        created
    }

    /// 对齐 `put`
    pub fn put<T: Send + Sync + 'static>(obj: T) {
        store()
            .lock()
            .borrow_mut()
            .insert(TypeId::of::<T>(), Arc::new(obj));
    }

    /// 对齐 `exists`
    pub fn exists<T: 'static>() -> bool {
        store().lock().borrow().contains_key(&TypeId::of::<T>())
    }

    /// 对齐 `remove`
    pub fn remove<T: 'static>() {
        store().lock().borrow_mut().remove(&TypeId::of::<T>());
    }

    /// 对齐 `destroy`
    pub fn destroy() {
        store().lock().borrow_mut().clear();
    }
}

#[cfg(test)]
mod singleton_idiomatic_parity {
    use super::*;

    #[derive(Debug)]
    struct Demo(i32);

    /// 对齐 Java Singleton get/put/exists/remove 可执行证据。
    #[test]
    fn singleton_get_put_exists_remove() {
        Singleton::remove::<Demo>();
        assert!(!Singleton::exists::<Demo>());
        let a = Singleton::get(|| Demo(7));
        let b = Singleton::get(|| Demo(99));
        assert_eq!(a.0, 7);
        assert_eq!(b.0, 7);
        assert!(Singleton::exists::<Demo>());
        Singleton::put(Demo(3));
        assert_eq!(Singleton::get(|| Demo(0)).0, 3);
        Singleton::remove::<Demo>();
        assert!(!Singleton::exists::<Demo>());
    }
}
