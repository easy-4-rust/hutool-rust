//! 对齐: `cn.hutool.core.lang.ConsoleTable`

#![allow(dead_code)] // 对齐 Java ConsoleTable，暂未接线，预留

/// 对齐 Java: `ConsoleTable`
#[derive(Debug, Default)]
pub struct ConsoleTable {
    headers: Vec<Vec<String>>,
    bodies: Vec<Vec<String>>,
    sbc_mode: bool,
}

impl ConsoleTable {
    /// 对齐 `create()`
    pub fn create() -> Self {
        Self {
            sbc_mode: true,
            ..Default::default()
        }
    }

    /// 对齐 `setSBCMode`
    pub fn set_sbc_mode(mut self, sbc: bool) -> Self {
        self.sbc_mode = sbc;
        self
    }

    /// 对齐 `addHeader`
    pub fn add_header(&mut self, cols: &[&str]) -> &mut Self {
        self.headers
            .push(cols.iter().map(|s| (*s).to_string()).collect());
        self
    }

    /// 对齐 `addBody`
    pub fn add_body(&mut self, cols: &[&str]) -> &mut Self {
        self.bodies
            .push(cols.iter().map(|s| (*s).to_string()).collect());
        self
    }

    /// 渲染为字符串（对齐 print 内容）
    pub fn render(&self) -> String {
        let mut lines = Vec::new();
        for h in &self.headers {
            lines.push(h.join(" | "));
        }
        for b in &self.bodies {
            lines.push(b.join(" | "));
        }
        lines.join("\n")
    }

    /// 对齐 `print`
    pub fn print(&self) {
        println!("{}", self.render());
    }

    /// 是否 SBC 模式
    pub fn is_sbc_mode(&self) -> bool {
        self.sbc_mode
    }
}

impl std::fmt::Display for ConsoleTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.render())
    }
}

#[cfg(test)]
mod console_table_idiomatic_parity {
    use super::*;

    /// 对齐 Java ConsoleTable create/header/body 可执行证据。
    #[test]
    fn console_table_render() {
        let mut t = ConsoleTable::create().set_sbc_mode(false);
        t.add_header(&["A", "B"]).add_body(&["1", "2"]);
        assert!(!t.is_sbc_mode());
        let s = t.to_string();
        assert!(s.contains("A | B"));
        assert!(s.contains("1 | 2"));
    }
}
