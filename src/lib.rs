mod components;
mod pages;
mod router;

use crate::components::navbar::navbar::Navbar;
use router::{switch, Route};
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        // Should load the css for the whole App
        <BrowserRouter>
          <Navbar/>
          // <ErrorMessage />
          <Switch<Route> render={Switch::render(switch)} />
        </BrowserRouter>
    }
}
