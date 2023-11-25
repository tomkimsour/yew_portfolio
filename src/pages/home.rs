use yew::prelude::*;

use crate::components::{
    about::about::About, profile::profile::Profile, project::projects::Projects,
};

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <div class="h-min lg:h-full flex flex-nowrap flex-col lg:px-24 pb-20 bg-bkg text-content">
            <Profile/>
            <About/>
            <Projects/>
        </div>
    }
}
