use stylist::yew::styled_component;
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct RefNavItemProps {
    pub to: String,
    pub name: String,
}

#[styled_component(RefNavItem)]
pub fn ref_nav_item(props: &RefNavItemProps) -> Html {
    html! {
            <a class="transition duration-200 transition-opacity color-inherit cursor-pointer opacity-60 hover:opacity-100 hover:decoration-inherit" href={props.to.clone()}>
                {props.name.clone()}
            </a>
    }
}
