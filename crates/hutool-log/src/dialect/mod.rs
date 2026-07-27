//! 对齐: `cn.hutool.log.dialect` 子包
//! 来源: hutool-log/src/main/java/cn/hutool/log/dialect/
//!
//! 中文说明: Hutool 日志方言子包模块，对齐 Java `cn.hutool.log.dialect.*`。
//! 子包包含 9 个厂商（commons/console/jboss/jdk/log4j/log4j2/logtube/slf4j/tinylog），
//! 每个厂商的 .rs 文件提供该厂商所有 Java 类的类型别名。

pub mod commons;
pub mod console;
pub mod jboss;
pub mod jdk;
pub mod log4j;
pub mod log4j2;
pub mod logtube;
pub mod slf4j;
pub mod tinylog;

pub use commons::{
    ApacheCommonsLog, ApacheCommonsLog4JLog, ApacheCommonsLogFactory,
};
pub use console::{
    ConsoleColorLog, ConsoleColorLogFactory, ConsoleLog, ConsoleLogFactory,
};
pub use jboss::{JbossLog, JbossLogFactory};
pub use jdk::{JdkLog, JdkLogFactory};
pub use log4j::{Log4jLog, Log4jLogFactory};
pub use log4j2::{Log4j2Log, Log4j2LogFactory};
pub use logtube::{LogTubeLog, LogTubeLogFactory};
pub use slf4j::{Slf4jLog, Slf4jLogFactory};
pub use tinylog::{TinyLog, TinyLog2, TinyLog2Factory, TinyLogFactory};
