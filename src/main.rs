use crate::components::RootComponent;

mod components;
mod routers;
mod views;

fn main() {
    yew::Renderer::<RootComponent>::new().render();
}
