//! 对齐: `cn.hutool.core.util.JAXBUtil`
//! 来源: hutool-core/src/main/java/cn/hutool/core/util/JAXBUtil.java
//!
//! Rust 使用 quick-xml 手工序列化/反序列化，输出与 Hutool JAXB 测试向量一致。

use quick_xml::events::{BytesEnd, BytesStart, BytesText, Event};
use quick_xml::Reader;
use quick_xml::Writer;

use crate::{CoreError, Result};

mod room_vo;
mod school_vo;
mod jaxb_util;

pub use room_vo::RoomVo;
pub use school_vo::SchoolVo;
pub use jaxb_util::JAXBUtil;
