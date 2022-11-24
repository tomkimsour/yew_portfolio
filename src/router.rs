use yew::prelude::*;
use yew_router::prelude::*;
use crate::pages::home::Home;


#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/#profile")]
    Profile,
    #[at("/#about")]
    About,
    #[at("/#project")]
    Project,
    #[at("/#contact")]
    Contact,
}

pub fn switch(route: &Route) -> Html {
    match route {
        Route::Home => html!{
            <Home/>
        },
    }
}
