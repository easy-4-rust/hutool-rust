//! Hutool `hutool-log` TEST parity —— 对齐 Java `cn.hutool.log.test.*`。
//!
//! 对齐: `cn.hutool.log.test.CustomLogTest`
//! 对齐: `cn.hutool.log.test.LogTest`
//! 对齐: `cn.hutool.log.test.LogTubeTest`
//! 对齐: `cn.hutool.log.test.StaticLogTest`
//!
//! Dialect note: Hutool backends (console / commons / tinylog / log4j / jboss / jdk /
//! slf4j / logtube) are migration aliases over one injectable `LogSink` (default:
//! tracing). Tests assert factory names, `{}` formatting, and null handling — not
//! third-party logger wiring.

use std::fmt;
use std::sync::{Arc, Mutex, OnceLock};

use hutool_log::dialect::commons::ApacheCommonsLogFactory;
use hutool_log::dialect::console::{ConsoleColorLogFactory, ConsoleLogFactory};
use hutool_log::dialect::jboss::JbossLogFactory;
use hutool_log::dialect::jdk::JdkLogFactory;
use hutool_log::dialect::log4j::Log4jLogFactory;
use hutool_log::dialect::log4j2::Log4j2LogFactory;
use hutool_log::dialect::logtube::LogTubeLogFactory;
use hutool_log::dialect::slf4j::Slf4jLogFactory;
use hutool_log::dialect::tinylog::{TinyLog2Factory, TinyLogFactory};
use hutool_log::level::Level;
use hutool_log::{
    GlobalLogFactory, LogFactory, LogLevel, LogRecord, LogSink, StaticLog, format_message,
};

const LINE: &str = "----------------------------------------------------------------------";

/// Serializes tests that mutate the process-wide [`GlobalLogFactory`].
fn global_lock() -> &'static Mutex<()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
}

/// Test sink that records every accepted [`LogRecord`].
#[derive(Default)]
struct RecordingSink {
    records: Mutex<Vec<LogRecord>>,
}

impl RecordingSink {
    /// Returns a snapshot of emitted records.
    fn records(&self) -> Vec<LogRecord> {
        self.records.lock().expect("records lock poisoned").clone()
    }

    /// Clears recorded events between assertions.
    fn clear(&self) {
        self.records.lock().expect("records lock poisoned").clear();
    }
}

impl LogSink for RecordingSink {
    fn emit(&self, record: &LogRecord) {
        self.records
            .lock()
            .expect("records lock poisoned")
            .push(record.clone());
    }
}

/// Installs `factory` as the global factory for the duration of `body`, then restores.
fn with_global_factory<R>(factory: LogFactory, body: impl FnOnce() -> R) -> R {
    let _guard = global_lock().lock().expect("global log lock poisoned");
    let previous = GlobalLogFactory::set(factory);
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(body));
    GlobalLogFactory::set(previous);
    match result {
        Ok(value) => value,
        Err(payload) => std::panic::resume_unwind(payload),
    }
}

/// Builds a dialect factory with Hutool's display name and a recording sink.
fn named_recording(name: &str) -> (LogFactory, Arc<RecordingSink>) {
    let sink = Arc::new(RecordingSink::default());
    let factory = LogFactory::new(name, sink.clone());
    (factory, sink)
}

/// Asserts Hutool-style templated custom-log line: `This is custom '{name}' log\n{LINE}`.
fn assert_custom_info_line(record: &LogRecord, factory_name: &str) {
    let expected = format!("This is custom '{factory_name}' log\n{LINE}");
    assert_eq!(record.level, LogLevel::Info);
    assert_eq!(record.message, expected);
}

/// Runs the CustomLogTest pattern: set factory, log nulls (optional), then templated info.
fn run_custom_dialect(
    factory_name: &str,
    include_nulls: bool,
    include_debug: bool,
) -> Vec<LogRecord> {
    let (factory, sink) = named_recording(factory_name);
    assert_eq!(factory.name(), factory_name);
    with_global_factory(factory, || {
        let log = LogFactory::get_current();
        if include_debug {
            log.log_nullable(LogLevel::Debug, None);
            log.debug_fmt("This is custom '{}' log\n{}", &[&factory_name, &LINE]);
        }
        if include_nulls {
            log.log_nullable(LogLevel::Info, None);
            log.log_nullable(LogLevel::Info, None);
        }
        log.info_fmt("This is custom '{}' log\n{}", &[&factory_name, &LINE]);
    });
    sink.records()
}

// ---------------------------------------------------------------------------
// CustomLogTest
// ---------------------------------------------------------------------------

