use stylist::yew::styled_component;
use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub struct LanguageStruct {
    pub key: u16,
    pub name: String,
    pub level: String,
}
#[derive(Properties, Clone, PartialEq)]
pub struct LanguageProps {
    pub category_title: String,
    pub languages: Vec<LanguageStruct>,
}

#[styled_component(Languages)]
pub fn languages(props: &LanguageProps) -> Html {
    html! {
        <section class="flex flex-col justify-start justify-items-center">
          <h2>{props.category_title.clone()}</h2>
          <div class="grid gap-2">
          {props.languages.iter().map( |language| html!{
                <div class="grid grid-cols-3 gap" key={language.key}>
                  <div class="cyan-text">
                    {language.name.clone()}
                  </div>
                  <div class="grey-text">{language.level.clone()}</div>
                </div>
            }).collect::<Html>()
          }
          </div>
        </section>
    }
}
