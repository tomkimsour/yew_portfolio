mod components;
mod pages;
mod router;

use crate::components::{
  footer::footer::Footer, 
  navbar::navbar::Navbar,
  button::color_theme_button::ColorThemeButton
};

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
          <ColorThemeButton/>
          <Switch<Route> render={switch} />
          <Footer/>
        </BrowserRouter>
    }
}
