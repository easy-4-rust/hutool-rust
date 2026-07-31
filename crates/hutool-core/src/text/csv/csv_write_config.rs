//! 对齐: `cn.hutool.core.text.csv.CsvWriteConfig`
//! 来源: hutool-core/src/main/java/cn/hutool/core/text/csv/CsvWriteConfig.java

use super::csv_config::CsvConfig;
use indexmap::IndexMap;

/// 对齐 Java: `CsvWriteConfig#`
#[derive(Debug, Clone)]
pub struct CsvWriteConfig {
    /// 公共配置。
    pub base: CsvConfig,
    /// 是否始终用引号包裹文本。
    pub always_delimit_text: bool,
    /// 行分隔符。
    pub line_delimiter: String,
    /// 是否以换行结尾。
    pub ending_line_break: bool,
    /// 表头别名映射。
    pub header_alias: IndexMap<String, String>,
}

impl Default for CsvWriteConfig {
    fn default() -> Self {
        Self::default_config()
    }
}

impl CsvWriteConfig {
    /// 对齐 Java: `CsvWriteConfig.defaultConfig()`
    pub fn default_config() -> Self {
        Self {
            base: CsvConfig::default(),
            always_delimit_text: false,
            line_delimiter: "\r\n".to_string(),
            ending_line_break: true,
            header_alias: IndexMap::new(),
        }
    }

    /// 对齐 Java: `setAlwaysDelimitText`
    pub fn set_always_delimit_text(&mut self, v: bool) -> &mut Self {
        self.always_delimit_text = v;
        self
    }

    /// 对齐 Java: `setLineDelimiter`
    pub fn set_line_delimiter(&mut self, delim: &str) -> &mut Self {
        self.line_delimiter = delim.to_string();
        self
    }

    /// 对齐 Java: `setEndingLineBreak`
    pub fn set_ending_line_break(&mut self, ending: bool) -> &mut Self {
        self.ending_line_break = ending;
        self
    }

    /// 对齐 Java: `addHeaderAlias`
    pub fn add_header_alias(&mut self, header: &str, alias: &str) -> &mut Self {
        self.header_alias
            .insert(header.to_string(), alias.to_string());
        self.base.add_header_alias(header, alias);
        self
    }
}
