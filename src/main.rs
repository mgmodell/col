mod app;
pub mod entities;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
