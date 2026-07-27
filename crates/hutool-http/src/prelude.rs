//! Re-exported convenience types for `hutool-http`.
//!
//! Usage:
//! ```rust
//! use hutool_http::prelude::*;
//! ```

pub use crate::{
    HttpClient, HttpClientBuilder, HttpConfig, HttpError, HttpRequest, HttpResponse, Method,
    RetryPolicy, StatusCode,
};
