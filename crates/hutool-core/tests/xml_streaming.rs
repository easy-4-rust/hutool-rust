//! Regression tests for bounded streaming XML and the compatibility DOM layer.

use std::{io::Cursor, ops::ControlFlow};

use hutool_core::{
    CoreError, NamespaceMode, XmlChild, XmlParseOptions, XmlTransformAction, XmlUtil,
    transform_xml, visit_xml,
};
use quick_xml::events::Event;

#[test]
fn iterative_dom_preserves_qualified_names_and_references() {
    let xml = concat!(
        "<a:root xmlns:a=\"urn:a\" xmlns:b=\"urn:b\">",
        "<b:item a:id=\"42\">A&amp;B</b:item>",
        "</a:root>"
    );
    let document = XmlUtil::parse_xml(xml).expect("parse namespaced XML");

    assert_eq!(document.root.tag, "a:root");
    assert_eq!(document.root.attribute("xmlns:a"), Some("urn:a"));
    let item = XmlUtil::get_element(&document.root, "b:item").expect("item");
    assert_eq!(item.attribute("a:id"), Some("42"));
    assert_eq!(item.text_content(), "A&B");
}

#[test]
fn local_name_mode_is_explicit_and_keeps_namespace_declarations() {
    let options = XmlParseOptions {
        namespace_mode: NamespaceMode::LocalName,
        ..XmlParseOptions::default()
    };
    let document =
        XmlUtil::parse_xml_with_options("<a:root xmlns:a=\"urn:a\"><a:item /></a:root>", &options)
            .expect("parse local names");

    assert_eq!(document.root.tag, "root");
    assert_eq!(document.root.attribute("xmlns:a"), Some("urn:a"));
    assert_eq!(
        XmlUtil::get_element(&document.root, "item").map(|node| node.tag.as_str()),
        Some("item")
    );
}

#[test]
fn defensive_limits_reject_depth_nodes_attributes_text_and_input() {
    let depth = XmlParseOptions {
        max_depth: 2,
        ..XmlParseOptions::default()
    };
    assert!(matches!(
        XmlUtil::parse_xml_with_options("<a><b><c /></b></a>", &depth),
        Err(CoreError::XmlLimit {
            resource: "depth",
            max: 2
        })
    ));

    let nodes = XmlParseOptions {
        max_nodes: 2,
        ..XmlParseOptions::default()
    };
    assert!(matches!(
        XmlUtil::parse_xml_with_options("<a><b /><c /></a>", &nodes),
        Err(CoreError::XmlLimit {
            resource: "node count",
            max: 2
        })
    ));

    let attributes = XmlParseOptions {
        max_attributes_per_element: 1,
        ..XmlParseOptions::default()
    };
    assert!(matches!(
        XmlUtil::parse_xml_with_options("<a x=\"1\" y=\"2\" />", &attributes),
        Err(CoreError::XmlLimit {
            resource: "attributes per element",
            max: 1
        })
    ));

    let text = XmlParseOptions {
        max_text_bytes: 3,
        ..XmlParseOptions::default()
    };
    assert!(matches!(
        XmlUtil::parse_xml_with_options("<a>four</a>", &text),
        Err(CoreError::XmlLimit {
            resource: "text bytes",
            max: 3
        })
    ));

    let input = XmlParseOptions {
        max_input_bytes: 4,
        ..XmlParseOptions::default()
    };
    assert!(matches!(
        XmlUtil::parse_xml_with_options("<a />", &input),
        Err(CoreError::XmlLimit {
            resource: "input bytes",
            max: 4
        })
    ));
}

#[test]
fn doctype_unknown_entities_and_invalid_attributes_are_not_silenced() {
    let doctype = "<!DOCTYPE root [<!ENTITY payload \"boom\">]><root>&payload;</root>";
    assert!(matches!(
        XmlUtil::parse_xml(doctype),
        Err(CoreError::XmlForbidden("DOCTYPE"))
    ));
    assert!(matches!(
        XmlUtil::parse_xml("<root>&unknown;</root>"),
        Err(CoreError::XmlForbidden("unknown general reference"))
    ));
    assert!(XmlUtil::parse_xml("<root a=\"unterminated></root>").is_err());
}

