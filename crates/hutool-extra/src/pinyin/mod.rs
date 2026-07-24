//! Pinyin helpers aligned with Hutool `PinyinUtil`, backed by the `pinyin` crate.

use pinyin::{ToPinyin, ToPinyinMulti};

mod pinyin_exception;
mod pinyin_engine;
mod default_pinyin_engine;
mod pinyin_factory;
mod pinyin_util;
mod houbb_pinyin_engine;
mod j_pinyin_engine;
mod pinyin4j_engine;
mod tiny_pinyin_engine;
mod bopomofo4j_engine;

pub use pinyin_exception::PinyinException;
pub use pinyin_engine::PinyinEngine;
pub use default_pinyin_engine::DefaultPinyinEngine;
pub use pinyin_factory::PinyinFactory;
pub use pinyin_util::PinyinUtil;
pub use houbb_pinyin_engine::HoubbPinyinEngine;
pub use j_pinyin_engine::JPinyinEngine;
pub use pinyin4j_engine::Pinyin4jEngine;
pub use tiny_pinyin_engine::TinyPinyinEngine;
pub use bopomofo4j_engine::Bopomofo4jEngine;
