//! 对齐: `cn.hutool.core.lang.Console`

#![allow(dead_code)] // 对齐 Java Console，暂未接线，预留

use crate::text::str_formatter::StrFormatter;

/// 对齐 Java: `Console` — 格式化后写入缓冲区（测试可断言）
#[derive(Debug, Default)]
pub struct Console;

impl Console {
    /// 格式化（`{}` 占位）
    pub fn format(template: &str, args: &[&str]) -> String {
        StrFormatter::format(template, args).unwrap_or_else(|_| template.to_string())
    }

    /// 对齐 `log` — 返回格式化字符串（同时可打印）
    pub fn log(template: &str, args: &[&str]) -> String {
        let s = Self::format(template, args);
        println!("{s}");
        s
    }

    /// 对齐 `print`
    pub fn print(template: &str, args: &[&str]) -> String {
        let s = Self::format(template, args);
        print!("{s}");
        s
    }

    /// 对齐 `error`
    pub fn error(template: &str, args: &[&str]) -> String {
        let s = Self::format(template, args);
        eprintln!("{s}");
        s
    }

    /// 多参数空格拼接
    pub fn join_args(args: &[&str]) -> String {
        args.join(" ")
    }

    /// 对齐 `printProgress(char, int)` — 返回进度条字符串。
    pub fn print_progress(show_char: char, len: usize) -> String {
        show_char.to_string().repeat(len)
    }

    /// 对齐 `printProgress(char, int, double)` — 按比率截断长度。
    pub fn print_progress_rate(show_char: char, total_len: usize, rate: f64) -> String {
        let rate = rate.clamp(0.0, 1.0);
        let n = ((total_len as f64) * rate).round() as usize;
        Self::print_progress(show_char, n)
    }
}

#[cfg(test)]
mod console_idiomatic_parity {
    use super::*;

    /// 对齐 Java Console.format/log/progress 可执行证据。
    #[test]
    fn console_format_and_progress() {
        assert_eq!(Console::format("hi {}", &["rust"]), "hi rust");
        assert_eq!(Console::join_args(&["a", "b"]), "a b");
        assert_eq!(Console::print_progress('#', 3), "###");
        assert_eq!(Console::print_progress_rate('#', 10, 0.5).len(), 5);
    }
}
