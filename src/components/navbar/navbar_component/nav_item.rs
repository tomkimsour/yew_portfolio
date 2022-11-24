use yew::prelude::*;
use stylist::yew::styled_component;
use crate::Route;
use yew_router::prelude::Link;


#[derive(Properties,Clone, PartialEq)]
pub struct NavItemProps {
    pub to: Route,
    pub name: String,
}


#[styled_component(NavItem)]
pub fn nav_item(props: &NavItemProps) -> Html {

    html! {
        <>
            <li className="inline-block">
                <Link<Route> classes="px-15px" to={props.to.clone()}>{ props.name.clone() }</Link<Route>>
            </li>
        </>
    }
}

