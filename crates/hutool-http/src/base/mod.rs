//! 对齐: `cn.hutool.http.base.HttpBase`
//! 来源: hutool-http/src/main/java/cn/hutool/http/base/HttpBase.java
//!
//! Hutool 风格的请求和响应元数据共享。

mod http_base;
mod http_base_error;

pub use http_base::HttpBase;
pub use http_base_error::HttpBaseError;

/// HTTP/1.0 协议版本标识。
pub const HTTP_1_0: &str = "HTTP/1.0";

/// HTTP/1.1 协议版本标识。
pub const HTTP_1_1: &str = "HTTP/1.1";
