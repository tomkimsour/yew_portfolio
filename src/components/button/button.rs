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
            <button class="border-b border-amber-950 text-black font-bold hover:bg-black hover:text-white py-2 px-7 rounded-full">
                {props.name.clone()}
            </button>
        </div>
    }
}