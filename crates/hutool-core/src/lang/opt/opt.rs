//! 对齐: `cn.hutool.core.lang.Opt`
//! 来源: hutool-core/src/main/java/cn/hutool/core/lang/Opt.java
//!
//! Hutool `Opt` 的 idiomatic Rust 实现：在 [`Option`] 之上附加可选异常上下文。

use crate::string::is_blank;
use std::fmt;

use super::opt_empty_error::OptEmptyError;

/// 对齐 Java: `cn.hutool.core.lang.Opt`
#[derive(Clone)]
pub struct Opt<T> {
    value: Option<T>,
    exception: Option<String>,
}

impl<T> Default for Opt<T> {
    fn default() -> Self {
        Self::empty()
    }
}

impl<T> Opt<T> {
    /// 对齐 Java: `Opt.empty()`
    #[must_use]
    pub fn empty() -> Self {
        Self {
            value: None,
            exception: None,
        }
    }

    /// 对齐 Java: `Opt.of(T)` — 要求值存在。
    #[must_use]
    pub fn of(value: T) -> Self {
        Self {
            value: Some(value),
            exception: None,
        }
    }

    /// 对齐 Java: `Opt.ofNullable(T)`
    #[must_use]
    pub fn of_nullable(value: Option<T>) -> Self {
        Self {
            value,
            exception: None,
        }
    }

    /// 对齐 Java: `Opt.ofTry(Func0)` — 闭包失败时记录异常消息并返回 empty。
    #[must_use]
    pub fn of_try<E: fmt::Display>(supplier: impl FnOnce() -> Result<T, E>) -> Self {
        match supplier() {
            Ok(v) => Self::of(v),
            Err(e) => Self {
                value: None,
                exception: Some(e.to_string()),
            },
        }
    }

    /// 对齐 Java: `Opt.get()`
    pub fn get(&self) -> Option<&T> {
        self.value.as_ref()
    }

    /// 取出内部值。
    pub fn into_inner(self) -> Option<T> {
        self.value
    }

