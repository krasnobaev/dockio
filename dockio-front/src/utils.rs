use gloo::console::{debug, log, info};

use base64::{engine::general_purpose, Engine as _};
use flate2::read::DeflateDecoder;
use js_sys::decode_uri_component;
use std::io::prelude::*;
use web_sys::{DomParser, Element, SupportedType};

use crate::model;

pub fn parse_mxfile_content(bytes: Vec<u8>) -> Option<(Element, crate::model::Nodes)> {
    let dia = String::from_utf8(bytes).unwrap();
    debug!(format!("diagram message {:?}", dia.replace("\n", "")));

    // build svg DOM Object
    let parser = DomParser::new().unwrap();
    let svg_doc = parser
        .parse_from_string(&dia, SupportedType::TextHtml)
        .unwrap();
    let svg_body = svg_doc.body().unwrap().children().item(0).unwrap();

    // extract mxfile from SVG tag 'content' attribute
    let mxfile_tag = svg_body.get_attribute(&"content").unwrap();
    debug!(format!("mxfile_tag {:?}", mxfile_tag));
    let text = if mxfile_tag.contains("mxGraphModel") {
        info!("mxfile tag is not compressed, skipping decompression and decoding");
        mxfile_tag
    } else {
        let mxfile_tag = parser
            .parse_from_string(&mxfile_tag, SupportedType::TextHtml)
            .unwrap();
        let mx_data = mxfile_tag.body().unwrap().children().item(0).unwrap().text_content().unwrap();
        debug!(format!("mx_data {:?}", mx_data));

        // decode mxfile contents into xml object
        let bytes = general_purpose::STANDARD.decode(mx_data).unwrap();
        let mut decoder = DeflateDecoder::new(bytes.as_slice());
        let mut text = String::new();
        decoder.read_to_string(&mut text).unwrap();
        text
    };

    let text = decode_uri_component(&text).unwrap().as_string().unwrap();
    debug!(format!("content {:?}", text.replace("\n", "")));
    let xml_doc = parser
        .parse_from_string(&text, SupportedType::TextHtml)
        .unwrap();

    debug!(format!("xml_doc {:?}", xml_doc));

    // map mx objects data into json
    let mx = xml_doc.get_elements_by_tag_name("object");
    log!(format!("mx {:?}", mx.length()));

    let mut nodes = model::Nodes(std::collections::HashMap::new());
    for i in 0..mx.length() {
        let object = mx.item(i).unwrap();
        let mx_cell = object.get_elements_by_tag_name("mxCell").item(0).unwrap();
        let mx_geo = object.get_elements_by_tag_name("mxGeometry").item(0).unwrap();

        let x = mx_geo
            .get_attribute("x")
            .unwrap_or("0".to_owned())
            .parse()
            .unwrap();
        let y = mx_geo
            .get_attribute("y")
            .unwrap_or("0".to_owned())
            .parse()
            .unwrap();
        let value = object.get_attribute("value").unwrap_or("".to_owned());
        let mut cname = object.get_attribute("cname").unwrap_or("".to_owned());
        let server = object.get_attribute("server").unwrap_or("".to_owned());

        // extract conainer id from rotation attribute '0.00101' => 1
        let cid = parse_cid_from_mxfile_rotation(mx_cell
            .get_attribute("style")
            .unwrap_or("".to_owned()));

        let orchestrator = if let Some(sname) = object.get_attribute("sname") {
            cname = sname;
            model::Orchestrator::SystemD
        } else {
            model::Orchestrator::Docker
        };

        nodes
            .0
            .insert(model::NodeKey(cname.clone(), server.clone()), model::Node {
                x,
                y,
                value,
                cname,
                cid,
                orchestrator,
                server,
            });
    }

    Some((svg_body, nodes))
}

// container id <--> (mxfile/svg attribute / CSSSelector) transformation

pub fn parse_cid_from_mxfile_rotation(rotation: String) -> u16 {
    rotation
        .split(";")
        .find_map(|s| s.strip_prefix("rotation="))
        .and_then(|s| s.split(".").last())
        .and_then(|s| s.parse::<u16>().ok())
        .unwrap_or(0) % 1000 / 10
}

pub fn parse_cid_from_svg_rotation(rotation: String) -> u16 {
    rotation
        .split("rotate")
        .find_map(|s| s.strip_prefix("("))
        .and_then(|s| s.split(" ").nth(0))
        .and_then(|s| s.split(".").last())
        .and_then(|s| s.parse::<u16>().ok())
        .unwrap_or(0) % 1000 / 10
}

// assuming cid is any integer
pub fn cid_into_css_selector<T: std::fmt::Display>(cid: T) -> String {
    format!(":has(+ g[transform*='0.001{:02}9'])", cid)
}
