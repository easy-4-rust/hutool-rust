//! 对齐: `cn.hutool.core.util.JAXBUtil`
//! 来源: hutool-core/src/main/java/cn/hutool/core/util/JAXBUtil.java
//!
//! Rust 使用 quick-xml 手工序列化/反序列化，输出与 Hutool JAXB 测试向量一致。

use quick_xml::events::{BytesEnd, BytesStart, BytesText, Event};
use quick_xml::Reader;
use quick_xml::Writer;

use crate::{CoreError, Result};

use super::school_vo::SchoolVo;

/// 对齐 Java `JAXBUtilTest.SchoolVo.RoomVo`。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RoomVo {
    /// 教室编号。
    pub room_no: String,
    /// 教室名称。
    pub room_name: String,
}
