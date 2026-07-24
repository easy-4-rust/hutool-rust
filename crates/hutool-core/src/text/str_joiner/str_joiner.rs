//! 对齐: `cn.hutool.core.text.StrJoiner`
//! 来源: hutool-core/src/main/java/cn/hutool/core/text/StrJoiner.java
//!
//! 字符串拼接器(类似 Java `StringJoiner`,支持前缀/后缀/`null` 模式)。

use crate::Result;

use super::null_mode::NullMode;

/// 对齐 Java: `StrJoiner#`
#[derive(Debug, Clone)]
pub struct StrJoiner {
    delimiter: String,
    prefix: String,
    suffix: String,
    wrap_element: bool,
    null_mode: NullMode,
    empty_result: Option<String>,
    /// 已拼接的中间内容（不含外层 prefix/suffix，除非 wrap_element）
    content: String,
    appended: bool,
}

impl Default for StrJoiner {
    fn default() -> Self {
        Self::of(",")
    }
}

impl StrJoiner {
    /// 对齐 Java: `StrJoiner::of(StrJoiner joiner)`
    pub fn of_joiner(other: &StrJoiner) -> Self {
        other.clone()
    }

    /// 对齐 Java: `StrJoiner::of(CharSequence delimiter)`
    pub fn of(delim: &str) -> Self {
        Self::of_wrapped(delim, "", "")
    }

    /// 对齐 Java: `StrJoiner::of(CharSequence delimiter, CharSequence prefix, CharSequence suffix)`
    pub fn of_wrapped(delim: &str, prefix: &str, suffix: &str) -> Self {
        Self {
            delimiter: delim.to_string(),
            prefix: prefix.to_string(),
            suffix: suffix.to_string(),
            wrap_element: false,
            null_mode: NullMode::Ignore,
            empty_result: None,
            content: String::new(),
            appended: false,
        }
    }

    /// 对齐 Java: `StrJoiner(CharSequence delimiter)`
    pub fn new_join(delim: &str) -> Self {
        Self::of(delim)
    }

    /// 对齐 Java: `StrJoiner(Appendable appendable, CharSequence delimiter)`
    pub fn with_appendable(delim: &str) -> Self {
        Self::of(delim)
    }

    /// 对齐 Java: `StrJoiner(CharSequence delimiter, CharSequence prefix, CharSequence suffix)`
    pub fn new_join_wrapped(delim: &str, prefix: &str, suffix: &str) -> Self {
        Self::of_wrapped(delim, prefix, suffix)
    }

    /// 对齐 Java: `StrJoiner::setDelimiter#StrJoiner (CharSequence)`
    pub fn set_delimiter(&mut self, delim: &str) -> Result<&mut Self> {
        self.delimiter = delim.to_string();
        Ok(self)
    }

    /// 对齐 Java: `StrJoiner::setPrefix#StrJoiner (CharSequence)`
    pub fn set_prefix(&mut self, prefix: &str) -> Result<&mut Self> {
        self.prefix = prefix.to_string();
        Ok(self)
    }

    /// 对齐 Java: `StrJoiner::setSuffix#StrJoiner (CharSequence)`
    pub fn set_suffix(&mut self, suffix: &str) -> Result<&mut Self> {
        self.suffix = suffix.to_string();
        Ok(self)
    }

    /// 对齐 Java: `StrJoiner::setWrapElement#StrJoiner (boolean)`
    pub fn set_wrap_element(&mut self, wrap: bool) -> Result<&mut Self> {
        self.wrap_element = wrap;
        Ok(self)
    }

    /// 对齐 Java: `StrJoiner::setNullMode#StrJoiner (NullMode)`
    pub fn set_null_mode(&mut self, mode: NullMode) -> Result<&mut Self> {
        self.null_mode = mode;
        Ok(self)
    }

    /// 对齐 Java: `StrJoiner::setEmptyResult#StrJoiner (String)`
    pub fn set_empty_result(&mut self, empty: &str) -> Result<&mut Self> {
        self.empty_result = Some(empty.to_string());
        Ok(self)
    }

    fn push_piece(&mut self, piece: &str) {
        if self.wrap_element {
            let wrapped = format!("{}{}{}", self.prefix, piece, self.suffix);
            if self.appended {
                self.content.push_str(&self.delimiter);
            }
            self.content.push_str(&wrapped);
        } else {
            if self.appended {
                self.content.push_str(&self.delimiter);
            }
            self.content.push_str(piece);
        }
        self.appended = true;
    }

