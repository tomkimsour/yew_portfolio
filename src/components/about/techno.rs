use yew::prelude::*;
use stylist::yew::styled_component;

#[derive(Clone, PartialEq)]
pub struct TechnoStruct {
  pub key : u16,
  pub title: String,
  pub names: Vec<String>,
}

#[derive(Properties, Clone, PartialEq)]
pub struct TechnoProps{
  pub category_title: String,
  pub technos: Vec<TechnoStruct>,
}

#[styled_component(Techno)]
pub fn techno(props: &TechnoProps) -> Html {
    let title = props.category_title.clone();
    let technos = props.clone().technos;
    html! {
    <div class="pr-2">
      <h2 class="categoryTitle">{title}</h2>
      {
        technos.iter().map(|techno| html!{
          <div key={techno.key}>
            <div class="title cyan-text">
              {techno.title.clone()}
            </div>
            <div>{techno.names.join(", ")}</div>
          </div>
        }).collect::<Html>()
      }
    </div>
    }
}