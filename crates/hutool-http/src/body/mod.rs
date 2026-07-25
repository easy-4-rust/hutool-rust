//! 对齐: `cn.hutool.http.body`
//! 来源: hutool-http/src/main/java/cn/hutool/http/body/
//! 中文说明: HTTP请求体模块，包含字节体、表单体、资源体和multipart体

mod bytes_form;
mod multipart;
mod multipart_stream;

pub use bytes_form::{BytesBody, FormUrlEncodedBody, RequestBody, ResourceBody};
pub use multipart::MultipartBody;
pub use multipart_stream::MultipartOutputStream;
