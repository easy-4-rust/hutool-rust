//! Hutool-aligned before/after aspects.

use crate::Method;
use parking_lot::Mutex;
use std::{
    any::type_name,
    collections::HashMap,
    fmt,
    sync::Arc,
    thread::{self, ThreadId},
    time::{Duration, Instant},
};

use super::aspect::Aspect;
use super::timing_event::TimingEvent;

/// An aspect that measures successful invocation time.
#[derive(Clone)]
pub struct TimeIntervalAspect {
    started: Arc<Mutex<HashMap<ThreadId, Vec<Instant>>>>,
    last_elapsed: Arc<Mutex<Duration>>,
    sink: Arc<TimingSink>,
}

impl fmt::Debug for TimeIntervalAspect {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("TimeIntervalAspect")
            .field("last_elapsed", &self.last_elapsed())
            .finish_non_exhaustive()
    }
}

impl Default for TimeIntervalAspect {
    fn default() -> Self {
        Self::with_sink(|_| {})
    }
}

impl TimeIntervalAspect {
    /// Creates an aspect with a no-op event sink.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates an aspect that reports completed timings to `sink`.
    pub fn with_sink<F>(sink: F) -> Self
    where
        F: Fn(&TimingEvent) + Send + Sync + 'static,
    {
        Self {
            started: Arc::new(Mutex::new(HashMap::new())),
            last_elapsed: Arc::new(Mutex::new(Duration::ZERO)),
            sink: Arc::new(sink),
        }
    }

    /// Returns the most recently completed duration.
    #[must_use]
    pub fn last_elapsed(&self) -> Duration {
        *self.last_elapsed.lock()
    }

    fn take_elapsed(&self) -> Duration {
        let thread = thread::current().id();
        let mut starts = self.started.lock();
        let elapsed = starts
            .get_mut(&thread)
            .and_then(Vec::pop)
            .map_or(Duration::ZERO, |started| started.elapsed());
        if starts.get(&thread).is_some_and(Vec::is_empty) {
            starts.remove(&thread);
        }
        elapsed
    }
}

impl<T, A, R: fmt::Debug, E> Aspect<T, A, R, E> for TimeIntervalAspect {
    fn before(&self, _target: &T, _method: &Method, _args: &A) -> bool {
        self.started
            .lock()
            .entry(thread::current().id())
            .or_default()
            .push(Instant::now());
        true
    }

    fn after(&self, _target: &T, method: &Method, _args: &A, return_value: Option<&R>) -> bool {
        let elapsed = self.take_elapsed();
        *self.last_elapsed.lock() = elapsed;
        (self.sink)(&TimingEvent {
            target_type: type_name::<T>(),
            method: method.name().to_owned(),
            elapsed,
            return_value: return_value.map(|value| format!("{value:?}")),
        });
        true
    }
}

type TimingSink = dyn Fn(&TimingEvent) + Send + Sync;
