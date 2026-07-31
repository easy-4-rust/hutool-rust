//! 对齐: `cn.hutool.http.body`
//! 来源: hutool-http/src/main/java/cn/hutool/http/body/
//! 中文说明: 请求体类型模块，包含字节体、表单体和资源体

mod request_body;
mod bytes_body;
mod form_url_encoded_body;
mod resource_body;

pub use request_body::RequestBody;
pub use bytes_body::BytesBody;
pub use form_url_encoded_body::FormUrlEncodedBody;
pub use resource_body::ResourceBody;