/// 对齐 Java: `CustomLogTest.consoleLogTest()`
#[test]
fn console_log_test() {
    let name = "Hutool Console Logging";
    let (factory, sink) = named_recording(name);
    // Type alias keeps the Hutool dialect name visible at the call site.
    let factory: ConsoleLogFactory = factory;
    assert_eq!(factory.name(), name);
    with_global_factory(factory, || {
        let log = LogFactory::get_current();
        log.info_fmt("This is custom '{}' log\n{}", &[&name, &LINE]);
    });
    let records = sink.records();
    assert_eq!(records.len(), 1);
    assert_custom_info_line(&records[0], name);
}

/// 对齐 Java: `CustomLogTest.consoleLogNullTest()`
#[test]
fn console_log_null_test() {
    let name = "Hutool Console Logging";
    let (factory, sink) = named_recording(name);
    let factory: ConsoleLogFactory = factory;
    with_global_factory(factory, || {
        let log = LogFactory::get_current();
        log.log_nullable(LogLevel::Info, None);
        log.log_nullable(LogLevel::Info, None);
    });
    let records = sink.records();
    assert_eq!(records.len(), 2);
    assert_eq!(records[0].message, "null");
    assert_eq!(records[1].message, "null");
}

/// 对齐 Java: `CustomLogTest.commonsLogTest()`
#[test]
fn commons_log_test() {
    let name = "Apache Common Logging";
    let _: ApacheCommonsLogFactory = LogFactory::named(name);
    let records = run_custom_dialect(name, true, false);
    assert_eq!(records.len(), 3);
    assert_eq!(records[0].message, "null");
    assert_eq!(records[1].message, "null");
    assert_custom_info_line(&records[2], name);
}

/// 对齐 Java: `CustomLogTest.tinyLogTest()`
#[test]
fn tiny_log_test() {
    let name = "TinyLog";
    let _: TinyLogFactory = LogFactory::named(name);
    let records = run_custom_dialect(name, true, false);
    assert_eq!(records.len(), 3);
    assert_custom_info_line(&records[2], name);
}

/// 对齐 Java: `CustomLogTest.tinyLog2Test()`
#[test]
fn tiny_log2_test() {
    let name = "TinyLog";
    let _: TinyLog2Factory = LogFactory::named(name);
    let records = run_custom_dialect(name, true, false);
    assert_eq!(records.len(), 3);
    assert_custom_info_line(&records[2], name);
}

/// 对齐 Java: `CustomLogTest.log4j2LogTest()`
///
/// Java 顺序：debug(null)、debug(template)、info(null)、info(null)、info(template) → 共 5 条。
#[test]
fn log4j2_log_test() {
    let name = "Log4j2";
    let _: Log4j2LogFactory = LogFactory::named(name);
    let records = run_custom_dialect(name, true, true);
    assert_eq!(records.len(), 5);
    assert_eq!(records[0].level, LogLevel::Debug);
    assert_eq!(records[0].message, "null");
    assert_eq!(
        records[1].message,
        format!("This is custom '{name}' log\n{LINE}")
    );
    assert_eq!(records[1].level, LogLevel::Debug);
    assert_eq!(records[2].message, "null");
    assert_eq!(records[3].message, "null");
    assert_custom_info_line(&records[4], name);
}

/// 对齐 Java: `CustomLogTest.log4jLogTest()`
#[test]
fn log4j_log_test() {
    let name = "Log4j";
    let _: Log4jLogFactory = LogFactory::named(name);
    let records = run_custom_dialect(name, true, false);
    assert_eq!(records.len(), 3);
    assert_custom_info_line(&records[2], name);
}

/// 对齐 Java: `CustomLogTest.jbossLogTest()`
#[test]
fn jboss_log_test() {
    let name = "JBoss Logging";
    let _: JbossLogFactory = LogFactory::named(name);
    let records = run_custom_dialect(name, true, false);
    assert_eq!(records.len(), 3);
    assert_custom_info_line(&records[2], name);
}

/// 对齐 Java: `CustomLogTest.jdkLogTest()`
#[test]
fn jdk_log_test() {
    let name = "JDK Logging";
    let _: JdkLogFactory = LogFactory::named(name);
    let records = run_custom_dialect(name, true, false);
    assert_eq!(records.len(), 3);
    assert_custom_info_line(&records[2], name);
}

/// 对齐 Java: `CustomLogTest.slf4jTest()`
#[test]
fn slf4j_test() {
    // Java: `new Slf4jLogFactory(false)` — failIfNOP disabled; Rust keeps name-only alias.
    let name = "Slf4j";
    let _: Slf4jLogFactory = LogFactory::named(name);
    let records = run_custom_dialect(name, true, false);
    assert_eq!(records.len(), 3);
    assert_custom_info_line(&records[2], name);
}

// ---------------------------------------------------------------------------
// LogTest
// ---------------------------------------------------------------------------

