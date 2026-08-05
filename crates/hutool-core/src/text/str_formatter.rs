//! 对齐: `cn.hutool.core.text.StrFormatter`

use crate::Result;

/// 对齐 Java: `StrFormatter`
#[derive(Debug, Clone, Copy, Default)]
pub struct StrFormatter;

impl StrFormatter {
    /// 对齐 `format(pattern, args...)`，占位符 `{}`，支持 `\{}` 与 `\\`
    pub fn format(pattern: &str, args: &[&str]) -> Result<String> {
        Ok(Self::format_with(pattern, "{}", args)?)
    }

    /// 对齐 `formatWith(pattern, placeholder, args...)`
    pub fn format_with(pattern: &str, placeholder: &str, args: &[&str]) -> Result<String> {
        let mut out = String::with_capacity(pattern.len() + 16);
        let mut arg_i = 0usize;
        let pb = placeholder.as_bytes();
        let bytes = pattern.as_bytes();
        let mut i = 0usize;
        while i < bytes.len() {
            if bytes[i] == b'\\' && i + 1 < bytes.len() {
                // \\ → \ ; \PLACEHOLDER → PLACEHOLDER literal
                if bytes[i + 1] == b'\\' {
                    out.push('\\');
                    i += 2;
                    continue;
                }
                if pattern[i + 1..].starts_with(placeholder) {
                    out.push_str(placeholder);
                    i += 1 + placeholder.len();
                    continue;
                }
            }
            if pattern[i..].starts_with(placeholder) {
                if arg_i < args.len() {
                    out.push_str(args[arg_i]);
                    arg_i += 1;
                }
                i += placeholder.len();
                continue;
            }
            // safe char push
            let ch = pattern[i..].chars().next().unwrap();
            out.push(ch);
            i += ch.len_utf8();
        }
        let _ = pb;
        Ok(out)
    }

    /// map 格式化（简易）
    pub fn format_map(
        template: &str,
        entries: &[(&str, &str)],
        _ignore_null: bool,
    ) -> Result<String> {
        let mut s = template.to_string();
        for (k, v) in entries {
            s = s.replace(&format!("{{{k}}}"), v);
        }
        Ok(s)
    }
}
