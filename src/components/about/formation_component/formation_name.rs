use stylist::yew::styled_component;
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct FormationNameProps {
    pub name: String,
}

#[styled_component(FormationName)]
pub fn formation_name(props: &FormationNameProps) -> Html {
    html! {
        <p class="cyan-text">{props.name.clone()}</p>
    }
}
