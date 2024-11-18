use crate::components::some_component::SomeComponent;
use crate::routers::main_router::MainRouter;
use yew::{html, Component, Context, Html};
use yew_router::components::Link;

pub struct AnotherView {}

impl Component for AnotherView {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        AnotherView {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
            <h1>{"Another view"}</h1>
            <div>
                <SomeComponent />
            </div>
            <Link<MainRouter> to={MainRouter::Main}>{ "Main view" }</Link<MainRouter>>
            </>
        }
    }
}
