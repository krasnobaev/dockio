use yew::prelude::*;
use gloo_console::{warn,log};

use gloo_net::websocket::{Message, futures::WebSocket};
use wasm_bindgen_futures::spawn_local;
use futures::{SinkExt, StreamExt};

use web_sys::{DomParser, SupportedType};
use gloo::utils::document;

use base64::{Engine as _, engine::general_purpose};
use flate2::read::DeflateDecoder;
use std::io::prelude::*;
use js_sys::decode_uri_component;

pub enum Msg {
}

pub struct App {
    app_ref: NodeRef,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let ws = WebSocket::open("ws://localhost:8081").unwrap();
        let (mut write, mut read) = ws.split();

        spawn_local(async move {
            write.send(Message::Text(String::from("ehlo"))).await.unwrap();
        });

        spawn_local(async move {
            while let Some(msg) = read.next().await {
                if let Ok(Message::Bytes(bytes)) = msg {
                    let dia = String::from_utf8(bytes).unwrap();
                    // log!(format!("diagram message {:?}", dia));

                    // embed svg into body
                    let parser = DomParser::new().unwrap();
                    let svg_doc = parser.parse_from_string(&dia, SupportedType::TextHtml).unwrap();
                    let svg_body = svg_doc.body().unwrap().children().item(0).unwrap();

                    // TODO: use app_ref
                    // TODO: use yew's update/Message mechanism
                    // body.append_child(&svg).unwrap();
                    document().body().unwrap().append_child(&svg_body).unwrap();

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
                    log!(format!("content {:?}", text));
                    let xml_doc = parser.parse_from_string(&text, SupportedType::TextHtml).unwrap();

                    // map mx objects data into json
                    // let xml_body = xml_doc.body().unwrap();
                    let mx = xml_doc.get_elements_by_tag_name("object");

                    log!(format!("mx {:?}", mx.length()));
                    for i in 0..mx.length() {
                        let item = mx.item(i).unwrap();

                        let geo = item.get_elements_by_tag_name("mxGeometry").item(0).unwrap();
                        let x = geo.get_attribute("x").unwrap_or("0".to_owned());
                        let y = geo.get_attribute("y").unwrap_or("0".to_owned());
                        let value = item.get_attribute("value").unwrap_or("".to_owned());
                        let cname = item.get_attribute("cname").unwrap_or("".to_owned());

                        let tuple = (x, y, value, cname);
                        log!(format!("tuple {:?}", tuple));
                    }
                } else {
                    warn!(format!("expected binary message with diagram, received {msg:?}"));
                }
            }
            log!("WebSocket Closed")
        });

        Self {
            app_ref: NodeRef::default(),
        }
    }

    fn changed(&mut self, _ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        false
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <app>
            </app>
        }
    }
}
