use crate::router::Route;
use stylist::{yew::styled_component,css,StyleSource};
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct NavItemProps {
    pub to: Route,
    pub name: String,
}

#[styled_component(NavItem)]
pub fn nav_item(props: &NavItemProps) -> Html {
    let button_styleshee : StyleSource = css!(
        r#"
            padding-left : 15px;
            padding-right: 15px;
        "#
    );
    html! {
        <>
            <li class="inline-block">
                <Link<Route>  to={props.to.clone()} classes={classes!(button_styleshee)}>{ props.name.clone() }</Link<Route>>
            </li>
        </>
    }
}
