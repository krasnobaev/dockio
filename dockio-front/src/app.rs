use std::collections::HashMap;
use std::time::Duration;
use std::vec;

use web_sys::Element;
use yew::platform::time::sleep;
use yew::prelude::*;
use gloo::console::{error, warn, info, log, debug, trace};

use gloo::net::websocket::{Message, futures::WebSocket};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use futures::StreamExt;

use super::utils;
use super::model;
use super::components::Tooltip;

// const KEY_DEBUG_CUR_STATE_SESSION_STORAGE: &'static str = "cur_state";

pub enum Msg {
    UpdateDiagram(Element),
    CleanDiagram,
    UpdateNodes(model::Nodes),
    Disconnected,
    UpdateStyles(model::Containers, String),
    ServerAsksForRestart,
    OnClick(MouseEvent),
}

pub struct App {
    app_ref: NodeRef,
    svg_ref: NodeRef,

    dia_nodes: model::Nodes,
    dia_styles: HashMap<model::NodeKey, Element>,
    cur_state: HashMap<model::NodeKey, model::Container>,
    selected_node: Option<model::NodeKey>,
    tooltip_text: String,
    hover_listeners: Vec<Option<Closure<dyn Fn(MouseEvent)>>>,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let ctx_copy = ctx.link().clone();