/// 对齐 Java: `LogTest.logTest()`
#[test]
fn log_test() {
    let (factory, sink) = named_recording("tracing");
    with_global_factory(factory, || {
        let log = LogFactory::get_current();
        log.debug_fmt("This is {} log", &[&Level::Debug]);
        log.info_fmt("This is {} log", &[&Level::Info]);
        log.warn_fmt("This is {} log", &[&Level::Warn]);
    });
    let records = sink.records();
    assert_eq!(records.len(), 3);
    assert_eq!(records[0].message, "This is DEBUG log");
    assert_eq!(records[1].message, "This is INFO log");
    assert_eq!(records[2].message, "This is WARN log");
    assert_eq!(records[0].level, LogLevel::Debug);
    assert_eq!(records[1].level, LogLevel::Info);
    assert_eq!(records[2].level, LogLevel::Warn);
}

/// 对齐 Java: `LogTest.logWithExceptionTest()`（Java 侧 `@Disabled`，此处改为可断言）
#[test]
fn log_with_exception_test() {
    let (factory, sink) = named_recording("tracing");
    with_global_factory(factory, || {
        let log = LogFactory::get_current();
        log.error_with_cause("我是错误消息", "test Exception");
    });
    let records = sink.records();
    assert_eq!(records.len(), 1);
    assert_eq!(records[0].level, LogLevel::Error);
    assert_eq!(records[0].message, "我是错误消息");
    assert_eq!(records[0].error.as_deref(), Some("test Exception"));
}

/// 对齐 Java: `LogTest.logNullTest()`
#[test]
fn log_null_test() {
    let (factory, sink) = named_recording("tracing");
    with_global_factory(factory, || {
        // Java: `Log.get()` — process-wide default logger.
        let log = StaticLog::get_default();
        log.log_nullable(LogLevel::Debug, None);
        log.log_nullable(LogLevel::Info, None);
        log.log_nullable(LogLevel::Warn, None);
    });
    let records = sink.records();
    assert_eq!(records.len(), 3);
    assert_eq!(records[0].level, LogLevel::Debug);
    assert_eq!(records[1].level, LogLevel::Info);
    assert_eq!(records[2].level, LogLevel::Warn);
    assert!(records.iter().all(|r| r.message == "null"));
}

/// 对齐 Java: `LogTest.parameterizedMessageEdgeCasesTest()`
#[test]
fn parameterized_message_edge_cases_test() {
    let (factory, sink) = named_recording("tracing");
    with_global_factory(factory, || {
        let log = LogFactory::get_current();
        log.info("No parameters");
        log.info_fmt("One: {}", &[&"param1"]);
        log.info_fmt("Two: {} and {}", &[&"param1", &"param2"]);
        log.info_fmt("Three: {}, {}, {}", &[&"param1", &"param2", &"param3"]);
        log.info_fmt(
            "Four: {}, {}, {}, {}",
            &[&"param1", &"param2", &"param3", &"param4"],
        );
        // 参数不足：leftover `{}` retained (Hutool StrFormatter).
        log.info_fmt("Missing param: {} and {}", &[&"only_one"]);
        // 参数过多：extras dropped.
        log.info_fmt("Extra param: {}", &[&"param1", &"extra_param"]);
    });
    let messages: Vec<String> = sink.records().into_iter().map(|r| r.message).collect();
    assert_eq!(
        messages,
        vec![
            "No parameters".to_owned(),
            "One: param1".to_owned(),
            "Two: param1 and param2".to_owned(),
            "Three: param1, param2, param3".to_owned(),
            "Four: param1, param2, param3, param4".to_owned(),
            "Missing param: only_one and {}".to_owned(),
            "Extra param: param1".to_owned(),
        ]
    );
    // Direct formatter parity (same rules as logged messages).
    assert_eq!(
        format_message("Missing param: {} and {}", &[&"only_one"]),
        "Missing param: only_one and {}"
    );
    assert_eq!(
        format_message("Extra param: {}", &[&"param1", &"extra_param"]),
        "Extra param: param1"
    );
}

/// 对齐 Java: `LogTest.i18nMessageTest()`
#[test]
fn i18n_message_test() {
    let (factory, sink) = named_recording("tracing");
    with_global_factory(factory, || {
        let log = LogFactory::get_current();
        log.info("中文消息测试");
        log.info_fmt("Message with unicode: {}", &[&"特殊字符©®™✓✗★☆"]);
        log.info("多语言混排: 中文, English, 日本語, 한글");
        log.info("Emoji测试: 😀🚀🌏");
    });
    let messages: Vec<String> = sink.records().into_iter().map(|r| r.message).collect();
    assert_eq!(messages[0], "中文消息测试");
    assert_eq!(messages[1], "Message with unicode: 特殊字符©®™✓✗★☆");
    assert_eq!(messages[2], "多语言混排: 中文, English, 日本語, 한글");
    assert_eq!(messages[3], "Emoji测试: 😀🚀🌏");
}

