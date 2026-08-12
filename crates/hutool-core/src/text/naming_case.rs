//! 对齐: `cn.hutool.core.text.NamingCase`
//! 来源: hutool-core/src/main/java/cn/hutool/core/text/NamingCase.java
//!
//! 命名规则封装:驼峰、下划线、短横连接等命名风格互转。

use crate::Result;

/// 对齐 Java: `NamingCase#`
#[derive(Debug, Clone, Copy, Default)]
pub struct NamingCase;

impl NamingCase {
    /// 对齐 Java: `NamingCase::toUnderlineCase#String (CharSequence str)`
    pub fn to_underline_case(str: &str) -> Result<String> {
        Self::to_symbol_case(str, '_')
    }

    /// 对齐 Java: `NamingCase::toKebabCase#String (CharSequence str)`
    pub fn to_kebab_case(str: &str) -> Result<String> {
        Self::to_symbol_case(str, '-')
    }

    /// 对齐 Java: `NamingCase::toSymbolCase#String (CharSequence str, char symbol)`
    pub fn to_symbol_case(str: &str, symbol: char) -> Result<String> {
        let chars: Vec<char> = str.chars().collect();
        let length = chars.len();
        let mut sb = String::new();
        for i in 0..length {
            let mut c = chars[i];
            if c.is_uppercase() {
                let pre = if i > 0 { Some(chars[i - 1]) } else { None };
                let next = if i + 1 < length {
                    Some(chars[i + 1])
                } else {
                    None
                };
                if let Some(pre_char) = pre {
                    if symbol == pre_char {
                        if next.is_none() || next.map(|n| n.is_lowercase()).unwrap_or(false) {
                            c = c.to_lowercase().next().unwrap_or(c);
                        }
                    } else if pre_char.is_lowercase() {
                        sb.push(symbol);
                        if next.is_none()
                            || next
                                .map(|n| n.is_lowercase() || n.is_ascii_digit())
                                .unwrap_or(false)
                        {
                            c = c.to_lowercase().next().unwrap_or(c);
                        }
                    } else if next.map(|n| n.is_lowercase()).unwrap_or(false) {
                        sb.push(symbol);
                        c = c.to_lowercase().next().unwrap_or(c);
                    }
                } else if next.is_none() || next.map(|n| n.is_lowercase()).unwrap_or(false) {
                    c = c.to_lowercase().next().unwrap_or(c);
                }
            }
            sb.push(c);
        }
        Ok(sb)
    }

    /// 对齐 Java: `NamingCase::toPascalCase#String (CharSequence name)`
    pub fn to_pascal_case(name: &str) -> Result<String> {
        let camel = Self::to_camel_case(name)?;
        if camel.is_empty() {
            return Ok(camel);
        }
        let mut chars = camel.chars();
        let first = chars.next().unwrap().to_uppercase().to_string();
        Ok(first + chars.as_str())
    }

    /// 对齐 Java: `NamingCase::toCamelCase#String (CharSequence name)`
    pub fn to_camel_case(name: &str) -> Result<String> {
        Self::to_camel_case_symbol(name, '_')
    }

    /// 对齐 Java: `NamingCase::toCamelCase#String (CharSequence name, char symbol)`
    pub fn to_camel_case_symbol(name: &str, symbol: char) -> Result<String> {
        Self::to_camel_case_full(name, symbol, true)
    }

    /// 对齐 Java: `NamingCase::toCamelCase#String (CharSequence name, char symbol, boolean otherCharToLower)`
    pub fn to_camel_case_full(name: &str, symbol: char, other_lower: bool) -> Result<String> {
        if !name.contains(symbol) {
            return Ok(name.to_string());
        }
        let mut sb = String::with_capacity(name.len());
        let mut upper_case = false;
        for c in name.chars() {
            if c == symbol {
                upper_case = true;
            } else if upper_case {
                for u in c.to_uppercase() {
                    sb.push(u);
                }
                upper_case = false;
            } else if other_lower {
                for l in c.to_lowercase() {
                    sb.push(l);
                }
            } else {
                sb.push(c);
            }
        }
        Ok(sb)
    }
}
