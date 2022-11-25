use crate::pages::home::Home;
use yew::prelude::*;
use yew_router::prelude::*;

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
        Route::Home => html! {
            <Home/>
        },
        Route::Profile => html! {
            <Home/>
        },
        Route::About => html! {
            <Home/>
        },
        Route::Project => html! {
            <Home/>
        },
        Route::Contact => html! {
            <Home/>
        },
    }
}
