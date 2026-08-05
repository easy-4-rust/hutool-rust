//! 对齐: `cn.hutool.core.text.finder.Finder`
//! 来源: hutool-core/src/main/java/cn/hutool/core/text/finder/Finder.java

/// 对齐 Java: `Finder#INDEX_NOT_FOUND`
pub const INDEX_NOT_FOUND: i32 = -1;

/// 对齐 Java: `Finder#` 接口
pub trait Finder {
    /// 对齐 Java: `Finder::start#int (int)`
    fn start(&self, from: i32) -> i32;

    /// 对齐 Java: `Finder::end#int (int)`
    fn end(&self, from: i32) -> i32;
}