        spawn_local(async move {
            loop {
                let hostname = web_sys::window().unwrap().location().hostname().unwrap();
                let ws = WebSocket::open(&format!("ws://{hostname}:8081")).unwrap();
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
                            match text.as_str() {
                                "Terminated" => {
                                    warn!("Terminated");
                                    ctx_copy.send_message(Msg::ServerAsksForRestart);
                                    break;
                                },
                                _ => {
                                    let containers: model::Containers = serde_json::from_str(&text).unwrap();
                                    debug!(format!("received text message: {:?}", containers));
                                    ctx_copy.send_message(Msg::UpdateStyles(containers, hostname.to_owned()));
                                },
                            }
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
            app_ref: NodeRef::default(),
            svg_ref: NodeRef::default(),

            dia_nodes: model::Nodes(HashMap::new()),
            dia_styles: HashMap::new(),
            cur_state: HashMap::new(),
            selected_node: None,
            tooltip_text: "".to_string(),
            hover_listeners: vec![],
        }
    }

    fn changed(&mut self, _ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        false
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::UpdateDiagram(svg_body) => {
                let svg_container = self
                    .svg_ref
                    .cast::<Element>()
                    .expect("Failed to cast app container div to HTMLElement");

                svg_container
                    .append_child(&svg_body)
                    .expect("Failed to append app div app container div");

                let rects = svg_container
                    .query_selector_all("rect")
                    .expect("Failed to query for rect nodes");

                let show_tooltip = |e: web_sys::MouseEvent| {
                    let target = e.target().unwrap();
                    let target_element = target.dyn_into::<web_sys::SvgRectElement>().unwrap();
                    let x = target_element.get_attribute("x").unwrap_or("".to_owned());
                    let y = target_element.get_attribute("y").unwrap_or("".to_owned());

                    // let sibling = target_element.next_sibling().unwrap().dyn_into::<web_sys::Element>().unwrap();
                    // let rotate = sibling.get_attribute("transform").unwrap_or("".to_owned());
                    // error!(format!("evt {x} {y}, {rotate}"));

                    let document = gloo::utils::document();
                    let tooltip = document.get_element_by_id("tooltip").unwrap();
                    tooltip.set_attribute("x", &x).unwrap();
                    tooltip.set_attribute("y", &y).unwrap();
                };
                let hide_tooltip = |_evt: web_sys::MouseEvent| {
                    let document = gloo::utils::document();
                    let tooltip = document.get_element_by_id("tooltip").unwrap();
                    tooltip.set_attribute("x", "0").unwrap();
                    tooltip.set_attribute("y", "0").unwrap();
                };

                for i in 0..rects.length() {
                    let rect = rects.item(i).unwrap();

                    let listener_show = Closure::<dyn Fn(MouseEvent)>::wrap(Box::new(show_tooltip));
                    let listener_hide = Closure::<dyn Fn(MouseEvent)>::wrap(Box::new(hide_tooltip));

                    rect.add_event_listener_with_callback(
                        "mousemove",
                        listener_show.as_ref().unchecked_ref(),
                    ).unwrap();
                    rect.add_event_listener_with_callback(
                        "mouseleave",
                        listener_hide.as_ref().unchecked_ref(),
                    ).unwrap();

                    self.hover_listeners.push(Some(listener_show));
                    self.hover_listeners.push(Some(listener_hide));
                }
            },
            Msg::CleanDiagram => {
                let app_container = self
                    .svg_ref
                    .cast::<Element>()
                    .expect("Failed to cast app container div to HTMLElement");

                if let Some(item) = app_container.child_nodes().item(0) {
                    let _ = app_container.remove_child(&item);
                }

                self.selected_node = None;
                self.hover_listeners = vec![];
            },
            Msg::UpdateNodes(nodes) => {
                info!(format!("UpdateNodes: {:?}", nodes));
                self.dia_nodes = nodes;

                self.dia_nodes.0.iter().for_each(|(_, node)| {
                    let style = gloo::utils::document().create_element("style").unwrap();
                    style.set_attribute("type", "text/css").unwrap();
                    gloo::utils::document().head().unwrap().append_child(&style).unwrap();
                    self.dia_styles.insert(model::NodeKey(node.cname.clone(), node.server.clone()), style);
                });
            },
            Msg::UpdateStyles(containers, server) => {
                containers.iter().for_each(|container| {
                    let cname = container.names.clone();
                    let key = model::NodeKey (cname.clone(), server.clone());

                    if self.dia_nodes.0.contains_key(&key) {
                        let node = self.dia_nodes.0.get(&key).unwrap();
                        let model::Node {cid, ..} = node;
                        let q = if container.state == "running" { 100 } else { 2 };
                        let selector = utils::cid_into_css_selector(cid);

                        let style = self.dia_styles.get(&key).unwrap();
                        style.set_inner_html(&format!(r#"{} {{
                            /* container {cname} available */
                            filter: invert({}%) sepia({}%) saturate(1352%) hue-rotate({}deg) brightness(119%) contrast(119%);
                        }}"#, selector, q, q, q));

                        trace!(format!("{:?} {:?}", key, container.clone()));
                        self.cur_state.insert(key, container.clone());
                    } else {
                        debug!(format!("container {} found in server response, but not on diagram", cname.clone()));
                    }
                });

                self.dia_nodes.0.keys().filter(|key| {
                    !containers.iter().any(|container| {
                        container.names == *key.0
                    })
                }).for_each(|key| {
                    let node = self.dia_nodes.0.get(key).unwrap();
                    let model::Node { cname, cid, ..} = node;
                    let key = model::NodeKey(cname.clone(), server.clone());

                    if let Some(style) = self.dia_styles.get(&key) {
                        style.set_inner_html(&format!(r#"{} {{
                            /* container {cname} not available */
                            filter: brightness(11%) contrast(11%);
                        }}"#, utils::cid_into_css_selector(cid)));
                    } else {
                        warn!(format!("container {} not found in server response, but on diagram", cname.clone()));
                    }
                });

                // <gloo::storage::SessionStorage as gloo::storage::Storage>::set(KEY_DEBUG_CUR_STATE_SESSION_STORAGE, self.cur_state.clone()).unwrap();
            },
            Msg::Disconnected => {
                self.dia_nodes = model::Nodes(HashMap::new());

                self.dia_styles.iter().for_each(|(_, style)| {
                    let _ = style.remove();
                });

                self.dia_styles = HashMap::new();
            },
            Msg::ServerAsksForRestart => {
                info!("ServerAsksForRestart");

                spawn_local(async move {
                    sleep(Duration::from_secs(2)).await;
                    web_sys::window().unwrap().location().reload().unwrap();
                })
            },

            // Mouse handling

            Msg::OnClick(_e) => {
                let document = gloo::utils::document();
                let tooltip = document.get_element_by_id("tooltip").unwrap();
                let x = tooltip.get_attribute("x").unwrap();
                let y = tooltip.get_attribute("y").unwrap();
                trace!(format!("evt {x} {y}"));

                if x == "0" && y == "0" {
                    return true;
                }

                let selector = format!("[x='{x}'][y='{y}'] + g");
                let node = document.query_selector(&selector).unwrap().unwrap();
                // read transform attribute
                let transform = node.get_attribute("transform").unwrap_or("".to_owned());
                let id = utils::parse_cid_from_svg_rotation(transform);
                trace!(format!("id {}", id));

                let cname = self.dia_nodes.0.iter().find_map(|(key, node)| {
                    if node.cid == id {
                        Some(key.0.clone())
                    } else {
                        None
                    }
                }).unwrap_or("".to_owned());

                if let Some(text) = self.cur_state.get(&model::NodeKey(cname, "ccdev.bdo.ru".to_owned())) {
                    trace!(format!("text {} -> {:?}", id, text));
                    self.tooltip_text = format!("{:#?}", text);
                } else {
                    self.tooltip_text = String::new();
                }
            },
        }

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <app
                ref={self.app_ref.clone()}
                onclick={ctx.link().callback(|e| Msg::OnClick(e))}
            >
                <div id="svg_container" ref={self.svg_ref.clone()} />
                <Tooltip
                    x="0" y="0"
                    text={self.tooltip_text.clone()}
                />
            </app>
        }
    }
}
