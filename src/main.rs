mod app;
pub mod entities;
pub mod user_components;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
