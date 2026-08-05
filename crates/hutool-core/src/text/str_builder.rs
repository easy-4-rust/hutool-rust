//! 对齐: `cn.hutool.core.text.StrBuilder`
//! 来源: hutool-core/src/main/java/cn/hutool/core/text/StrBuilder.java
//!
//! 可变字符串构建器,等价于 Java `StringBuilder`（含越界 insert 空格填充）。

use crate::{CoreError, Result};

/// 对齐 Java: `StrBuilder#DEFAULT_CAPACITY`
pub const DEFAULT_CAPACITY: i32 = 16;

/// 对齐 Java: `StrBuilder#`
#[derive(Debug, Clone)]
pub struct StrBuilder {
    buf: Vec<char>,
}

impl Default for StrBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl StrBuilder {
    /// 对齐 Java: `StrBuilder::create()`
    pub fn create() -> Self {
        Self::with_capacity(DEFAULT_CAPACITY)
    }

    /// 对齐 Java: `StrBuilder::create(int initialCapacity)`
    pub fn with_capacity(cap: i32) -> Self {
        Self {
            buf: Vec::with_capacity(cap.max(0) as usize),
        }
    }

    /// 对齐 Java: `StrBuilder::create(CharSequence... strs)`
    pub fn from_strs(strs: &[&str]) -> Self {
        let mut s = Self::create();
        for t in strs {
            let _ = s.append_str(t);
        }
        s
    }

    /// 对齐 Java: `StrBuilder()`
    pub fn new() -> Self {
        Self::create()
    }

    /// 对齐 Java: `StrBuilder(int initialCapacity)`
    pub fn with_capacity_ctor(cap: i32) -> Self {
        Self::with_capacity(cap)
    }

    /// 对齐 Java: `StrBuilder(CharSequence... strs)`
    pub fn from_strs_ctor(strs: &[&str]) -> Self {
        Self::from_strs(strs)
    }

    /// 对齐 Java: `StrBuilder::append#StrBuilder (Object obj)`
    pub fn append_object(&mut self, obj: &dyn std::fmt::Display) -> Result<&mut Self> {
        self.append_str(&obj.to_string())
    }

    /// 对齐 Java: `StrBuilder::append#StrBuilder (char c)`
    pub fn append_char(&mut self, c: char) -> Result<&mut Self> {
        self.buf.push(c);
        Ok(self)
    }

    /// 对齐 Java: `StrBuilder::append#StrBuilder (char[] src)`
    pub fn append_chars(&mut self, src: &[char]) -> Result<&mut Self> {
        self.buf.extend_from_slice(src);
        Ok(self)
    }

    /// 对齐 Java: `StrBuilder::append#StrBuilder (char[] src, int srcPos, int length)`
    pub fn append_chars_range(&mut self, src: &[char], pos: i32, len: i32) -> Result<&mut Self> {
        let start = pos.max(0) as usize;
        let end = (start + len.max(0) as usize).min(src.len());
        self.buf.extend_from_slice(&src[start..end]);
        Ok(self)
    }

    /// 对齐 Java: `StrBuilder::append#StrBuilder (CharSequence csq)`
    pub fn append_str(&mut self, cs: &str) -> Result<&mut Self> {
        self.buf.extend(cs.chars());
        Ok(self)
    }

    /// 对齐 Java: `StrBuilder::append#StrBuilder (CharSequence csq, int start, int end)`
    pub fn append_str_range(&mut self, cs: &str, start: i32, end: i32) -> Result<&mut Self> {
        let chars: Vec<char> = cs.chars().collect();
        let s = start.max(0) as usize;
        let e = (end.max(0) as usize).min(chars.len());
        if s < e {
            self.buf.extend_from_slice(&chars[s..e]);
        }
        Ok(self)
    }

    /// 对齐 Java: `StrBuilder::insert#StrBuilder (int index, Object obj)`
    pub fn insert_object(&mut self, index: i32, obj: &dyn std::fmt::Display) -> Result<&mut Self> {
        self.insert_str(index, &obj.to_string())
    }

    /// 对齐 Java: `StrBuilder::insert#StrBuilder (int index, char c)`
    pub fn insert_char(&mut self, index: i32, c: char) -> Result<&mut Self> {
        let s = [c];
        self.insert_chars_at(index, &s)
    }

    /// 对齐 Java: `StrBuilder::insert#StrBuilder (int index, CharSequence csq)`
    pub fn insert_str(&mut self, index: i32, cs: &str) -> Result<&mut Self> {
        let chars: Vec<char> = cs.chars().collect();
        self.insert_chars_at(index, &chars)
    }

    /// 对齐 Java: `StrBuilder::insert#StrBuilder (int index, CharSequence csq, int start, int end)`
    pub fn insert_str_range(
        &mut self,
        index: i32,
        cs: &str,
        start: i32,
        end: i32,
    ) -> Result<&mut Self> {
        let chars: Vec<char> = cs.chars().collect();
        let s = start.max(0) as usize;
        let e = (end.max(0) as usize).min(chars.len());
        if s < e {
            self.insert_chars_at(index, &chars[s..e])?;
        }
        Ok(self)
    }

