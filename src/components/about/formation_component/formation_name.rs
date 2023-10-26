use yew::prelude::*;
use stylist::yew::styled_component;


#[derive(Properties, Clone, PartialEq)]
pub struct FormationNameProps{
  pub name : String,
}

#[styled_component(FormationName)]
pub fn formation_name(props: &FormationNameProps) -> Html {

    html! {
        <div class="formation-name cyan-text">{props.name.clone()}</div>
    }
}
