use stylist::{yew::styled_component,css,StyleSource};
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct RefNavItemProps {
    pub to: String,
    pub name: String,
}

#[styled_component(RefNavItem)]
pub fn ref_nav_item(props: &RefNavItemProps) -> Html {
    let button_stylesheet : StyleSource = css!(
        r#"
            padding-left : 15px;
            padding-right: 15px;
        "#
    );
    html! {
        <>
            <li class="inline-block">
                <a class={classes!(button_stylesheet)} href={props.to.clone()}>{props.name.clone()}</a>
            </li>
        </>
    }
}