    fn insert_chars_at(&mut self, index: i32, chars: &[char]) -> Result<&mut Self> {
        let idx = index.max(0) as usize;
        if idx > self.buf.len() {
            // Hutool: pad with spaces when inserting past end
            self.buf.resize(idx, ' ');
        }
        self.buf.splice(idx..idx, chars.iter().copied());
        Ok(self)
    }

    /// 对齐 Java: `StrBuilder::hasContent#boolean ()`
    pub fn has_content(&self) -> Result<bool> {
        Ok(!self.buf.is_empty())
    }

    /// 对齐 Java: `StrBuilder::isEmpty#boolean ()`
    pub fn is_empty(&self) -> Result<bool> {
        Ok(self.buf.is_empty())
    }

    /// 对齐 Java: `StrBuilder::clear#StrBuilder ()`
    pub fn clear(&mut self) -> Result<&mut Self> {
        self.buf.clear();
        Ok(self)
    }

    /// 对齐 Java: `StrBuilder::reset#StrBuilder ()`
    pub fn reset(&mut self) -> Result<&mut Self> {
        self.clear()
    }

    /// 对齐 Java: `StrBuilder::delTo#StrBuilder (int newPosition)`
    pub fn del_to(&mut self, pos: i32) -> Result<&mut Self> {
        // Java: delTo(newPosition) => del(newPosition, this.position)
        let p = pos.max(0);
        let end = self.buf.len() as i32;
        self.del(p, end)
    }

    /// 对齐 Java: `StrBuilder::del#StrBuilder (int start, int end)`
    pub fn del(&mut self, start: i32, end: i32) -> Result<&mut Self> {
        let len = self.buf.len();
        let mut s = start.max(0) as usize;
        let mut e = end.max(0) as usize;
        if s > len {
            s = len;
        }
        if e > len {
            e = len;
        }
        if s < e {
            self.buf.drain(s..e);
        }
        Ok(self)
    }

    /// 对齐 Java: `StrBuilder::toString#String (boolean isReset)`
    pub fn to_string_reset(&mut self, reset: bool) -> Result<String> {
        let s = self.buf.iter().collect::<String>();
        if reset {
            self.buf.clear();
        }
        Ok(s)
    }

    /// 对齐 Java: `StrBuilder::toStringAndReset#String ()`
    pub fn to_string_and_reset(&mut self) -> Result<String> {
        self.to_string_reset(true)
    }

    /// 对齐 Java: `StrBuilder::toString#String ()`
    pub fn to_string(&self) -> Result<String> {
        Ok(self.buf.iter().collect())
    }

    /// 对齐 Java: `StrBuilder::length#int ()`
    pub fn length(&self) -> Result<i32> {
        Ok(self.buf.len() as i32)
    }

    /// 对齐 Java: `StrBuilder::charAt#char (int index)`
    pub fn char_at(&self, index: i32) -> Result<char> {
        let len = self.buf.len() as i32;
        let idx = if index < 0 { len + index } else { index };
        if idx < 0 || idx >= len {
            return Err(CoreError::InvalidArgument {
                name: "index",
                reason: "StringIndexOutOfBoundsException",
            });
        }
        Ok(self.buf[idx as usize])
    }

    /// 对齐 Java: `StrBuilder::subSequence#CharSequence (int start, int end)`
    pub fn sub_sequence(&self, start: i32, end: i32) -> Result<String> {
        self.sub_string_range(start, end)
    }

    /// 对齐 Java: `StrBuilder::subString#String (int start)`
    pub fn sub_string(&self, start: i32) -> Result<String> {
        self.sub_string_range(start, self.buf.len() as i32)
    }

    /// 对齐 Java: `StrBuilder::subString#String (int start, int end)`
    pub fn sub_string_range(&self, start: i32, end: i32) -> Result<String> {
        let s = start.max(0) as usize;
        let e = (end.max(0) as usize).min(self.buf.len());
        if s >= e {
            return Ok(String::new());
        }
        Ok(self.buf[s..e].iter().collect())
    }

    /// 对齐 Java: `StrBuilder::getChars#void (int srcBegin, int srcEnd, char[] dst, int dstBegin)`
    pub fn get_chars(&self, begin: i32, end: i32, dst: &mut [char], dst_begin: i32) -> Result<()> {
        let s = begin.max(0) as usize;
        let e = (end.max(0) as usize).min(self.buf.len());
        let d = dst_begin.max(0) as usize;
        let slice = &self.buf[s..e];
        dst[d..d + slice.len()].copy_from_slice(slice);
        Ok(())
    }
}

impl std::fmt::Display for StrBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.buf.iter().collect::<String>())
    }
}
