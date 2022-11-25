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
        <div>
          <h2 class="categoryTitle">{props.category_title.clone()}</h2>
          {props.languages.iter().map( |language| html!{
              <div class="language" key={language.key}>
                <div class="title">
                  {language.name.clone()}
                </div>
                <div class="level">{language.level.clone()}</div>
              </div>
          }).collect::<Html>()
        }
        </div>
    }
}
