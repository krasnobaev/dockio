use std::collections::HashMap;
use std::time::Duration;

use web_sys::Element;
use yew::platform::time::sleep;
use yew::prelude::*;
use gloo::console::{warn, info, log};

use gloo::net::websocket::{Message, futures::WebSocket};
use wasm_bindgen_futures::spawn_local;
use futures::{SinkExt, StreamExt};

use super::utils;
use super::model;

pub enum Msg {
    UpdateDiagram(Element),
    CleanDiagram,
    UpdateNodes(model::Nodes),
    Disconnect,
    UpdateStyles(model::Containers),
}

pub struct App {
    apps_container_ref: NodeRef,
    nodes: model::Nodes,
    styles: HashMap<String, Element>
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
                            let containers: model::Containers = serde_json::from_str(&text).unwrap();
                            log!(format!("received text message: {:?}", containers));
                            ctx_copy.send_message(Msg::UpdateStyles(containers));
                        },
                        Err(err) => {
                            warn!(format!("websocket error: {:?}", err));
                        },
                    }
                }

                log!("WebSocket Closed");
                ctx_copy.send_message(Msg::Disconnect);
                sleep(Duration::from_secs(2)).await;
            }
        });

        Self {
            apps_container_ref: NodeRef::default(),
            nodes: model::Nodes(HashMap::new()),
            styles: HashMap::new(),
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

                self.nodes.0.iter().for_each(|(_, node)| {
                    let style = gloo::utils::document().create_element("style").unwrap();
                    style.set_attribute("type", "text/css").unwrap();
                    gloo::utils::document().head().unwrap().append_child(&style).unwrap();
                    self.styles.insert(node.cname.clone(), style);
                });
            },
            Msg::UpdateStyles(containers) => {
                containers.iter().for_each(|container| {
                    let cname = &container.names;

                    if self.nodes.0.contains_key(cname) {
                        let node = self.nodes.0.get(cname).unwrap();
                        let model::Node {x, y, ..} = node;
                        let q = if container.state == "running" { 100 } else { 2 };

                        let style = self.styles.get(cname).unwrap();
                        style.set_inner_html(&format!(r#"rect[x="{}"][y="{}"] {{
                            filter: invert({}%) sepia({}%) saturate(1352%) hue-rotate({}deg) brightness(119%) contrast(119%);
                        }}"#, x, y, q, q, q));
                    } else {
                        info!(format!("container {} found in server response, but not on diagram", cname));
                    }
                });

                self.nodes.0.keys().filter(|key| {
                    !containers.iter().any(|container| {
                        container.names == **key
                    })
                }).for_each(|key| {
                    let node = self.nodes.0.get(key).unwrap();
                    let model::Node {x, y, cname, ..} = node;

                    let style = self.styles.get(cname).unwrap();
                    style.set_inner_html(&format!(r#"rect[x="{}"][y="{}"] {{
                        filter: brightness(11%) contrast(11%);
                    }}"#, x, y));
                });
            },
            Msg::Disconnect => {
                self.nodes = model::Nodes(HashMap::new());

                self.styles.iter().for_each(|(_, style)| {
                    let _ = style.remove();
                });

                self.styles = HashMap::new();
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
