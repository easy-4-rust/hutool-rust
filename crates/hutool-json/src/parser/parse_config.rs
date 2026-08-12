use crate::{JsonError, Result};

/// 对齐: `cn.hutool.json.JSONTokener` 中的解析配置
/// 中文说明: 防御性解析器选项，对应 Hutool 的 `ParseConfig`。
///
/// Defensive parser options corresponding to Hutool's `ParseConfig`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParseConfig {
    keep_strings: bool,
    max_nesting_depth: usize,
    pub(crate) max_input_bytes: usize,
}

impl Default for ParseConfig {
    fn default() -> Self {
        Self {
            keep_strings: false,
            max_nesting_depth: 128,
            max_input_bytes: 16 * 1024 * 1024,
        }
    }
}

impl ParseConfig {
    /// 中文说明: 创建生产环境默认配置。
    /// 对齐 Java 方法: `create`
    #[must_use]
    pub fn create() -> Self {
        Self::default()
    }

    /// 中文说明: 是否将 XML 标量值保持为字符串。
    /// 对齐 Java 方法: `isKeepStrings`
    #[must_use]
    pub const fn is_keep_strings(&self) -> bool {
        self.keep_strings
    }

    /// 中文说明: 设置是否将 XML 标量值保持为字符串。
    /// 对齐 Java 方法: `setKeepStrings`
    pub const fn set_keep_strings(&mut self, value: bool) -> &mut Self {
        self.keep_strings = value;
        self
    }

    /// 中文说明: 返回最大嵌套深度限制。
    /// 对齐 Java 方法: `getMaxNestingDepth`
    #[must_use]
    pub const fn max_nesting_depth(&self) -> usize {
        self.max_nesting_depth
    }

    /// 中文说明: 设置最大嵌套深度限制。
    /// 对齐 Java 方法: `setMaxNestingDepth`
    pub const fn set_max_nesting_depth(&mut self, value: usize) -> &mut Self {
        self.max_nesting_depth = value;
        self
    }

    /// 中文说明: 返回最大编码输入大小限制。
    /// 对齐 Java 方法: `getMaxInputBytes`
    #[must_use]
    pub const fn max_input_bytes(&self) -> usize {
        self.max_input_bytes
    }

    /// 中文说明: 设置最大编码输入大小限制。
    /// 对齐 Java 方法: `setMaxInputBytes`
    pub const fn set_max_input_bytes(&mut self, value: usize) -> &mut Self {
        self.max_input_bytes = value;
        self
    }

    pub(crate) fn validate(&self, input: &str) -> Result<()> {
        if input.len() > self.max_input_bytes {
            return Err(JsonError::Limit("input bytes"));
        }
        let mut depth = 0_usize;
        let mut quoted = false;
        let mut escaped = false;
        for character in input.chars() {
            if quoted {
                if escaped {
                    escaped = false;
                } else if character == '\\' {
                    escaped = true;
                } else if character == '"' {
                    quoted = false;
                }
                continue;
            }
            match character {
                '"' => quoted = true,
                '{' | '[' => {
                    depth = depth.saturating_add(1);
                    if depth > self.max_nesting_depth {
                        return Err(JsonError::Limit("nesting depth"));
                    }
                }
                '}' | ']' => depth = depth.saturating_sub(1),
                _ => {}
            }
        }
        Ok(())
    }
}
