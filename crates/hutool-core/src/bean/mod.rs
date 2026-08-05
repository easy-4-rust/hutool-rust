//! `cn.hutool.core.bean` 子包对齐
//!
//! 自动生成的模块入口,1:1 镜像 Java 包结构。
//! 每个子模块对应一个 Java 类(`.java` → `.rs`),命名遵循 snake_case。
//! 详细对齐信息见各 `.rs` 文件头注释。

pub mod bean_desc;
pub mod bean_desc_cache;
pub mod bean_exception;
pub mod bean_info_cache;
pub mod bean_path;
pub mod bean_util;
pub mod copier;
pub mod dyna_bean;
pub mod null_wrapper_bean;
pub mod prop_desc;
pub mod record_util;

pub use bean_exception::BeanException;
pub use bean_util::BeanUtil;
