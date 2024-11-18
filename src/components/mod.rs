pub mod some_component;

use crate::routers::main_router::MainRouterSwitcher;
use yew::{html, Component, Context, Html};
use yew_router::HashRouter;

pub struct RootComponent {}

impl Component for RootComponent {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        RootComponent {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <HashRouter>
                <MainRouterSwitcher/>
                <footer style="margin-top: 100px;"><a href="https://github.com/averichev/yew-starter-template">{"Source of this starter template"}</a></footer>
            </HashRouter>
        }
    }
}
