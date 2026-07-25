//! 对齐: `cn.hutool.http` (HTML工具模块)
//! 来源: hutool-http/src/main/java/cn/hutool/http/HtmlUtil.java
//! 中文说明: HTML工具模块，提供HTML标签处理和XSS过滤功能

mod filter;
mod util;

pub use filter::HtmlFilter;
pub use util::HtmlUtil;
