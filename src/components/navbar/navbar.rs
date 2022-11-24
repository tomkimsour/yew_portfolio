use stylist::yew::styled_component;
use yew::prelude::*;
use crate::Route;
use crate::components::navbar::navbar_component::nav_item::{NavItem,NavItemProps};


#[derive(Properties,Clone, PartialEq)]
pub struct NavBarProps {
    pub key: u16,
    pub nav_item: NavItemProps,
}

#[styled_component(Navbar)]
pub fn navbar() -> Html {

    let navItems = vec![
        NavBarProps{key : 0, nav_item : NavItemProps{name :"Introduction".to_string(), to : Route::Profile}},
        NavBarProps{key : 1, nav_item : NavItemProps{name :"About".to_string(), to : Route::About}},
        NavBarProps{key : 2, nav_item : NavItemProps{name :"Projects".to_string(), to : Route::Project}},
        NavBarProps{key : 3, nav_item : NavItemProps{name :"Contact".to_string(), to : Route::Contact}},
    ];

    html! {
        <>
            <nav class="fixed w-full mt-0 pr-40 py-14 justify-end">
              <ul class="list-none">
                {
                    navItems.into_iter().map(|nav_bar| {
                        html!{<NavItem key={nav_bar.key} name={nav_bar.nav_item.name} to={nav_bar.nav_item.to}/>}
                    }).collect::<Html>()
                }
              </ul>
            </nav>
        </>
    }
}

