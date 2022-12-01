use yew::prelude::*;
use stylist::yew::styled_component;
#[derive(PartialEq, Properties)]
pub struct ButtonProps {
    pub name: String,
}

#[styled_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    html! {
        <div>
        <button class="border-1 border-black text-black font-bold hover:bg-black hover:text-white  py-2 px-8 rounded-full">
            {props.name.clone()}
        </button>
        </div>
    }
}