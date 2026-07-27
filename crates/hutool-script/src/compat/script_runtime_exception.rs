//! `ScriptRuntimeException` 兼容对象。
//!
//! 对应 Java 类：`cn.hutool.script.ScriptRuntimeException`
//! Java 来源：`hutool-script/src/main/java/cn/hutool/script/ScriptRuntimeException.java`

use crate::ScriptError;
use std::fmt;

/// 位置感知的脚本运行时异常，对齐 Hutool 的运行时异常语义。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScriptRuntimeException {
    message: String,
    file_name: Option<String>,
    line_number: Option<usize>,
    column_number: Option<usize>,
}

impl ScriptRuntimeException {
    /// 创建仅含错误消息的异常。
    #[must_use]
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            file_name: None,
            line_number: None,
            column_number: None,
        }
    }

    /// 创建带源码位置信息的异常。
    #[must_use]
    pub fn with_location(
        message: impl Into<String>,
        file_name: impl Into<String>,
        line_number: usize,
        column_number: Option<usize>,
    ) -> Self {
        Self {
            message: message.into(),
            file_name: Some(file_name.into()),
            line_number: Some(line_number),
            column_number,
        }
    }

    /// 顺序替换 `{}` 占位符，兼容 Hutool 的格式化构造方式。
    #[must_use]
    pub fn formatted(template: &str, parameters: &[&dyn fmt::Display]) -> Self {
        let mut message = template.to_owned();
        for parameter in parameters {
            message = message.replacen("{}", &parameter.to_string(), 1);
        }
        Self::new(message)
    }

    pub(crate) fn from_error(error: &ScriptError, file_name: Option<&str>) -> Self {
        let (line_number, column_number) = match error {
            ScriptError::Evaluation(error) => position(error.position()),
            ScriptError::Compilation(error) => position(error.position()),
            _ => (None, None),
        };
        Self {
            message: error.to_string(),
            file_name: file_name.map(str::to_owned),
            line_number,
            column_number,
        }
    }

    /// 返回不含位置信息的基础消息。
    #[must_use]
    pub fn base_message(&self) -> &str {
        &self.message
    }

    /// 返回 1-based 行号。
    #[must_use]
    pub const fn line_number(&self) -> Option<usize> {
        self.line_number
    }

    /// 返回 1-based 列号。
    #[must_use]
    pub const fn column_number(&self) -> Option<usize> {
        self.column_number
    }

    /// 返回源文件名。
    #[must_use]
    pub fn file_name(&self) -> Option<&str> {
        self.file_name.as_deref()
    }
}

fn position(position: rhai::Position) -> (Option<usize>, Option<usize>) {
    (position.line(), position.position())
}

impl fmt::Display for ScriptRuntimeException {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut rendered = self.message.clone();
        if let Some(file_name) = &self.file_name {
            rendered.push_str(" in ");
            rendered.push_str(file_name);
            if let Some(line) = self.line_number {
                rendered.push_str(" at line number ");
                rendered.push_str(&line.to_string());
            }
            if let Some(column) = self.column_number {
                rendered.push_str(" at column number ");
                rendered.push_str(&column.to_string());
            }
        }
        formatter.write_str(&rendered)
    }
}

impl std::error::Error for ScriptRuntimeException {}
