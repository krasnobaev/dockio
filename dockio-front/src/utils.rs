use gloo_console::log;

use base64::{Engine as _, engine::general_purpose};
use flate2::read::DeflateDecoder;
use std::io::prelude::*;
use js_sys::decode_uri_component;
use web_sys::{DomParser, SupportedType, Element};

pub fn parse_mxfile_content (bytes: Vec<u8>) -> (Element, crate::model::Nodes) {
    let dia = String::from_utf8(bytes).unwrap();
    // log!(format!("diagram message {:?}", dia));

    // embed svg into body
    let parser = DomParser::new().unwrap();
    let svg_doc = parser.parse_from_string(&dia, SupportedType::TextHtml).unwrap();
    let svg_body = svg_doc.body().unwrap().children().item(0).unwrap();

    // extract mxfile
    let mxfile_tag = svg_body.get_attribute(&"content").unwrap();
    let content = mxfile_tag.split_at(76).1.split_at(mxfile_tag.len() - 76 - 19).0;
    // log!(format!("content {:?}", content));

    // decode mxfile contents into xml object
    let bytes = general_purpose::STANDARD.decode(content).unwrap();
    let mut decoder = DeflateDecoder::new(bytes.as_slice());
    let mut text = String::new();
    decoder.read_to_string(&mut text).unwrap();
    let text = decode_uri_component(&text).unwrap().as_string().unwrap();
    // log!(format!("content {:?}", text));
    let xml_doc = parser.parse_from_string(&text, SupportedType::TextHtml).unwrap();

    // map mx objects data into json
    let mx = xml_doc.get_elements_by_tag_name("object");
    log!(format!("mx {:?}", mx.length()));

    let mut nodes = crate::model::Nodes(std::collections::HashMap::new());
    for i in 0..mx.length() {
        let item = mx.item(i).unwrap();

        let geo = item.get_elements_by_tag_name("mxGeometry").item(0).unwrap();
        let x = geo.get_attribute("x").unwrap_or("0".to_owned()).parse().unwrap();
        let y = geo.get_attribute("y").unwrap_or("0".to_owned()).parse().unwrap();
        let value = item.get_attribute("value").unwrap_or("".to_owned());
        let cname = item.get_attribute("cname").unwrap_or("".to_owned());

        let tuple = (x, y, value, &cname);
        log!(format!("tuple {:?}", tuple));
        nodes.0.insert((x, y), cname);
    }

    (svg_body, nodes)
}
