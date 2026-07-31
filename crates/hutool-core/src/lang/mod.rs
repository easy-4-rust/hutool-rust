//! `cn.hutool.core.lang` 子包对齐（行为可运行子集）
//!
//! Dict/Validator 由 crate 根模块提供完整实现；此处导出 Assert/Opt/Pair 等 lang 核心类型，
//! 并保留已实现的 ansi/tree/snowflake 等子模块。磁盘上未 `mod` 的 Java 签名桩不参与编译。

pub mod ansi;
pub mod assert_;
pub mod caller;
pub mod chain;
pub mod class_scanner;
pub mod console;
pub mod console_table;
pub mod consistent_hash;
pub mod default_segment;
pub mod editor;
pub mod enum_item;
pub mod filter;
pub mod func;
pub mod hash;
pub mod id;
pub mod intern;
pub mod loader;
pub mod matcher;
pub mod object_id;
pub mod opt;
pub mod pair;
pub mod pattern_pool;
pub mod pid;
pub mod range;
pub mod reflect;
pub mod regex_pool;
pub mod replacer;
pub mod segment;
pub mod simple_cache;
pub mod singleton;
pub mod snowflake;
pub mod tree;
pub mod tuple;
pub mod uuid_fast;
pub mod version;
pub mod weight_list_random;
pub mod weight_random;

pub use object_id::ObjectId;
pub use snowflake::Snowflake;