/// Displays a slice like Java `List#toString`: `[a, b]`.
struct JavaListDisplay<'a, T: fmt::Display>(&'a [T]);

impl<T: fmt::Display> fmt::Display for JavaListDisplay<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        for (i, item) in self.0.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{item}")?;
        }
        write!(f, "]")
    }
}

/// Displays entries like Java `HashMap#toString`: `{key=value}`.
struct JavaMapDisplay<'a>(&'a [(&'a str, &'a dyn fmt::Display)]);

impl fmt::Display for JavaMapDisplay<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;
        for (i, (key, value)) in self.0.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{key}={value}")?;
        }
        write!(f, "}}")
    }
}

/// Literal `"null"` for a null object argument (Java `StringBuilder.append(null)`).
struct NullDisplay;

impl fmt::Display for NullDisplay {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("null")
    }
}

/// 对齐 Java: `LogTest.complexObjectTest()`
#[test]
fn complex_object_test() {
    let list = ["item1", "item2"];
    let map_entries: [(&str, &dyn fmt::Display); 1] = [("key", &"value")];
    let (factory, sink) = named_recording("tracing");
    with_global_factory(factory, || {
        let log = LogFactory::get_current();
        log.info_fmt("List: {}", &[&JavaListDisplay(&list)]);
        log.info_fmt("Map: {}", &[&JavaMapDisplay(&map_entries)]);
        log.info_fmt("Null object: {}", &[&NullDisplay]);
    });
    let messages: Vec<String> = sink.records().into_iter().map(|r| r.message).collect();
    assert_eq!(messages[0], "List: [item1, item2]");
    assert_eq!(messages[1], "Map: {key=value}");
    assert_eq!(messages[2], "Null object: null");
}

// ---------------------------------------------------------------------------
// LogTubeTest
// ---------------------------------------------------------------------------

/// 对齐 Java: `LogTubeTest.logTest()`
#[test]
fn log_tube_log_test() {
    let name = "LogTube";
    let (factory, sink) = named_recording(name);
    let factory: LogTubeLogFactory = factory;
    assert_eq!(factory.name(), name);
    with_global_factory(factory, || {
        let log = LogFactory::get_current();
        log.debug("LogTube debug test.");
    });
    let records = sink.records();
    assert_eq!(records.len(), 1);
    assert_eq!(records[0].level, LogLevel::Debug);
    assert_eq!(records[0].message, "LogTube debug test.");
}

// ---------------------------------------------------------------------------
// StaticLogTest
// ---------------------------------------------------------------------------

/// 对齐 Java: `StaticLogTest.test()`
#[test]
fn test() {
    let name = "Hutool Console Logging";
    let (factory, sink) = named_recording(name);
    let factory: ConsoleLogFactory = factory;
    with_global_factory(factory, || {
        StaticLog::debug_fmt("This is static {} log", &[&"debug"]);
        StaticLog::info_fmt("This is static {} log", &[&"info"]);
    });
    let records = sink.records();
    assert_eq!(records.len(), 2);
    assert_eq!(records[0].level, LogLevel::Debug);
    assert_eq!(records[0].message, "This is static debug log");
    assert_eq!(records[1].level, LogLevel::Info);
    assert_eq!(records[1].message, "This is static info log");
}

/// 对齐 Java: `StaticLogTest.colorTest()`
#[test]
fn color_test() {
    let name = "Hutool Console Color Logging";
    let (factory, sink) = named_recording(name);
    let factory: ConsoleColorLogFactory = factory;
    assert_eq!(factory.name(), name);
    with_global_factory(factory, || {
        StaticLog::debug_fmt("This is static {} log", &[&"debug"]);
        StaticLog::info_fmt("This is static {} log", &[&"info"]);
        StaticLog::error_fmt("This is static {} log", &[&"error"]);
        StaticLog::warn_fmt("This is static {} log", &[&"warn"]);
        StaticLog::trace_fmt("This is static {} log", &[&"trace"]);
    });
    let records = sink.records();
    assert_eq!(records.len(), 5);
    let expected = [
        (LogLevel::Debug, "This is static debug log"),
        (LogLevel::Info, "This is static info log"),
        (LogLevel::Error, "This is static error log"),
        (LogLevel::Warn, "This is static warn log"),
        (LogLevel::Trace, "This is static trace log"),
    ];
    for (record, (level, message)) in records.iter().zip(expected) {
        assert_eq!(record.level, level);
        assert_eq!(record.message, message);
    }
    // Color dialect is a name/factory alias; sink path matches plain console.
    sink.clear();
    assert!(sink.records().is_empty());
}
