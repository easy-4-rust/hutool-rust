//! Byte / form / resource request bodies aligned with Hutool `cn.hutool.http.body`.

use crate::http_util::HttpUtil;
use indexmap::IndexMap;
use std::fmt;
use std::io::Write;
use std::path::{Path, PathBuf};

use super::request_body::RequestBody;

/// `application/x-www-form-urlencoded` body.
///
/// Java: `cn.hutool.http.body.FormUrlEncodedBody`
#[derive(Debug, Clone)]
pub struct FormUrlEncodedBody {
    encoded: String,
    charset: String,
}

impl FormUrlEncodedBody {
    /// Java: `FormUrlEncodedBody.create(Map, Charset)` / constructor.
    #[must_use]
    pub fn create(form: &IndexMap<String, String>, charset: &str) -> Self {
        Self {
            encoded: HttpUtil::to_params_form(form, true),
            charset: charset.to_string(),
        }
    }

    /// Alias for [`Self::create`].
    #[must_use]
    pub fn new(form: &IndexMap<String, String>, charset: &str) -> Self {
        Self::create(form, charset)
    }

    /// Returns the encoded payload.
    #[must_use]
    pub fn encoded(&self) -> &str {
        &self.encoded
    }

    /// Java: `FormUrlEncodedBody.write(OutputStream out)`
    pub fn write(&self, out: &mut impl Write) -> std::io::Result<()> {
        out.write_all(self.encoded.as_bytes())?;
        out.flush()
    }
}

impl fmt::Display for FormUrlEncodedBody {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Hutool toString returns the encoded body string.
        f.write_str(&self.encoded)
    }
}

impl RequestBody for FormUrlEncodedBody {
    fn write(&self, out: &mut dyn Write) -> std::io::Result<()> {
        out.write_all(self.encoded.as_bytes())?;
        out.flush()
    }
}
