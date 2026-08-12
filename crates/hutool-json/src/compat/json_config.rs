//! 对齐: `cn.hutool.json.JSONConfig`
//! 来源: `/Users/wandl/workspaces/workspace-github/hutool/hutool-json/src/main/java/cn/hutool/json/JSONConfig.java`
//! 中文说明: 提供 Hutool 风格的 JSON 配置对象，控制解析与序列化行为。

/// Hutool 兼容的 JSON 配置。
///
/// 对齐 Java 类: `cn.hutool.json.JSONConfig`
/// 来源: `cn.hutool.json.JSONConfig`
///
/// 控制 JSON 解析和序列化行为的配置选项。
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(clippy::struct_excessive_bools)]
pub struct JSONConfig {
    ignore_error: bool,
    ignore_case: bool,
    date_format: Option<String>,
    ignore_null_value: bool,
    transient_support: bool,
    strip_trailing_zeros: bool,
    check_duplicate: bool,
    write_long_as_string: bool,
    natural_key_order: bool,
}

impl Default for JSONConfig {
    fn default() -> Self {
        Self {
            ignore_error: false,
            ignore_case: false,
            date_format: None,
            ignore_null_value: false,
            transient_support: true,
            strip_trailing_zeros: true,
            check_duplicate: false,
            write_long_as_string: false,
            natural_key_order: false,
        }
    }
}

impl JSONConfig {
    /// 中文说明: 创建 Hutool 兼容的默认配置。
    /// 对齐 Java 方法: `JSONConfig.create`
    #[must_use]
    pub fn create() -> Self {
        Self::default()
    }

    /// 中文说明: JSON 对象在 Rust 中是否有序（始终返回 true）。
    /// 对齐 Java 方法: `isOrder`
    #[must_use]
    pub const fn is_order(&self) -> bool {
        true
    }

    /// 中文说明: 保留已废弃的 Hutool 选项，实际为空操作。
    /// 对齐 Java 方法: `setOrder`
    pub const fn set_order(&mut self, _order: bool) -> &mut Self {
        self
    }

    /// 中文说明: 设置使用字典序键排序方式序列化对象。
    /// 对齐 Java 方法: `setNatureKeyComparator`
    pub const fn set_nature_key_comparator(&mut self) -> &mut Self {
        self.natural_key_order = true;
        self
    }

    /// 中文说明: 是否使用字典序键排序。
    /// 对齐 Java 方法: `hasNatureKeyComparator`
    #[must_use]
    pub const fn has_nature_key_comparator(&self) -> bool {
        self.natural_key_order
    }

    /// 中文说明: 是否忽略转换失败。
    /// 对齐 Java 方法: `isIgnoreError`
    #[must_use]
    pub const fn is_ignore_error(&self) -> bool {
        self.ignore_error
    }

    /// 中文说明: 设置是否忽略转换错误。
    /// 对齐 Java 方法: `setIgnoreError`
    pub const fn set_ignore_error(&mut self, value: bool) -> &mut Self {
        self.ignore_error = value;
        self
    }

    /// 中文说明: 是否忽略键的 ASCII 大小写。
    /// 对齐 Java 方法: `isIgnoreCase`
    #[must_use]
    pub const fn is_ignore_case(&self) -> bool {
        self.ignore_case
    }

    /// 中文说明: 设置是否忽略键的 ASCII 大小写。
    /// 对齐 Java 方法: `setIgnoreCase`
    pub const fn set_ignore_case(&mut self, value: bool) -> &mut Self {
        self.ignore_case = value;
        self
    }

    /// 中文说明: 返回配置的日期格式。
    /// 对齐 Java 方法: `getDateFormat`
    #[must_use]
    pub fn date_format(&self) -> Option<&str> {
        self.date_format.as_deref()
    }

    /// 中文说明: 设置日期格式，空值恢复时间戳模式。
    /// 对齐 Java 方法: `setDateFormat`
    pub fn set_date_format(&mut self, value: &str) -> &mut Self {
        self.date_format = (!value.is_empty()).then(|| value.to_owned());
        self
    }

    /// 中文说明: 是否省略 null 值的字段和数组元素。
    /// 对齐 Java 方法: `isIgnoreNullValue`
    #[must_use]
    pub const fn is_ignore_null_value(&self) -> bool {
        self.ignore_null_value
    }

    /// 中文说明: 设置是否省略 null 值。
    /// 对齐 Java 方法: `setIgnoreNullValue`
    pub const fn set_ignore_null_value(&mut self, value: bool) -> &mut Self {
        self.ignore_null_value = value;
        self
    }

    /// 中文说明: 是否忽略 Java 风格的 transient 字段。
    /// 对齐 Java 方法: `isTransientSupport`
    #[must_use]
    pub const fn is_transient_support(&self) -> bool {
        self.transient_support
    }

    /// 中文说明: 设置是否支持 Java transient 字段兼容。
    /// 对齐 Java 方法: `setTransientSupport`
    pub const fn set_transient_support(&mut self, value: bool) -> &mut Self {
        self.transient_support = value;
        self
    }

    /// 中文说明: 是否去除小数尾部的零。
    /// 对齐 Java 方法: `isStripTrailingZeros`
    #[must_use]
    pub const fn is_strip_trailing_zeros(&self) -> bool {
        self.strip_trailing_zeros
    }

    /// 中文说明: 设置是否去除小数尾部的零。
    /// 对齐 Java 方法: `setStripTrailingZeros`
    pub const fn set_strip_trailing_zeros(&mut self, value: bool) -> &mut Self {
        self.strip_trailing_zeros = value;
        self
    }

    /// 中文说明: 是否检查重复的键。
    /// 对齐 Java 方法: `isCheckDuplicate`
    #[must_use]
    pub const fn is_check_duplicate(&self) -> bool {
        self.check_duplicate
    }

    /// 中文说明: 设置是否检查重复键。
    /// 对齐 Java 方法: `setCheckDuplicate`
    pub const fn set_check_duplicate(&mut self, value: bool) -> &mut Self {
        self.check_duplicate = value;
        self
    }

    /// 中文说明: 是否将 64 位整数序列化为字符串（JavaScript 安全）。
    /// 对齐 Java 方法: `isWriteLongAsString`
    #[must_use]
    pub const fn is_write_long_as_string(&self) -> bool {
        self.write_long_as_string
    }

    /// 中文说明: 设置是否将 64 位整数序列化为字符串。
    /// 对齐 Java 方法: `setWriteLongAsString`
    pub const fn set_write_long_as_string(&mut self, value: bool) -> &mut Self {
        self.write_long_as_string = value;
        self
    }
}
