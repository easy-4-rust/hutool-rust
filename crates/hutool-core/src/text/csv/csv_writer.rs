//! 对齐: `cn.hutool.core.text.csv.CsvWriter`

use super::csv_write_config::CsvWriteConfig;
use crate::Result;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

/// 对齐 Java: `CsvWriter#`
#[derive(Debug)]
pub struct CsvWriter {
    config: CsvWriteConfig,
    path: Option<std::path::PathBuf>,
    buf: String,
    append: bool,
}

impl CsvWriter {
    /// 对齐 Java: `CsvWriter(File, Charset)`
    pub fn for_path(path: impl AsRef<Path>, append: bool) -> Self {
        Self {
            config: CsvWriteConfig::default(),
            path: Some(path.as_ref().to_path_buf()),
            buf: String::new(),
            append,
        }
    }

    /// 内存写入器
    pub fn in_memory() -> Self {
        Self {
            config: CsvWriteConfig::default(),
            path: None,
            buf: String::new(),
            append: false,
        }
    }

    /// 对齐 Java: `setAlwaysDelimitText` via config
    pub fn with_config(mut self, config: CsvWriteConfig) -> Self {
        self.config = config;
        self
    }

    /// 对齐 Java: `setAlwaysDelimitText`
    pub fn set_always_delimit_text(&mut self, always: bool) -> &mut Self {
        self.config.always_delimit_text = always;
        self
    }

    /// 对齐 Java: `setLineDelimiter`
    pub fn set_line_delimiter(&mut self, delim: &str) -> &mut Self {
        self.config.line_delimiter = delim.to_string();
        self
    }

    /// 对齐 Java: `writeComment`
    pub fn write_comment(&mut self, comment: &str) -> Result<&mut Self> {
        let c = self.config.base.comment_character.unwrap_or('#');
        self.buf.push(c);
        self.buf.push_str(comment);
        self.buf.push_str(&self.config.line_delimiter);
        Ok(self)
    }

    /// 对齐 Java: `write(Iterable)` 别名
    pub fn write(&mut self, rows: &[Vec<String>]) -> Result<&mut Self> {
        self.write_rows(rows)
    }

    /// 对齐 Java: `write(String[]...)`
    pub fn write_rows(&mut self, rows: &[Vec<String>]) -> Result<&mut Self> {
        for row in rows {
            self.write_line(row)?;
        }
        Ok(self)
    }

    /// 对齐 Java: `writeHeaderLine`
    pub fn write_header_line(&mut self, fields: &[&str]) -> Result<&mut Self> {
        let owned: Vec<String> = fields.iter().map(|s| (*s).to_string()).collect();
        self.write_line(&owned)
    }

    /// 对齐 Java: `writeLine(String...)`
    pub fn write_line_strs(&mut self, fields: &[&str]) -> Result<&mut Self> {
        let owned: Vec<String> = fields.iter().map(|s| (*s).to_string()).collect();
        self.write_line(&owned)
    }

    /// 写一行
    pub fn write_line(&mut self, fields: &[String]) -> Result<&mut Self> {
        let line = fields
            .iter()
            .map(|f| self.escape_field(f))
            .collect::<Vec<_>>()
            .join(&self.config.base.field_separator.to_string());
        self.buf.push_str(&line);
        self.buf.push_str(&self.config.line_delimiter);
        Ok(self)
    }

    /// 对齐 Java: `writeBeans`（以 map 列表近似）
    pub fn write_maps(
        &mut self,
        rows: &[indexmap::IndexMap<String, String>],
        properties: Option<&[&str]>,
    ) -> Result<&mut Self> {
        if rows.is_empty() {
            return Ok(self);
        }
        let keys: Vec<String> = if let Some(props) = properties {
            props.iter().map(|s| (*s).to_string()).collect()
        } else {
            rows[0].keys().cloned().collect()
        };
        let header: Vec<String> = keys
            .iter()
            .map(|k| {
                self.config
                    .header_alias
                    .get(k)
                    .cloned()
                    .unwrap_or_else(|| k.clone())
            })
            .collect();
        self.write_line(&header)?;
        for row in rows {
            let vals: Vec<String> = keys
                .iter()
                .map(|k| row.get(k).cloned().unwrap_or_default())
                .collect();
            self.write_line(&vals)?;
        }
        Ok(self)
    }

    fn escape_field(&self, field: &str) -> String {
        let delim = self.config.base.text_delimiter;
        let sep = self.config.base.field_separator;
        let need = self.config.always_delimit_text
            || field.contains(sep)
            || field.contains(delim)
            || field.contains('\n')
            || field.contains('\r');
        if !need {
            return field.to_string();
        }
        let mut out = String::new();
        out.push(delim);
        for c in field.chars() {
            if c == delim {
                out.push(delim);
            }
            out.push(c);
        }
        out.push(delim);
        out
    }

    /// 刷新到文件
    pub fn flush(&mut self) -> Result<()> {
        if let Some(ref path) = self.path {
            let mut opts = OpenOptions::new();
            opts.create(true).write(true);
            if self.append {
                opts.append(true);
            } else {
                opts.truncate(true);
            }
            let mut f = opts.open(path)?;
            f.write_all(self.buf.as_bytes())?;
            self.buf.clear();
        }
        Ok(())
    }

    /// 关闭（flush）
    pub fn close(&mut self) -> Result<()> {
        self.flush()
    }

    /// 内存内容
    pub fn into_string(self) -> String {
        self.buf
    }

    /// 当前缓冲
    pub fn as_str(&self) -> &str {
        &self.buf
    }
}
