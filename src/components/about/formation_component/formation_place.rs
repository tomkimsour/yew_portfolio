use stylist::yew::styled_component;
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct FormationPlaceProps {
    pub place: String,
}

#[styled_component(FormationPlace)]
pub fn formation_place(props: &FormationPlaceProps) -> Html {
    html! {
        <p>{props.place.clone()}</p>
    }
}
