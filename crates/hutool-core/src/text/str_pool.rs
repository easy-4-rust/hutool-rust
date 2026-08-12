//! 对齐: `cn.hutool.core.text.StrPool`
//! 来源: hutool-core/src/main/java/cn/hutool/core/text/StrPool.java
//!
//! 常用字符串常量定义(Rust 形式: 常量模块)。

#![allow(dead_code)]

// ---------- 字符常量(C_*) ----------

/// 字符常量：空格符
pub const C_SPACE: char = ' ';
/// 字符常量：制表符
pub const C_TAB: char = '\t';
/// 字符常量：点
pub const C_DOT: char = '.';
/// 字符常量：斜杠
pub const C_SLASH: char = '/';
/// 字符常量：反斜杠
pub const C_BACKSLASH: char = '\\';
/// 字符常量：回车符
pub const C_CR: char = '\r';
/// 字符常量：换行符
pub const C_LF: char = '\n';
/// 字符常量：下划线
pub const C_UNDERLINE: char = '_';
/// 字符常量：逗号
pub const C_COMMA: char = ',';
/// 字符常量：花括号（左）
pub const C_DELIM_START: char = '{';
/// 字符常量：花括号（右）
pub const C_DELIM_END: char = '}';
/// 字符常量：中括号（左）
pub const C_BRACKET_START: char = '[';
/// 字符常量：中括号（右）
pub const C_BRACKET_END: char = ']';
/// 字符常量：冒号
pub const C_COLON: char = ':';
/// 字符常量：艾特
pub const C_AT: char = '@';

// ---------- 字符串常量 ----------

/// 制表符 `"\t"`
pub const TAB: &str = "\t";
/// 点 `"."`
pub const DOT: &str = ".";
/// 双点 `".."`
pub const DOUBLE_DOT: &str = "..";
/// 斜杠 `"/"`
pub const SLASH: &str = "/";
/// 反斜杠 `"\\"`
pub const BACKSLASH: &str = "\\";
/// 回车符 `"\r"`
pub const CR: &str = "\r";
/// 换行符 `"\n"`
pub const LF: &str = "\n";
/// Windows 换行 `"\r\n"`
pub const CRLF: &str = "\r\n";
/// 下划线 `"_"`
pub const UNDERLINE: &str = "_";
/// 减号（连接符） `"-"`
pub const DASHED: &str = "-";
/// 逗号 `","`
pub const COMMA: &str = ",";
/// 花括号（左） `"{`
pub const DELIM_START: &str = "{";
/// 花括号（右） `"}"`
pub const DELIM_END: &str = "}";
/// 中括号（左） `"["`
pub const BRACKET_START: &str = "[";
/// 中括号（右） `"]"`
pub const BRACKET_END: &str = "]";
/// 冒号 `":"`
pub const COLON: &str = ":";
/// 艾特 `"@"`
pub const AT: &str = "@";
/// HTML 不间断空格转义 `"&nbsp;"`
pub const HTML_NBSP: &str = "&nbsp;";
/// HTML And 符转义 `"&amp;"`
pub const HTML_AMP: &str = "&amp;";
/// HTML 双引号转义 `"&quot;"`
pub const HTML_QUOTE: &str = "&quot;";
/// HTML 单引号转义 `"&apos;"`
pub const HTML_APOS: &str = "&apos;";
/// HTML 小于号转义 `"&lt;"`
pub const HTML_LT: &str = "&lt;";
/// HTML 大于号转义 `"&gt;"`
pub const HTML_GT: &str = "&gt;";
/// 空 JSON `"{}"`
pub const EMPTY_JSON: &str = "{}";
