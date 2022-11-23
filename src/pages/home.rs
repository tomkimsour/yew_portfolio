use yew::prelude::*;
use router::{switch,Route};

use crate::components::profile::profile::Profile;

#[function_component(Home)]
pub fn home() -> Html {

    html! {
        <>
            <Profile/>
            <About/>
            <Footer/>
        </>
    }
}
