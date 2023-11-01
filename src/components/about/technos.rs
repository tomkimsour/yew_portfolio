use yew::prelude::*;
use stylist::yew::styled_component;

#[derive(Clone, PartialEq)]
pub struct TechnosStruct {
  pub key : u16,
  pub title: String,
  pub names: Vec<String>,
}

#[derive(Properties, Clone, PartialEq)]
pub struct TechnosProps{
  pub category_title: String,
  pub technos: Vec<TechnosStruct>,
}

#[styled_component(Technos)]
pub fn technos(props: &TechnosProps) -> Html {
    let title = props.category_title.clone();
    let technos = props.clone().technos;
    html! {
    <div class="pr-2">
      <h2 class="categoryTitle">{title}</h2>
      {
        technos.iter().map(|techno| html!{
          <div key={techno.key} class="pb-2">
            <div id="cyan-text" class="title cyan-text">
              {techno.title.clone()}
            </div>
            <div>{techno.names.join(", ")}</div>
          </div>
        }).collect::<Html>()
      }
    </div>
    }
}