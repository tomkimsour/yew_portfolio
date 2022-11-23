use stylist::yew::styled_component;
use yew::prelude::*;
use serde_json::{from_reader,Value};
use std::fs;


#[styled_component(Navbar)]
pub fn navbar() -> Html {

    let file = fs::File::open("../../../assets/nav.json")
        .expect("file should open read only");
    let json: Value = from_reader(file)
        .expect("file should be proper JSON");
    let first_name = json.get("FirstName")
        .expect("file should have FirstName key");


    html! {
        <>
            <nav class="fixed w-full mt-0 pr-40 py-14 justify-end">
              <ul class="list-none">
            {
                data.navItems.into_iter().map(|nav_item, key| {
                    html!{<NavItem key={key} name={nav_item.name} to={nav_item.to}/>}
                }).collect::<Html>()
            }
              </ul>
            </nav>
        </>
    }
}

