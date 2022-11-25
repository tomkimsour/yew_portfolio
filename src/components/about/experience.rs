// import { ExperienceCategory } from "./Interfaces.ts";
use stylist::yew::styled_component;
use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub struct ExperienceStruct {
    pub key: u16,
    pub title: String,
    pub description: String,
    pub technos: Vec<String>,
}
#[derive(Properties, Clone, PartialEq)]
pub struct ExperienceProps {
    pub category_title: String,
    pub experiences: Vec<ExperienceStruct>,
}

#[styled_component(Experience)]
pub fn experience(props: &ExperienceProps) -> Html {
    html! {
        <div>
          <h2 class="categoryTitle">{props.category_title.clone()}</h2>
          {props.experiences.iter().map( |experience| html!{
            <div class="experience" key={experience.key}>
                <div class="title">
                {experience.title.clone()}
                </div>
                if !experience.description.is_empty(){
                    <div class="description">
                        {experience.description.clone()}
                    </div>
                }
                
                <div class="technos">{experience.technos.join(", ")}</div>
            </div>
          }).collect::<Html>()
        }
        </div>
    }
}
