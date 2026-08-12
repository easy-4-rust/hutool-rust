//! 对齐: `cn.hutool.http.useragent` (规则错误部分)
//! 来源: hutool-http/src/main/java/cn/hutool/http/useragent/
//! 中文说明: User-Agent规则错误类型，封装正则表达式编译错误

/// Errors produced while registering a custom User-Agent rule.
pub type RuleError = regex::Error;
