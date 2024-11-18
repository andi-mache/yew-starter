use crate::views::another_view::AnotherView;
use crate::views::main_view::MainView;
use yew::{html, Component, Context, Html};
use yew_router::{Routable, Switch};

#[derive(Clone, Routable, PartialEq)]
pub enum MainRouter {
    #[at("/")]
    Main,
    #[at("/another")]
    Another,
}

pub struct MainRouterSwitcher;

impl Component for MainRouterSwitcher {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        MainRouterSwitcher {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <Switch<MainRouter> render={MainRouterSwitcher::switch} />
        }
    }
}

impl MainRouterSwitcher {
    fn switch(routes: MainRouter) -> Html {
        match routes {
            MainRouter::Main => {
                html! {
                    <MainView />
                }
            }
            MainRouter::Another => {
                html! {
                    <AnotherView />
                }
            }
        }
    }
}
