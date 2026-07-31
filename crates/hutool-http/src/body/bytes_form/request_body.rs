//! 对齐: `cn.hutool.http.body.RequestBody`
//! 来源: hutool-http/src/main/java/cn/hutool/http/body/RequestBody.java
//! 中文说明: 请求体写入器trait，定义将请求体写入输出流的接口

use std::io::Write;

/// Marker trait for request body writers (Hutool `RequestBody`).
///
/// Java: `cn.hutool.http.body.RequestBody`
pub trait RequestBody {
    /// Writes the body bytes to `out`.
    ///
    /// Java: `RequestBody.write(OutputStream out)`
    fn write(&self, out: &mut dyn Write) -> std::io::Result<()>;
}
