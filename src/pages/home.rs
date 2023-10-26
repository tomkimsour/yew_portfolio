use yew::prelude::*;

use crate::components::{
    profile::profile::Profile,
    about::about::About,
    project::projects::Projects
};

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <div class="h-full flex flex-nowrap flex-col px-24 pb-20">
            <Profile/>
            <About/>
            <Projects/>
        </div>
    }
}
