mod app;
mod model;
mod utils;
mod components;
mod styles;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
