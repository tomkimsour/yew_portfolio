use yew::prelude::*;
use stylist::yew::styled_component;


#[derive(Properties, Clone, PartialEq)]
pub struct FormationDescriptionProps{
  pub description : String,
}

#[styled_component(FormationDescription)]
pub fn formation_description(props: &FormationDescriptionProps) -> Html {

    html! {
        <div>{props.description.clone()}</div>
    }
}

