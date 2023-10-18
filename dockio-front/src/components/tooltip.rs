use web_sys::HtmlElement;
use gloo::console::trace;
use yew::prelude::*;

use super::super::styles;

pub enum Msg {
    Show,
    Hide,
}

#[derive(Properties, Debug, PartialEq)]
pub struct Props {
    pub x: String,
    pub y: String,
    pub text: String,
}

pub struct Tooltip {
    node_ref: NodeRef,
}

impl Component for Tooltip {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        gloo::utils::document().head().unwrap().append_child(&styles::visible_style()).unwrap();
        gloo::utils::document().head().unwrap().append_child(&styles::hidden_style()).unwrap();

        Self {
            node_ref: NodeRef::default(),
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        if ctx.props().text != old_props.text {
            trace!("tooltip text changed");
            if ctx.props().text.len() > 0 {
                ctx.link().send_message(Msg::Show);
            } else {
                ctx.link().send_message(Msg::Hide);
            }
        } else {
            trace!("tooltip text not changed");
            return false
        }

        true
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Show => {
                let tooltip = self.node_ref.cast::<HtmlElement>().unwrap();
                tooltip.set_attribute("class", "visible").unwrap();
            },
            Msg::Hide => {
                let tooltip = self.node_ref.cast::<HtmlElement>().unwrap();
                tooltip.set_attribute("class", "hidden").unwrap();
            },
        }

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <tooltip
                id="tooltip"
                ref={self.node_ref.clone()}
                x={ctx.props().x.clone()}
                y={ctx.props().y.clone()}
            >
                { &ctx.props().text }
            </tooltip>
        }
    }
}
