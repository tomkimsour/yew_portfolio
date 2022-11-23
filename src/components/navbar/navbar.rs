use stylist::yew::styled_component;
use yew::prelude::*;
use yew::function_component;


#[function_component(Navbar)]
pub fn navbar() -> Html {

    html! {
        <>
            <nav class="fixed w-full mt-0 pr-40 py-14 justify-end">
              <ul class="list-none">
            {}
                // {data.navItems.map((navItem, key) => (
                //   <NavItem key={key} name={navItem.name} to={navItem.to} />
                // ))}
              </ul>
            </nav>
        </>
    }
}