    /// 对齐 Java: `StrJoiner::append#StrJoiner (Object)` — `None` 表示 null
    pub fn append_option(&mut self, obj: Option<&str>) -> Result<&mut Self> {
        match obj {
            None => match self.null_mode {
                NullMode::Ignore => Ok(self),
                NullMode::ToEmpty => {
                    self.push_piece("");
                    Ok(self)
                }
                NullMode::NullString => {
                    self.push_piece("null");
                    Ok(self)
                }
            },
            Some(s) => {
                self.push_piece(s);
                Ok(self)
            }
        }
    }

    /// 对齐 Java: `StrJoiner::append#StrJoiner (Object)`
    pub fn append_object(&mut self, obj: &dyn std::fmt::Display) -> Result<&mut Self> {
        self.push_piece(&obj.to_string());
        Ok(self)
    }

    /// 对齐 Java: `StrJoiner::append#StrJoiner (T[] array)`
    pub fn append_array<T: std::fmt::Display>(&mut self, arr: &[T]) -> Result<&mut Self> {
        for item in arr {
            self.append_object(item)?;
        }
        Ok(self)
    }

    /// 对齐 Java: `StrJoiner::append#StrJoiner (Iterator<T>)`
    pub fn append_iter<I, T>(&mut self, iter: I) -> Result<&mut Self>
    where
        I: IntoIterator<Item = T>,
        T: std::fmt::Display,
    {
        for item in iter {
            self.append_object(&item)?;
        }
        Ok(self)
    }

    /// 对齐 Java: `StrJoiner::append#StrJoiner (CharSequence csq)`
    pub fn append_str(&mut self, cs: &str) -> Result<&mut Self> {
        self.push_piece(cs);
        Ok(self)
    }

    /// 对齐 Java: `StrJoiner::append#StrJoiner (CharSequence csq, int startInclude, int endExclude)`
    pub fn append_str_range(
        &mut self,
        cs: &str,
        start: i32,
        end: i32,
    ) -> Result<&mut Self> {
        let chars: Vec<char> = cs.chars().collect();
        let s = start.max(0) as usize;
        let e = (end.max(0) as usize).min(chars.len());
        let piece: String = if s < e {
            chars[s..e].iter().collect()
        } else {
            String::new()
        };
        self.push_piece(&piece);
        Ok(self)
    }

    /// 对齐 Java: `StrJoiner::append#StrJoiner (char c)`
    pub fn append_char(&mut self, c: char) -> Result<&mut Self> {
        self.push_piece(&c.to_string());
        Ok(self)
    }

    /// 追加可展开的嵌套集合（对齐 joinMultiArrayTest：二维结构展平）。
    pub fn append_nested_strs(&mut self, groups: &[Vec<String>]) -> Result<&mut Self> {
        for g in groups {
            for s in g {
                self.append_str(s)?;
            }
        }
        Ok(self)
    }

    /// 对齐 Java: `StrJoiner::merge#StrJoiner (StrJoiner)`
    pub fn merge(&mut self, other: &StrJoiner) -> Result<&mut Self> {
        // Hutool merge: append other's result without its wrap when empty content is skipped
        let other_mid = other.content.clone();
        if !other_mid.is_empty() {
            if self.appended {
                self.content.push_str(&self.delimiter);
            }
            self.content.push_str(&other_mid);
            self.appended = true;
        }
        Ok(self)
    }

    /// 对齐 Java: `StrJoiner::length#int ()`
    pub fn length(&self) -> Result<i32> {
        Ok(self.to_string()?.chars().count() as i32)
    }

    /// 对齐 Java: `StrJoiner::toString#String ()`
    pub fn to_string(&self) -> Result<String> {
        if !self.appended {
            if let Some(ref empty) = self.empty_result {
                return Ok(empty.clone());
            }
            if self.wrap_element {
                return Ok(String::new());
            }
            return Ok(format!("{}{}", self.prefix, self.suffix));
        }
        if self.wrap_element {
            Ok(self.content.clone())
        } else {
            Ok(format!("{}{}{}", self.prefix, self.content, self.suffix))
        }
    }
}

impl std::fmt::Display for StrJoiner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string().unwrap_or_default())
    }
}
