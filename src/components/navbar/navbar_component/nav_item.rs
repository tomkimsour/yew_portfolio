use crate::router::Route;
use stylist::{yew::styled_component,css,StyleSource};
use yew::prelude::*;
use yew_router::prelude::*;

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
                // <Link<Route>  to={props.to.clone()} classes={classes!(button_styleshee)}>{ props.name.clone() }</Link<Route>>
                <a class={classes!(button_stylesheet)} href={props.to.clone()}>{props.name.clone()}</a>
            </li>
        </>
    }
}
