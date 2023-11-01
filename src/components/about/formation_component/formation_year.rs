use yew::prelude::*;
use stylist::yew::styled_component;

#[derive(Properties, Clone, PartialEq)]
pub struct FormationYearProps{
  pub year : String,
}

#[styled_component(FormationYear)]
pub fn formation_year(props: &FormationYearProps) -> Html {

    html! {
        <div>{props.year.clone()}</div>
    }
}


