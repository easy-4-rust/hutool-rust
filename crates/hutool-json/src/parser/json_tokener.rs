use std::io::Read;

use serde_json::Value;

use crate::{JSONArray, JSONConfig, JSONObject, JsonError, Result};

use super::parse_config::ParseConfig;

/// 对齐: `cn.hutool.json.JSONTokener`
/// 中文说明: 有状态的 Unicode 分词器，保留 Hutool 的迁移接口。
///
/// Stateful Unicode tokenizer retaining Hutool's migration surface.
#[derive(Debug, Clone)]
pub struct JSONTokener {
    input: String,
    position: usize,
    previous: Option<char>,
    pub(crate) config: JSONConfig,
}

impl JSONTokener {
    /// 中文说明: 使用默认防御性限制创建分词器。
    /// 对齐 Java 方法: `new JSONTokener(String)`
    pub fn new(input: &str, config: JSONConfig) -> Result<Self> {
        Self::with_limits(input, config, ParseConfig::default())
    }

    /// 中文说明: 使用显式防御性限制创建分词器。
    /// 对齐 Java 方法: `new JSONTokener(Reader)`
    pub fn with_limits(input: &str, config: JSONConfig, limits: ParseConfig) -> Result<Self> {
        limits.validate(input)?;
        Ok(Self {
            input: input.to_owned(),
            position: 0,
            previous: None,
            config,
        })
    }

    /// 中文说明: 从读取器读取有界 UTF-8 字节创建分词器。
    /// 对齐 Java 方法: `new JSONTokener(Reader)`
    pub fn from_reader(
        reader: &mut dyn Read,
        config: JSONConfig,
        limits: ParseConfig,
    ) -> Result<Self> {
        let mut bytes = Vec::new();
        reader
            .take(limits.max_input_bytes as u64 + 1)
            .read_to_end(&mut bytes)?;
        if bytes.len() > limits.max_input_bytes {
            return Err(JsonError::Limit("input bytes"));
        }
        let input = String::from_utf8(bytes)?;
        Self::with_limits(&input, config, limits)
    }

    /// 中文说明: 返回 JSON 容器的配置。
    /// 对齐 Java 方法: `getConfig`
    #[must_use]
    pub const fn config(&self) -> &JSONConfig {
        &self.config
    }

    /// 中文说明: 是否所有输入已被消耗。
    /// 对齐 Java 方法: `end`
    #[must_use]
    pub fn end(&self) -> bool {
        self.position >= self.input.len()
    }

    /// 中文说明: 是否还有下一个字符可用。
    /// 对齐 Java 方法: `more`
    #[must_use]
    pub fn more(&self) -> bool {
        !self.end()
    }

    /// 中文说明: 返回下一个 Unicode 标量值。
    /// 对齐 Java 方法: `next`
    pub fn next_char(&mut self) -> Option<char> {
        let character = self.input[self.position..].chars().next()?;
        self.position += character.len_utf8();
        self.previous = Some(character);
        Some(character)
    }

    /// 中文说明: 回退一个已读取的标量值。
    /// 对齐 Java 方法: `back`
    pub fn back(&mut self) -> Result<()> {
        let previous = self
            .previous
            .take()
            .ok_or_else(|| JsonError::Syntax("cannot step back twice".into()))?;
        self.position = self.position.saturating_sub(previous.len_utf8());
        Ok(())
    }

    /// 中文说明: 要求下一个标量值等于指定字符。
    /// 对齐 Java 方法: `nextExpected`
    pub fn next_expected(&mut self, expected: char) -> Result<char> {
        let Some(actual) = self.next_char() else {
            return Err(JsonError::Syntax(format!("expected {expected}, found end")));
        };
        if actual != expected {
            return Err(JsonError::Syntax(format!(
                "expected {expected}, found {actual}"
            )));
        }
        Ok(actual)
    }

    /// 中文说明: 读取指定数量的 Unicode 标量值。
    /// 对齐 Java 方法: `nextN`
    pub fn next_n(&mut self, count: usize) -> Result<String> {
        let mut output = String::new();
        for _ in 0..count {
            output.push(
                self.next_char()
                    .ok_or_else(|| JsonError::Syntax("unexpected end".into()))?,
            );
        }
        Ok(output)
    }

    /// 中文说明: 跳过空白字符并返回下一个标量值。
    /// 对齐 Java 方法: `nextClean`
    pub fn next_clean(&mut self) -> Option<char> {
        loop {
            let character = self.next_char()?;
            if !character.is_whitespace() {
                return Some(character);
            }
        }
    }

    /// 中文说明: 读取带引号的 JSON 字符串（起始引号已被消耗）。
    /// 对齐 Java 方法: `nextString`
    pub fn next_string(&mut self, quote: char) -> Result<String> {
        let mut output = String::new();
        while let Some(character) = self.next_char() {
            if character == quote {
                return Ok(output);
            }
            if character != '\\' {
                output.push(character);
                continue;
            }
            let escaped = self
                .next_char()
                .ok_or_else(|| JsonError::Syntax("unterminated escape".into()))?;
            match escaped {
                'b' => output.push('\u{8}'),
                'f' => output.push('\u{c}'),
                'n' => output.push('\n'),
                'r' => output.push('\r'),
                't' => output.push('\t'),
                '"' | '\\' | '/' => output.push(escaped),
                _ => return Err(JsonError::Syntax("unsupported escape".into())),
            }
        }
        Err(JsonError::Syntax("unterminated string".into()))
    }

    /// 中文说明: 读取文本直到遇到分隔符（不消耗分隔符）。
    /// 对齐 Java 方法: `nextTo`
    #[must_use]
    pub fn next_to(&mut self, delimiters: &str) -> String {
        let start = self.position;
        while let Some(character) = self.next_char() {
            if delimiters.contains(character) {
                let end = self.position - character.len_utf8();
                self.position = end;
                self.previous = None;
                return self.input[start..end].trim().to_owned();
            }
        }
        self.input[start..].trim().to_owned()
    }

    /// 中文说明: 跳过并包含标记字符串。
    /// 对齐 Java 方法: `skipPast`
    pub fn skip_past(&mut self, marker: &str) -> bool {
        if let Some(offset) = self.input[self.position..].find(marker) {
            self.position += offset + marker.len();
            self.previous = None;
            true
        } else {
            self.position = self.input.len();
            false
        }
    }

    /// 中文说明: 跳转到指定标量值（不消耗它）。
    /// 对齐 Java 方法: `skipTo`
    pub fn skip_to(&mut self, target: char) -> Option<char> {
        let offset = self.input[self.position..].find(target)?;
        self.position += offset;
        self.previous = None;
        Some(target)
    }

    /// 中文说明: 将剩余输入解析为一个完整的 JSON 值。
    /// 对齐 Java 方法: `nextValue`
    pub fn next_value(&mut self) -> Result<Value> {
        let remaining = self.input[self.position..].trim();
        let value = crate::parse(remaining)?;
        self.position = self.input.len();
        self.previous = None;
        Ok(value)
    }

    /// 中文说明: 将剩余输入解析为 JSON 数组。
    /// 对齐 Java 方法: `toArray`
    pub fn to_array(&mut self) -> Result<JSONArray> {
        JSONArray::from_value(self.next_value()?, self.config.clone())
    }
}
