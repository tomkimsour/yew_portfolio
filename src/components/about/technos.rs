use stylist::yew::styled_component;
use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub struct TechnosStruct {
    pub key: u16,
    pub title: String,
    pub names: Vec<String>,
}

#[derive(Properties, Clone, PartialEq)]
pub struct TechnosProps {
    pub category_title: String,
    pub technos: Vec<TechnosStruct>,
}

#[styled_component(Technos)]
pub fn technos(props: &TechnosProps) -> Html {
    let title = props.category_title.clone();
    let technos = props.clone().technos;
    html! {
    <section>
      <h2 class="categoryTitle">{title}</h2>
      <div class="grid gap-2">
      {
        technos.iter().map(|techno| html!{
          <div key={techno.key}>
            <p class="cyan-text">
              {techno.title.clone()}
            </p>
            <p>{techno.names.join(" - ")}</p>
          </div>
        }).collect::<Html>()
      }
      </div>
    </section>
    }
}
