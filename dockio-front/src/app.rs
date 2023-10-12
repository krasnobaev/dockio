use std::collections::HashMap;
use std::time::Duration;

use web_sys::Element;
use yew::platform::time::sleep;
use yew::prelude::*;
use gloo_console::{error, warn, log};

use gloo_net::websocket::{Message, futures::WebSocket};
use wasm_bindgen_futures::spawn_local;
use futures::{SinkExt, StreamExt};

use super::utils;
use super::model;

pub enum Msg {
    UpdateDiagram(Element),
    CleanDiagram,
    UpdateNodes(model::Nodes),
    Suspend,
}

pub struct App {
    apps_container_ref: NodeRef,
    nodes: model::Nodes,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let ctx_copy = ctx.link().clone();

        spawn_local(async move {
            loop {
                let ws = WebSocket::open("ws://localhost:8081").unwrap();
                let (_write, mut read) = ws.split();

                while let Some(msg) = read.next().await {
                    match msg {
                        Ok(Message::Bytes(bytes)) => {
                            log!(format!("received binary"));

                            let (svg_body, nodes) = utils::parse_mxfile_content(bytes);
                            ctx_copy.send_message(Msg::CleanDiagram);
                            ctx_copy.send_message(Msg::UpdateDiagram(svg_body));
                            ctx_copy.send_message(Msg::UpdateNodes(nodes));
                        },
                        Ok(Message::Text(text)) => {
                            log!(format!("received text message: {:?}", text));
                        },
                        Err(err) => {
                            warn!(format!("websocket error: {:?}", err));
                        },
                    }
                }

                log!("WebSocket Closed");
                ctx_copy.send_message(Msg::Suspend);
                sleep(Duration::from_secs(2)).await;
            }
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
            Msg::UpdateDiagram(svg_body) => {
                let app_container = self
                    .apps_container_ref
                    .cast::<Element>()
                    .expect("Failed to cast app container div to HTMLElement");

                app_container
                    .append_child(&svg_body)
                    .expect("Failed to append app div app container div");
            },
            Msg::CleanDiagram => {
                let app_container = self
                    .apps_container_ref
                    .cast::<Element>()
                    .expect("Failed to cast app container div to HTMLElement");

                if let Some(item) = app_container.child_nodes().item(0) {
                    let _ = app_container.remove_child(&item);
                }
            },
            Msg::UpdateNodes(nodes) => {
                self.nodes = nodes;
            },
            Msg::Suspend => {
                ();
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
