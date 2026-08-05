//! `cn.hutool.core.text` 子包对齐
//!
//! 自动生成的模块入口,1:1 镜像 Java 包结构。
//! 每个子模块对应一个 Java 类(`.java` → `.rs`),命名遵循 snake_case。
//! 详细对齐信息见各 `.rs` 文件头注释。

pub mod ant_path_matcher;
pub mod ascii_str_cache;
pub mod char_pool;
pub mod char_sequence_util;
pub mod csv;
pub mod escape;
pub mod finder;
pub mod naming_case;
pub mod passwd_strength;
pub mod replacer;
pub mod simhash;
pub mod split;
pub mod str_builder;
pub mod str_ext;
pub mod str_formatter;
pub mod str_joiner;
pub mod str_matcher;
pub mod str_pool;
pub mod str_splitter;
pub mod str_util;
pub mod text_similarity;
pub mod unicode_util;

pub use ant_path_matcher::AntPathMatcher;
pub use ascii_str_cache::{ASCIIStrCache, AsciiStrCache};
pub use char_sequence_util::CharSequenceUtil;
pub use naming_case::NamingCase;
pub use passwd_strength::{PasswdLevel, PasswdStrength};
pub use str_builder::StrBuilder;
pub use str_formatter::StrFormatter;
pub use str_joiner::{NullMode, StrJoiner};
pub use str_matcher::StrMatcher;
pub use str_splitter::StrSplitter;
pub use text_similarity::TextSimilarity;
pub use unicode_util::UnicodeUtil;
