use crate::components::about::formation_component::formation_description::FormationDescription;
use crate::components::about::formation_component::formation_name::FormationName;
use crate::components::about::formation_component::formation_place::FormationPlace;
use crate::components::about::formation_component::formation_year::FormationYear;
use stylist::yew::styled_component;
use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub struct FormationStruct {
    pub id: u16,
    pub name: String,
    pub description: String,
    pub year: String,
    pub place: String,
}
#[derive(Properties, Clone, PartialEq)]
pub struct FormationProps {
    pub category_title: String,
    pub formations: Vec<FormationStruct>,
}

#[styled_component(Formation)]
pub fn formation(props: &FormationProps) -> Html {
    html! {
        <section class="py-2">
          <h2>{props.category_title.clone()}</h2>
          {
            props.clone().formations.
            into_iter().map( |formation| {
              html!{
                  <div class="formation flex flex-row w-full" key={formation.id}>
                    <div class="w-6/12">
                      <FormationName name={formation.name} />
                      <FormationDescription
                        description={formation.description}
                      />
                    </div>
                    <div class="grey-text w-6/12">
                      <FormationYear year={formation.year} />
                      <FormationPlace place={formation.place} />
                    </div>
                  </div>
              }
            }).collect::<Html>()
          }
        </section>
    }
}
