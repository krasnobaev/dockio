use yew::prelude::*;
use gloo_console::{warn,log};

use gloo_net::websocket::{Message, futures::WebSocket};
use wasm_bindgen_futures::spawn_local;
use futures::{SinkExt, StreamExt};

pub enum Msg {
}

pub struct App {
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
                    log!(format!("diagram message {:?}", dia));
                } else {
                    warn!(format!("expected binary message with diagram, received {msg:?}"));
                }
            }
            log!("WebSocket Closed")
        });

        Self {}
    }

    fn changed(&mut self, _ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        false
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <main>
            </main>
        }
    }
}
