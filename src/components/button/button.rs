use stylist::yew::styled_component;
use yew::prelude::*;
#[derive(PartialEq, Properties)]
pub struct ButtonProps {
    pub name: String,
    pub link: String,
}

#[styled_component(DownloadButton)]
pub fn button(props: &ButtonProps) -> Html {
    html! {
        <div>
            <a href={props.link.clone()} download="">
                <button class="border border-black hover:bg-black hover:text-white py-2 px-11 rounded-full">
                    {props.name.clone()}
                </button>
            </a>
        </div>
    }
}
