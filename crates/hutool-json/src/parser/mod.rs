use std::io::Read;

use serde_json::Value;

use crate::{JSONArray, JSONConfig, JSONObject, JsonError, Result};

mod parse_config;
mod json_tokener;
mod json_parser;

pub use parse_config::ParseConfig;
pub use json_tokener::JSONTokener;
pub use json_parser::JSONParser;
