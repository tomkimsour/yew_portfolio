mod components;
mod pages;
mod router;

use crate::components::{
    button::color_theme_button::ColorThemeButton, footer::footer::Footer, navbar::navbar::Navbar,
};

use router::{switch, Route};
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
      <BrowserRouter>
        <Navbar/>
        <ColorThemeButton/>
        <Switch<Route> render={switch} />
        <Footer/>
      </BrowserRouter>
    }
}
