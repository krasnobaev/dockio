use std::collections::HashMap;
use std::time::Duration;

use web_sys::Element;
use yew::platform::time::sleep;
use yew::prelude::*;
use gloo::console::{warn, info, log, debug};

use gloo::net::websocket::{Message, futures::WebSocket};
use wasm_bindgen_futures::spawn_local;
use futures::StreamExt;

use super::utils;
use super::model;

pub enum Msg {
    UpdateDiagram(Element),
    CleanDiagram,
    UpdateNodes(model::Nodes),
    Disconnected,
    UpdateStyles(model::Containers, String),
}

pub struct App {
    apps_container_ref: NodeRef,
    nodes: model::Nodes,
    styles: HashMap<model::NodeKey, Element>
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

                            if let Some((svg_body, nodes)) = utils::parse_mxfile_content(bytes) {
                                ctx_copy.send_message(Msg::CleanDiagram);
                                ctx_copy.send_message(Msg::UpdateDiagram(svg_body));
                                ctx_copy.send_message(Msg::UpdateNodes(nodes));
                            } else {
                                ctx_copy.send_message(Msg::Disconnected);
                            }
                        },
                        Ok(Message::Text(text)) => {
                            let containers: model::Containers = serde_json::from_str(&text).unwrap();
                            debug!(format!("received text message: {:?}", containers));
                            ctx_copy.send_message(Msg::UpdateStyles(containers, "localhost".to_owned()));
                        },
                        Err(err) => {
                            warn!(format!("websocket error: {:?}", err));
                        },
                    }
                }

                log!("WebSocket Closed");
                ctx_copy.send_message(Msg::Disconnected);
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
                info!(format!("UpdateNodes: {:?}", nodes));
                self.nodes = nodes;

                self.nodes.0.iter().for_each(|(_, node)| {
                    let style = gloo::utils::document().create_element("style").unwrap();
                    style.set_attribute("type", "text/css").unwrap();
                    gloo::utils::document().head().unwrap().append_child(&style).unwrap();
                    self.styles.insert(model::NodeKey(node.cname.clone(), node.server.clone()), style);
                });
            },
            Msg::UpdateStyles(containers, server) => {
                containers.iter().for_each(|container| {
                    let cname = container.names.clone();
                    let key = model::NodeKey (cname.clone(), server.clone());

                    if self.nodes.0.contains_key(&key) {
                        let node = self.nodes.0.get(&key).unwrap();
                        let model::Node {cid, ..} = node;
                        let q = if container.state == "running" { 100 } else { 2 };

                        let style = self.styles.get(&key).unwrap();
                        style.set_inner_html(&format!(r#"{} {{
                            /* container {cname} available */
                            filter: invert({}%) sepia({}%) saturate(1352%) hue-rotate({}deg) brightness(119%) contrast(119%);
                        }}"#, utils::cid_into_css_selector(cid), q, q, q));
                    } else {
                        debug!(format!("container {} found in server response, but not on diagram", cname.clone()));
                    }
                });

                self.nodes.0.keys().filter(|key| {
                    !containers.iter().any(|container| {
                        container.names == *key.0
                    })
                }).for_each(|key| {
                    let node = self.nodes.0.get(key).unwrap();
                    let model::Node { cname, cid, ..} = node;
                    let key = model::NodeKey(cname.clone(), server.clone());

                    if let Some(style) = self.styles.get(&key) {
                        style.set_inner_html(&format!(r#"{} {{
                            /* container {cname} not available */
                            filter: brightness(11%) contrast(11%);
                        }}"#, utils::cid_into_css_selector(cid)));
                    } else {
                        warn!(format!("container {} not found in server response, but on diagram", cname.clone()));
                    }
                });
            },
            Msg::Disconnected => {
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
