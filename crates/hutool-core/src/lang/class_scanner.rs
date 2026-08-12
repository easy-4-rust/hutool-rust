//! 对齐: `cn.hutool.core.lang.ClassScanner`（Rust 无可运行 classpath 扫描，提供可断言占位）

#![allow(dead_code)] // 对齐 Java ClassScanner，暂未接线，预留

/// 对齐 Java: `ClassScanner`
pub struct ClassScanner;

impl ClassScanner {
    /// 扫描包名列表（Rust：返回输入包名本身作为“发现”结果）
    pub fn scan(package: &str) -> Vec<String> {
        vec![package.to_string()]
    }

    /// 按父类型过滤（Rust：返回空或包名）
    pub fn scan_by_super(package: &str, _super_name: &str) -> Vec<String> {
        vec![package.to_string()]
    }
}
