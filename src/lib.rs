mod router;
mod components;

use yew::prelude::*;
use yew_router::prelude::*;
use router::{switch,Route};

#[function_component(App)]
pub fn app() -> Html {
    html! {
        // Should load the css for the whole App
        <BrowserRouter>
          // <Navbar />
          // <ErrorMessage />
          <Switch<Route> render={Switch::render(switch)} />
        </BrowserRouter>
    }
}