#[test]
fn invalid_char_sanitization_requires_explicit_opt_in() {
    let xml = "<root>a\u{0}b</root>";
    assert!(XmlUtil::parse_xml(xml).is_err());

    let options = XmlParseOptions {
        sanitize_invalid_chars: true,
        ..XmlParseOptions::default()
    };
    let document = XmlUtil::parse_xml_with_options(xml, &options).expect("sanitized XML");
    assert_eq!(document.root.text_content(), "ab");
}

#[test]
fn visitor_can_stop_before_remaining_input_is_read() {
    let xml = Cursor::new(b"<root><first /></root><broken".as_slice());
    let result = visit_xml(xml, XmlParseOptions::default(), |event| {
        if matches!(event, Event::Empty(start) if start.name().as_ref() == b"first") {
            return Ok(ControlFlow::Break("found"));
        }
        Ok(ControlFlow::Continue(()))
    })
    .expect("early visitor result");

    assert_eq!(result, ControlFlow::Break("found"));
}

#[test]
fn transform_uses_writer_and_can_filter_events() {
    let xml =
        Cursor::new(b"<root><!--secret--><item key=\"A&amp;B\">A&amp;B</item></root>".as_slice());
    let output = transform_xml(xml, Vec::new(), XmlParseOptions::default(), |event| {
        Ok(if matches!(event, Event::Comment(_)) {
            XmlTransformAction::Drop
        } else {
            XmlTransformAction::Keep
        })
    })
    .expect("transform");
    let output = String::from_utf8(output).expect("UTF-8 output");

    assert!(!output.contains("secret"));
    assert!(output.contains("key=\"A&amp;B\""));
    assert_eq!(
        XmlUtil::parse_xml(&output)
            .expect("transformed XML")
            .root
            .children,
        vec![XmlChild::Element(hutool_core::XmlNode {
            tag: "item".to_owned(),
            attributes: [("key".to_owned(), "A&B".to_owned())].into_iter().collect(),
            children: vec![XmlChild::Text("A&B".to_owned())],
        })]
    );
}

#[test]
fn dom_writer_escapes_text_and_attributes() {
    let mut document = XmlUtil::parse_xml("<root><item /></root>").expect("parse");
    document
        .root
        .attributes
        .insert("quote".to_owned(), "\" & <".to_owned());
    document
        .root
        .children
        .push(XmlChild::Text("A & B".to_owned()));

    let output =
        XmlUtil::to_string_result(&document, false, true).expect("write XML with escaping");
    assert!(output.contains("quote=\"&quot; &amp; &lt;\""));
    assert!(output.contains("A &amp; B"));
    XmlUtil::parse_xml(&output).expect("writer emitted valid XML");
}

#[cfg(feature = "xml-serde")]
#[test]
fn optional_xml_serde_avoids_json_intermediate() {
    use hutool_core::XmlSerde;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct Message {
        #[serde(rename = "@id")]
        id: u64,
        body: String,
    }

    let message = Message {
        id: 7,
        body: "hello".to_owned(),
    };
    let xml = XmlSerde::to_string(&message).expect("serialize");
    let from_str: Message = XmlSerde::from_str(&xml).expect("deserialize string");
    let from_reader: Message =
        XmlSerde::from_reader(Cursor::new(xml.as_bytes())).expect("deserialize reader");
    let mut output = Vec::new();
    XmlSerde::to_writer(&mut output, &message).expect("serialize writer");

    assert_eq!(from_str, message);
    assert_eq!(from_reader, message);
    assert_eq!(String::from_utf8(output).expect("UTF-8"), xml);
}
