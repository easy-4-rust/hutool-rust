//! 对齐: `cn.hutool.core.text.csv.CsvReadConfig`

use super::csv_config::CsvConfig;
use indexmap::IndexMap;

/// 对齐 Java: `CsvReadConfig#`
#[derive(Debug, Clone)]
pub struct CsvReadConfig {
    /// 公共配置。
    pub base: CsvConfig,
    /// 起始行号。
    pub begin_line_no: i64,
    /// 结束行号。
    pub end_line_no: i64,
    /// 表头行号。
    pub header_line_no: i64,
    /// 是否跳过空行。
    pub skip_empty_rows: bool,
    /// 字段数不一致时报错。
    pub error_on_different_field_count: bool,
    /// 是否去除字段空白。
    pub trim_field: bool,
    /// 表头别名映射。
    pub header_alias: IndexMap<String, String>,
}

impl Default for CsvReadConfig {
    fn default() -> Self {
        Self::default_config()
    }
}

impl CsvReadConfig {
    /// 对齐 Java: `CsvReadConfig.defaultConfig()`
    pub fn default_config() -> Self {
        Self {
            base: CsvConfig::default(),
            begin_line_no: 0,
            end_line_no: i64::MAX,
            header_line_no: -1,
            skip_empty_rows: true,
            error_on_different_field_count: false,
            trim_field: false,
            header_alias: IndexMap::new(),
        }
    }

    /// 对齐 Java: `setContainsHeader(true)` → headerLineNo = 0
    pub fn set_contains_header(&mut self, contains: bool) -> &mut Self {
        // Java: headerLineNo = containsHeader ? beginLineNo : -1
        self.header_line_no = if contains { self.begin_line_no } else { -1 };
        self
    }

    /// 对齐 Java: `setBeginLineNo`
    pub fn set_begin_line_no(&mut self, n: i64) -> &mut Self {
        self.begin_line_no = n;
        self
    }

    /// 对齐 Java: `setEndLineNo`
    pub fn set_end_line_no(&mut self, n: i64) -> &mut Self {
        self.end_line_no = n;
        self
    }

    /// 对齐 Java: `setTextDelimiter`
    pub fn set_text_delimiter(&mut self, c: char) -> &mut Self {
        self.base.text_delimiter = c;
        self
    }

    /// 对齐 Java: `setFieldSeparator`
    pub fn set_field_separator(&mut self, c: char) -> &mut Self {
        self.base.field_separator = c;
        self
    }

    /// 对齐 Java: `disableComment`
    pub fn disable_comment(&mut self) -> &mut Self {
        self.base.comment_character = None;
        self
    }

    /// 对齐 Java: `addHeaderAlias`
    pub fn add_header_alias(&mut self, header: &str, alias: &str) -> &mut Self {
        self.header_alias.insert(header.to_string(), alias.to_string());
        self
    }

    /// 对齐 Java: `setTrimField`
    pub fn set_trim_field(&mut self, trim: bool) -> &mut Self {
        self.trim_field = trim;
        self
    }

    /// 对齐 Java: `setHeaderLineNo`
    pub fn set_header_line_no(&mut self, n: i64) -> &mut Self {
        self.header_line_no = n;
        self
    }

    /// 对齐 Java: `setSkipEmptyRows`
    pub fn set_skip_empty_rows(&mut self, skip: bool) -> &mut Self {
        self.skip_empty_rows = skip;
        self
    }

    /// 对齐 Java: `setErrorOnDifferentFieldCount`
    pub fn set_error_on_different_field_count(&mut self, error: bool) -> &mut Self {
        self.error_on_different_field_count = error;
        self
    }
}
