//! 对齐: `cn.hutool.core.text.csv.CsvParser`
//! 来源: hutool-core CsvParser（FastCSV 风格）

use super::csv_read_config::CsvReadConfig;
use super::csv_row::CsvRow;
use super::csv_tokener::CsvTokener;
use crate::Result;
use indexmap::IndexMap;

const CR: i32 = '\r' as i32;
const LF: i32 = '\n' as i32;

/// 对齐 Java: `CsvParser#`
#[derive(Debug)]
pub struct CsvParser {
    config: CsvReadConfig,
    tokener: CsvTokener,
    pre_char: i32,
    in_quotes: bool,
    current_field: String,
    header: Option<CsvRow>,
    line_no: i64,
    in_quotes_line_count: i64,
    first_line_field_count: i32,
    max_field_count: usize,
    finished: bool,
}

impl CsvParser {
    /// 对齐 Java: `CsvParser(Reader, CsvReadConfig)`
    pub fn new(text: &str, config: Option<CsvReadConfig>) -> Self {
        Self {
            config: config.unwrap_or_default(),
            tokener: CsvTokener::from_str(text),
            pre_char: -1,
            in_quotes: false,
            current_field: String::new(),
            header: None,
            line_no: -1,
            in_quotes_line_count: 0,
            first_line_field_count: -1,
            max_field_count: 0,
            finished: false,
        }
    }

    /// 对齐 Java: `nextRow`
    pub fn next_row(&mut self) -> Result<Option<CsvRow>> {
        while !self.finished {
            let current_fields = self.read_line();
            let field_count = current_fields.len();
            if field_count < 1 {
                break;
            }
            if self.line_no < self.config.begin_line_no {
                continue;
            }
            if self.line_no > self.config.end_line_no {
                break;
            }
            if self.config.skip_empty_rows && field_count == 1 && current_fields[0].is_empty() {
                continue;
            }
            if self.config.error_on_different_field_count {
                if self.first_line_field_count < 0 {
                    self.first_line_field_count = field_count as i32;
                } else if field_count as i32 != self.first_line_field_count {
                    return Err(crate::CoreError::Codec(format!(
                        "Line {} has {} fields, but first line has {} fields",
                        self.line_no, field_count, self.first_line_field_count
                    )));
                }
            }
            if field_count > self.max_field_count {
                self.max_field_count = field_count;
            }
            if self.line_no == self.config.header_line_no && self.header.is_none() {
                self.init_header(current_fields);
                continue;
            }
            let header_map = self.header.as_ref().and_then(|h| h.header_map.clone());
            return Ok(Some(CsvRow::new(self.line_no, header_map, current_fields)));
        }
        Ok(None)
    }

    fn init_header(&mut self, current_fields: Vec<String>) {
        let mut local: IndexMap<String, usize> = IndexMap::new();
        for (i, field) in current_fields.iter().enumerate() {
            let mut field = field.clone();
            if let Some(alias) = self.config.header_alias.get(&field) {
                field = alias.clone();
            }
            if !field.is_empty() && !local.contains_key(&field) {
                local.insert(field, i);
            }
        }
        self.header = Some(CsvRow::new(self.line_no, Some(local), current_fields));
    }

    fn read_line(&mut self) -> Vec<String> {
        if self.in_quotes_line_count > 0 {
            self.line_no += self.in_quotes_line_count;
            self.in_quotes_line_count = 0;
        }
        let mut current_fields = Vec::new();
        let mut pre_char = self.pre_char;
        let mut in_comment = false;
        let field_sep = self.config.base.field_separator as i32;
        let text_delim = self.config.base.text_delimiter as i32;

        loop {
            let c = self.tokener.next();
            if c < 0 {
                if !self.current_field.is_empty() || pre_char == field_sep {
                    if self.in_quotes {
                        self.current_field.push(self.config.base.text_delimiter);
                    }
                    let field = std::mem::take(&mut self.current_field);
                    self.add_field(&mut current_fields, field);
                }
                self.finished = true;
                break;
            }

            if pre_char < 0 || pre_char == CR || pre_char == LF {
                if !self.in_quotes {
                    if let Some(cc) = self.config.base.comment_character {
                        if c == cc as i32 {
                            in_comment = true;
                        }
                    }
                }
            }
            if in_comment {
                if c == CR || c == LF {
                    self.line_no += 1;
                    in_comment = false;
                }
                continue;
            }

            if self.in_quotes {
                if c == text_delim {
                    let next = self.tokener.next();
                    if next != text_delim {
                        self.in_quotes = false;
                        self.tokener.back();
                    }
                } else if Self::is_line_end(c, pre_char) {
                    self.in_quotes_line_count += 1;
                }
                self.current_field
                    .push(char::from_u32(c as u32).unwrap_or('\0'));
            } else if c == field_sep {
                let field = std::mem::take(&mut self.current_field);
                self.add_field(&mut current_fields, field);
            } else if c == text_delim && Self::is_field_begin(pre_char, field_sep) {
                self.in_quotes = true;
                self.current_field
                    .push(char::from_u32(c as u32).unwrap_or('\0'));
            } else if c == CR {
                let field = std::mem::take(&mut self.current_field);
                self.add_field(&mut current_fields, field);
                pre_char = c;
                break;
            } else if c == LF {
                if pre_char != CR {
                    let field = std::mem::take(&mut self.current_field);
                    self.add_field(&mut current_fields, field);
                    pre_char = c;
                    break;
                }
            } else {
                self.current_field
                    .push(char::from_u32(c as u32).unwrap_or('\0'));
            }
            pre_char = c;
        }
        self.pre_char = pre_char;
        self.line_no += 1;
        current_fields
    }

    fn add_field(&self, current_fields: &mut Vec<String>, mut field: String) {
        let delim = self.config.base.text_delimiter;
        while field.ends_with('\n') || field.ends_with('\r') {
            field.pop();
        }
        if field.starts_with(delim) && field.ends_with(delim) && field.chars().count() >= 2 {
            let chars: Vec<char> = field.chars().collect();
            field = chars[1..chars.len() - 1].iter().collect();
        }
        if self.config.trim_field {
            field = field.trim().to_string();
        }
        current_fields.push(field);
    }

    fn is_line_end(c: i32, pre: i32) -> bool {
        (c == CR || c == LF) && pre != CR
    }

    fn is_field_begin(pre: i32, field_sep: i32) -> bool {
        pre == -1 || pre == field_sep || pre == LF || pre == CR
    }

    /// 读取全部行
    pub fn read_all(&mut self) -> Result<Vec<CsvRow>> {
        let mut rows = Vec::new();
        while let Some(row) = self.next_row()? {
            rows.push(row);
        }
        Ok(rows)
    }

    /// header 原始列表
    pub fn header_fields(&self) -> Option<&[String]> {
        self.header.as_ref().map(|h| h.fields.as_slice())
    }
}