    /// 对齐 Java: `Opt.isEmpty()`
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.value.is_none()
    }

    /// 对齐 Java: `Opt.isPresent()`
    #[must_use]
    pub fn is_present(&self) -> bool {
        self.value.is_some()
    }

    /// 对齐 Java: `Opt.getException()`
    pub fn get_exception(&self) -> Option<&str> {
        self.exception.as_deref()
    }

    /// 对齐 Java: `Opt.isFail()`
    #[must_use]
    pub fn is_fail(&self) -> bool {
        self.exception.is_some()
    }

    /// 对齐 Java: `Opt.ifFail(Consumer)`
    pub fn if_fail(self, action: impl FnOnce(&str)) -> Self {
        if let Some(ref e) = self.exception {
            action(e);
        }
        self
    }

    /// 对齐 Java: `Opt.ifPresent(Consumer)`
    pub fn if_present(self, action: impl FnOnce(&T)) -> Self {
        if let Some(ref v) = self.value {
            action(v);
        }
        self
    }

    /// 对齐 Java: `Opt.ifPresentOrElse(Consumer, VoidFunc0)`
    pub fn if_present_or_else(self, action: impl FnOnce(&T), empty: impl FnOnce()) -> Self {
        match &self.value {
            Some(v) => action(v),
            None => empty(),
        }
        self
    }

    /// 对齐 Java: `Opt.mapOrElse(Function, VoidFunc0)`
    pub fn map_or_else<U>(
        self,
        mapper: impl FnOnce(T) -> U,
        empty: impl FnOnce(),
    ) -> Opt<U> {
        match self.value {
            Some(v) => Opt {
                value: Some(mapper(v)),
                exception: self.exception,
            },
            None => {
                empty();
                Opt {
                    value: None,
                    exception: self.exception,
                }
            }
        }
    }

    /// 对齐 Java: `Opt.filter(Predicate)`
    pub fn filter(self, predicate: impl FnOnce(&T) -> bool) -> Self {
        match self.value {
            Some(ref v) if predicate(v) => self,
            Some(_) => Self {
                value: None,
                exception: self.exception,
            },
            None => self,
        }
    }

    /// 对齐 Java: `Opt.map(Function)`
    pub fn map<U>(self, mapper: impl FnOnce(T) -> U) -> Opt<U> {
        Opt {
            value: self.value.map(mapper),
            exception: self.exception,
        }
    }

    /// 对齐 Java: `Opt.flatMap(Function)`
    pub fn flat_map<U>(self, mapper: impl FnOnce(T) -> Opt<U>) -> Opt<U> {
        match self.value {
            Some(v) => {
                let mut next = mapper(v);
                if next.exception.is_none() {
                    next.exception = self.exception;
                }
                next
            }
            None => Opt {
                value: None,
                exception: self.exception,
            },
        }
    }

    /// 对齐 Java: `Opt.flattedMap` — 映射到 `Option`。
    pub fn flatted_map<U>(self, mapper: impl FnOnce(T) -> Option<U>) -> Opt<U> {
        Opt {
            value: self.value.and_then(mapper),
            exception: self.exception,
        }
    }

    /// 对齐 Java: `Opt.peek(Consumer)`
    pub fn peek(self, action: impl FnOnce(&T)) -> Self {
        if let Some(ref v) = self.value {
            action(v);
        }
        self
    }

    /// 对齐 Java: `Opt.peeks(Consumer...)`
    pub fn peeks(self, actions: &[fn(&T)]) -> Self {
        if let Some(ref v) = self.value {
            for action in actions {
                action(v);
            }
        }
        self
    }

    /// 对齐 Java: `Opt.or(Supplier)`
    pub fn or(self, supplier: impl FnOnce() -> Opt<T>) -> Opt<T> {
        if self.is_present() {
            self
        } else {
            supplier()
        }
    }

    /// 对齐 Java: `Opt.stream()` — 0 或 1 个元素的迭代器。
    pub fn stream(self) -> std::option::IntoIter<T> {
        self.value.into_iter()
    }

    /// 对齐 Java: `Opt.orElse(T)`
    pub fn or_else(self, other: T) -> T {
        self.value.unwrap_or(other)
    }

    /// 对齐 Java: `Opt.exceptionOrElse(T)` — 失败时返回兜底值，否则取内部值（可空则 panic 对齐 NSEE）。
    pub fn exception_or_else(self, other: T) -> T {
        if self.is_fail() {
            other
        } else {
            self.value.expect("Opt.exceptionOrElse on empty without fail")
        }
    }

    /// 对齐 Java: `Opt.orElseGet(Supplier)`
    pub fn or_else_get(self, supplier: impl FnOnce() -> T) -> T {
        self.value.unwrap_or_else(supplier)
    }

    /// 对齐 Java: `Opt.orElseThrow()`
    pub fn or_else_throw(self) -> Result<T, OptEmptyError> {
        self.value.ok_or(OptEmptyError)
    }

    /// 对齐 Java: `Opt.orElseThrow(Supplier)`
    pub fn or_else_throw_with<E>(self, supplier: impl FnOnce() -> E) -> Result<T, E> {
        self.value.ok_or_else(supplier)
    }

    /// 对齐 Java: `Opt.orElseThrow(Function, String)`
    pub fn or_else_throw_msg<E>(
        self,
        mapper: impl FnOnce(String) -> E,
        message: impl Into<String>,
    ) -> Result<T, E> {
        self.value.ok_or_else(|| mapper(message.into()))
    }

    /// 对齐 Java: `Opt.toOptional()`
    pub fn to_optional(self) -> Option<T> {
        self.value
    }
}

impl Opt<String> {
    /// 对齐 Java: `Opt.ofBlankAble(T)` — 空白字符串视为 empty。
    #[must_use]
    pub fn of_blank_able(value: Option<String>) -> Self {
        match value {
            Some(s) if !is_blank(&s) => Self::of(s),
            _ => Self::empty(),
        }
    }
}

impl<T> Opt<Vec<T>> {
    /// 对齐 Java: `Opt.ofEmptyAble(Collection)`
    #[must_use]
    pub fn of_empty_able(value: Option<Vec<T>>) -> Self {
        match value {
            Some(v) if !v.is_empty() => Self::of(v),
            _ => Self::empty(),
        }
    }
}

impl<T: PartialEq> PartialEq for Opt<T> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<T: Eq> Eq for Opt<T> {}

impl<T: std::hash::Hash> std::hash::Hash for Opt<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

impl<T: fmt::Debug> fmt::Debug for Opt<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.value {
            Some(v) => write!(f, "Opt[{v:?}]"),
            None => write!(f, "Opt.empty"),
        }
    }
}

impl<T: fmt::Display> fmt::Display for Opt<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.value {
            Some(v) => write!(f, "Opt[{v}]"),
            None => write!(f, "Opt.empty"),
        }
    }
}
