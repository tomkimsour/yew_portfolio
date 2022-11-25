use yew::prelude::*;
// use router::{switch,Route};

use crate::components::{
    footer::footer::Footer, 
    profile::profile::Profile,
    about::about::About
};

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
