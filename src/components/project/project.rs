use stylist::{yew::styled_component };
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct ProjectProps{
    pub key:u16,
    pub image:String, 
    pub title: String,
    pub description: String, 
    pub tech_list: Vec<String>,
    pub year: String, 
    pub url: String, 
}

#[styled_component(Project)]
pub fn project(props: &ProjectProps) -> Html {
    // let button_stylesheet : StyleSource = css!(
    //     r#"
    //         padding-left : 15px;
    //         padding-right: 15px;
    //     "#
    // );
    html! {
        <>
                <div class="group relative">
                    <div class="min-h-80 aspect-w-1 aspect-h-1 w-full overflow-hidden rounded-md bg-gray-200 group-hover:opacity-75">
                        <img src={props.image.clone()} alt={props.title.clone()} class="h-full w-full object-left-top object-center"/>
                    </div>
                    <div class="mt-4 flex justify-between">
                    <div>
                        <h3 class="text-sm text-gray-700">
                        <a href={props.url.clone()}>
                            <span aria-hidden="true" class="absolute inset-0"></span>
                            {props.title.clone()}
                        </a>
                        </h3>
                        <p class="mt-1 text-sm text-gray-500">{props.description.clone()}</p>
                    </div>
                    <p class="text-sm font-medium text-gray-900">{props.year.clone()}</p>
                    </div>
                </div>
        </>
    }
}
