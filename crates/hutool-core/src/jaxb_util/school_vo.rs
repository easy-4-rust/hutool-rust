//! 对齐: `cn.hutool.core.util.JAXBUtil`
//! 来源: hutool-core/src/main/java/cn/hutool/core/util/JAXBUtil.java
//!
//! Rust 使用 quick-xml 手工序列化/反序列化，输出与 Hutool JAXB 测试向量一致。

use quick_xml::events::{BytesEnd, BytesStart, BytesText, Event};
use quick_xml::Reader;
use quick_xml::Writer;

use crate::{CoreError, Result};

use super::room_vo::RoomVo;

/// 对齐 Java `JAXBUtilTest.SchoolVo`。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SchoolVo {
    /// 学校名称。
    pub school_name: String,
    /// 学校地址。
    pub school_address: String,
    /// 教室信息。
    pub room: RoomVo,
}
