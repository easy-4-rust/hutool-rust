//! `hutool-poi` — 对齐 Hutool `cn.hutool.poi.*` 的 Office 文档处理模块。
//!
//! ## 当前状态
//!
//! ⚪ **未实现的占位骨架**：本 crate 当前有 79 个 `.rs` 文件用于登记 Java POI API 形状，
//! 其中 67 个文件包含 `unimplemented!()`。它不由 `hutool` facade 暴露，也没有可用的文档引擎。
//! 以下项目仅是未来可能的引擎来源，不是当前依赖：
//!
//! - `easyexcel-rs`：Excel 读写（对应 `cn.hutool.poi.excel.*`）
//! - `easydoc-rs`：Word 读写（对应 `cn.hutool.poi.word.*`）
//! - `easyofd-rs`：OFD 写入（对应 `cn.hutool.poi.ofd.*`）
//! - `easypdf-rs`：PDF 处理（对应 `cn.hutool.poi.pdf.*`）
//!
//! ## 模块结构
//!
//! ```text
//! hutool-poi/src/
//! ├── lib.rs                    ← 本文件（目录索引）
//! ├── excel/                    ← cn.hutool.poi.excel
//! │   ├── excel_util.rs         ← ExcelUtil（静态门面）
//! │   ├── excel_reader.rs       ← ExcelReader
//! │   ├── excel_writer.rs       ← ExcelWriter
//! │   ├── big_excel_writer.rs   ← BigExcelWriter（SXSSF）
//! │   ├── cell/                 ← Cell 相关接口和值类型
//! │   │   ├── setters/          ← CellSetter 实现
//! │   │   └── values/           ← CellValue 实现
//! │   ├── editors/              ← CellEditor 实现
//! │   ├── reader/               ← SheetReader 实现
//! │   ├── sax/                  ← SAX 流式读取
//! │   │   └── handler/          ← RowHandler 实现
//! │   └── style/                ← 样式工具
//! ├── exceptions/               ← POIException
//! ├── ofd/                      ← OfdWriter
//! └── word/                     ← Word07Writer / WordUtil
//! ```

#![forbid(unsafe_code)]

pub mod excel;
pub mod exceptions;
pub mod ofd;
pub mod prelude;
pub mod word;

// 顶层 POI 类
mod global_poi_config;
mod poi_checker;

pub use global_poi_config::GlobalPoiConfig;
pub use poi_checker::PoiChecker;
