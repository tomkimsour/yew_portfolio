use crate::components::navbar::navbar_component::nav_item::{RefNavItem, RefNavItemProps};
use stylist::yew::styled_component;
use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub struct NavBarProps {
    pub key: u16,
    pub nav_item: RefNavItemProps,
}

#[styled_component(Navbar)]
pub fn navbar() -> Html {
    let nav_items = vec![
        NavBarProps {
            key: 1,
            nav_item: RefNavItemProps {
                name: "ABOUT".to_string(),
                to: "#about".to_string(),
            },
        },
        NavBarProps {
            key: 2,
            nav_item: RefNavItemProps {
                name: "PROJECTS".to_string(),
                to: "#project".to_string(),
            },
        },
        NavBarProps {
            key: 3,
            nav_item: RefNavItemProps {
                name: "CONTACT".to_string(),
                to: "#contact".to_string(),
            },
        },
    ];

    let nav_items = nav_items.into_iter().
                        map(|nav_bar| {
                        html!{<RefNavItem key={nav_bar.key} name={nav_bar.nav_item.name} to={nav_bar.nav_item.to}/>}
                    }).collect::<Html>();

    html! {
        <>
            <nav class="z-50 grid box-border fixed w-full justify-end px-24 py-16 text-content">
                <div class="spacer"></div>
                <div class="m-auto grid gap-6 grid-flow-col">
                    {nav_items}
                </div>
            </nav>
        </>
    }
}
