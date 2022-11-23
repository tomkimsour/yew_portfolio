use yew::prelude::*;
use stylist::yew::styled_component;


#[derive(Properties, PartialEq)]
pub struct NavItemProps {
    pub to: String,
    pub name: String,
}


#[styled_component(NavItem)]
pub fn nav_item(props: &NavItemProps) -> Html {

    html! {
        <>
            <li className="inline-block">
                <Link<Route> class="px-15px" to={props.to}>{ props.name }</Link<Route>>
            </li>
        </>
    }
}

