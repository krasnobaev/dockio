use std::collections::HashMap;

use web_sys::Element;
use yew::prelude::*;
use gloo_console::{warn,log};

use gloo_net::websocket::{Message, futures::WebSocket};
use wasm_bindgen_futures::spawn_local;
use futures::{SinkExt, StreamExt};

use super::utils;
use super::model;

pub enum Msg {
    UpdateDiagram(Element, model::Nodes),
}

pub struct App {
    apps_container_ref: NodeRef,
    nodes: model::Nodes,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let ws = WebSocket::open("ws://localhost:8081").unwrap();
        let (mut write, mut read) = ws.split();

        spawn_local(async move {
            write.send(Message::Text(String::from("ehlo"))).await.unwrap();
        });

        let ctx_copy = ctx.link().clone();
        spawn_local(async move {
            while let Some(msg) = read.next().await {
                if let Ok(Message::Bytes(bytes)) = msg {
                    let (svg_body, nodes) = utils::parse_mxfile_content(bytes);

                    ctx_copy.send_message(Msg::UpdateDiagram(svg_body, nodes));
                } else {
                    warn!(format!("expected binary message with diagram, received {msg:?}"));
                }
            }
            log!("WebSocket Closed")
        });

        Self {
            apps_container_ref: NodeRef::default(),
            nodes: model::Nodes(HashMap::new()),
        }
    }

    fn changed(&mut self, _ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        false
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::UpdateDiagram(svg_body, nodes) => {
                let app_container = self
                    .apps_container_ref
                    .cast::<Element>()
                    .expect("Failed to cast app container div to HTMLElement");

                app_container
                    .append_child(&svg_body)
                    .expect("Failed to append app div app container div");

                self.nodes = nodes;
            },
        }

        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <app ref={self.apps_container_ref.clone()}>
            </app>
        }
    }
}
