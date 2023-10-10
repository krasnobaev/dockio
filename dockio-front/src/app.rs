use yew::prelude::*;
use gloo_console::{warn,log};

use gloo_net::websocket::{Message, futures::WebSocket};
use wasm_bindgen_futures::spawn_local;
use futures::{SinkExt, StreamExt};

use web_sys::{DomParser, SupportedType, Element};
use gloo::utils::document;

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

                    let parser = DomParser::new().unwrap();
                    let doc = parser.parse_from_string(&dia, SupportedType::TextHtml).unwrap().body().unwrap();
                    log!(format!("parsed {:?}", doc));

                    // TODO: use app_ref
                    // TODO: use yew's update/Message mechanism
                    // body.append_child(&svg).unwrap();
                    document().body().unwrap().append_child(&doc).unwrap();
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
