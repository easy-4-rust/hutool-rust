//! `cn.hutool.ai.model.*` 子包对齐。
//!
//! Rust 侧仅 `openai` 实现是 active；其他厂商保留 stub 入口（标注 🚫），
//! 以便按 Java 1:1 拆分结构。

pub mod deepseek;
pub mod doubao;
pub mod gemini;
pub mod grok;
pub mod hutool;
pub mod ollama;
pub mod openai;

pub use openai::OpenAiCompatibleProvider;