use crate::components::navbar::navbar_component::nav_item::{RefNavItem, RefNavItemProps};
use crate::Route;
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
                name: "Introduction".to_string(),
                to: "#profile".to_string(),
            },
        },
        NavBarProps {
            key: 2,
            nav_item: RefNavItemProps {
                name: "About".to_string(),
                to: "#about".to_string(),
            },
        },
        NavBarProps {
            key: 3,
            nav_item: RefNavItemProps {
                name: "Projects".to_string(),
                to: "#project".to_string(),
            },
        },
        NavBarProps {
            key: 4,
            nav_item: RefNavItemProps {
                name: "Contact".to_string(),
                to: "#contact".to_string(),
            },
        },
    ];

    html! {
        <>
            <nav class="fixed w-full mt-0 pr-40 py-14 justify-end">
              <ul class="list-none">
                {
                    nav_items.into_iter().map(|nav_bar| {
                        html!{<RefNavItem key={nav_bar.key} name={nav_bar.nav_item.name} to={nav_bar.nav_item.to}/>}
                    }).collect::<Html>()
                }
              </ul>
            </nav>
        </>
    }
}
