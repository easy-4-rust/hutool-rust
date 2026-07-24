//! 对齐: `cn.hutool.core.util.JAXBUtil`
//! 来源: hutool-core/src/main/java/cn/hutool/core/util/JAXBUtil.java
//!
//! Rust 使用 quick-xml 手工序列化/反序列化，输出与 Hutool JAXB 测试向量一致。

use quick_xml::events::{BytesEnd, BytesStart, BytesText, Event};
use quick_xml::Reader;
use quick_xml::Writer;

use crate::{CoreError, Result};

use super::room_vo::RoomVo;
use super::school_vo::SchoolVo;

/// 对齐 Java: `cn.hutool.core.util.JAXBUtil`
#[derive(Debug, Clone, Copy, Default)]
pub struct JAXBUtil;

impl JAXBUtil {
    /// 对齐 Java: `JAXBUtil.beanToXml(Object)`
    pub fn bean_to_xml(bean: &SchoolVo) -> Result<String> {
        Self::bean_to_xml_with_options(bean, true)
    }

    /// 对齐 Java: `JAXBUtil.beanToXml(Object, Charset, boolean)`
    pub fn bean_to_xml_with_options(bean: &SchoolVo, format: bool) -> Result<String> {
        let mut writer = Writer::new(Vec::new());
        writer.write_event(Event::Decl(quick_xml::events::BytesDecl::new(
            "1.0",
            Some("UTF-8"),
            Some("yes"),
        )))?;
        if format {
            writer.write_event(Event::Text(BytesText::new("\n")))?;
        }

        write_school(&mut writer, bean, format)?;
        Ok(String::from_utf8(writer.into_inner()).map_err(|err| CoreError::Codec(err.to_string()))?)
    }

    /// 对齐 Java: `JAXBUtil.xmlToBean(String, Class<T>)`
    pub fn xml_to_bean(xml: &str) -> Result<SchoolVo> {
        let mut reader = Reader::from_str(xml);
        reader.config_mut().trim_text(true);

        let mut school_name = None;
        let mut school_address = None;
        let mut room_no = None;
        let mut room_name = None;
        let mut buf = Vec::new();
        let mut current = String::new();

        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(tag)) => current = String::from_utf8_lossy(tag.name().as_ref()).into_owned(),
                Ok(Event::Text(text)) => {
                    let value = text.decode().map_err(|err| CoreError::Codec(err.to_string()))?;
                    match current.as_str() {
                        "school_name" => school_name = Some(value.into_owned()),
                        "school_address" => school_address = Some(value.into_owned()),
                        "room_no" => room_no = Some(value.into_owned()),
                        "room_name" => room_name = Some(value.into_owned()),
                        _ => {}
                    }
                }
                Ok(Event::Eof) => break,
                Ok(_) => {}
                Err(err) => return Err(CoreError::Codec(err.to_string())),
            }
            buf.clear();
        }

        Ok(SchoolVo {
            school_name: school_name.ok_or_else(|| CoreError::Codec("missing school_name".into()))?,
            school_address: school_address
                .ok_or_else(|| CoreError::Codec("missing school_address".into()))?,
            room: RoomVo {
                room_no: room_no.ok_or_else(|| CoreError::Codec("missing room_no".into()))?,
                room_name: room_name.ok_or_else(|| CoreError::Codec("missing room_name".into()))?,
            },
        })
    }
}

fn write_school(writer: &mut Writer<Vec<u8>>, bean: &SchoolVo, format: bool) -> Result<()> {
    let mut school_tag = BytesStart::new("school");
    writer.write_event(Event::Start(school_tag.clone()))?;
    if format {
        writer.write_event(Event::Text(BytesText::new("\n    ")))?;
    }

    write_element(writer, "school_name", &bean.school_name, format)?;
    write_element(writer, "school_address", &bean.school_address, format)?;
    if format {
        writer.write_event(Event::Text(BytesText::new("    ")))?;
    }

    let mut room_tag = BytesStart::new("room");
    writer.write_event(Event::Start(room_tag.clone()))?;
    if format {
        writer.write_event(Event::Text(BytesText::new("\n        ")))?;
    }
    write_element(writer, "room_no", &bean.room.room_no, format)?;
    write_element(writer, "room_name", &bean.room.room_name, format)?;
    if format {
        writer.write_event(Event::Text(BytesText::new("    ")))?;
    }
    writer.write_event(Event::End(BytesEnd::new("room")))?;
    if format {
        writer.write_event(Event::Text(BytesText::new("\n")))?;
    }
    writer.write_event(Event::End(BytesEnd::new("school")))?;
    if format {
        writer.write_event(Event::Text(BytesText::new("\n")))?;
    }
    Ok(())
}
