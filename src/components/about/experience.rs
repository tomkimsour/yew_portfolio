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
        <section>
          <h2 class="">{props.category_title.clone()}</h2>
          <div class="w-5/6 grid gap-2">
            {props.experiences.iter().map( |experience| html!{
                <div id="experience" key={experience.key}>
                    <div class="title cyan-text">
                    {experience.title.clone()}
                    </div>
                    if !experience.description.is_empty(){
                        <div>
                            {experience.description.clone()}
                        </div>
                    }

                    <div class="grey-text">{experience.technos.join(", ")}</div>
                </div>
            }).collect::<Html>()
            }
          </div>
        </section>
    }
}
