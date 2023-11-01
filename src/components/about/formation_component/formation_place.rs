use yew::prelude::*;
use stylist::yew::styled_component;

#[derive(Properties, Clone, PartialEq)]
pub struct FormationPlaceProps{
  pub place : String,
}

#[styled_component(FormationPlace)]
pub fn formation_place(props: &FormationPlaceProps) -> Html {

    html! {
        <div>{props.place.clone()}</div>
    }
}
