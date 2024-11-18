use crate::routers::main_router::MainRouter;
use yew::{html, Component, Context, Html};
use yew_router::prelude::Link;

pub struct MainView {}

impl Component for MainView {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        MainView {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
            <h1>{"Hello world!"}</h1>
            <img src={"img/logo.svg"} />
            <Link<MainRouter> to={MainRouter::Another}>{ "Another view" }</Link<MainRouter>>
            </>
        }
    }
}
