use std::{
    collections::HashMap,
    fmt,
    sync::{Arc, RwLock},
};

use crate::abstract_log::AbstractLog;
use crate::global_log_factory::GlobalLogFactory;
use crate::log::Log;
use crate::log_sink::LogSink;
use crate::tracing_sink::TracingSink;

/// Logger factory with thread-safe instance caching and no implicit global state.
#[derive(Clone)]
pub struct LogFactory {
    name: String,
    sink: Arc<dyn LogSink>,
    cache: Arc<RwLock<HashMap<String, Arc<dyn Log>>>>,
}

impl fmt::Debug for LogFactory {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("LogFactory")
            .field("name", &self.name)
            .finish_non_exhaustive()
    }
}

impl Default for LogFactory {
    fn default() -> Self {
        Self::new("tracing", Arc::new(TracingSink))
    }
}

impl LogFactory {
    /// Creates an independently owned factory.
    #[must_use]
    pub fn new(name: &str, sink: Arc<dyn LogSink>) -> Self {
        Self {
            name: name.to_owned(),
            sink,
            cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    /// Creates a factory with a dialect display name and the default tracing sink.
    ///
    /// 对齐 Java: dialect `LogFactory` constructors such as `new ConsoleLogFactory()`.
    #[must_use]
    pub fn named(name: &str) -> Self {
        Self::new(name, Arc::new(TracingSink))
    }
    /// Returns the factory/backend name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }
    /// Gets or creates one named logger.
    ///
    /// # Panics
    ///
    /// Panics when another thread poisoned the internal logger-cache lock.
    #[must_use]
    pub fn get(&self, name: &str) -> Arc<dyn Log> {
        if let Some(logger) = self
            .cache
            .read()
            .expect("log cache read lock poisoned")
            .get(name)
        {
            return Arc::clone(logger);
        }
        let mut cache = self.cache.write().expect("log cache write lock poisoned");
        Arc::clone(
            cache
                .entry(name.to_owned())
                .or_insert_with(|| Arc::new(AbstractLog::new(name, Arc::clone(&self.sink)))),
        )
    }
    /// Creates an uncached logger.
    #[must_use]
    pub fn create(&self, name: &str) -> Arc<dyn Log> {
        Arc::new(AbstractLog::new(name, Arc::clone(&self.sink)))
    }
    #[must_use]
    /// Returns the number of cached logger instances.
    ///
    /// # Panics
    ///
    /// Panics when another thread poisoned the internal logger-cache lock.
    pub fn cached_len(&self) -> usize {
        self.cache
            .read()
            .expect("log cache read lock poisoned")
            .len()
    }
    /// Removes all cached logger instances.
    ///
    /// # Panics
    ///
    /// Panics when another thread poisoned the internal logger-cache lock.
    pub fn clear(&self) {
        self.cache
            .write()
            .expect("log cache write lock poisoned")
            .clear()
    }
    /// 对齐 Java: `LogFactory.setCurrentLogFactory(LogFactory)`.
    pub fn set_current(factory: LogFactory) -> LogFactory {
        GlobalLogFactory::set(factory)
    }
    /// 对齐 Java: `LogFactory.get()` — returns a logger from the process-wide factory.
    #[must_use]
    pub fn get_current() -> Arc<dyn Log> {
        GlobalLogFactory::get().get("default")
    }
    /// 对齐 Java: `LogFactory.get(String name)`.
    #[must_use]
    pub fn get_by_name(name: &str) -> Arc<dyn Log> {
        GlobalLogFactory::get().get(name)
    }
}
