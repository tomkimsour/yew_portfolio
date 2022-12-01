use yew::prelude::*;
use stylist::{yew::styled_component, StyleSource};

#[styled_component(ColorThemeButton)]
pub fn color_theme_button() -> Html {
    let floating_button_stylesheet : StyleSource = css!(
        r#"
            position: fixed;
            bottom: 7%;
            right: 5%;
        "#
    );
    html! {
        <>
        // <div class="relative h-32 w-32">
        //     <div class="absolute bottom-0 right-0 h-16 w-16">{"09"}</div>
        // </div>
        <div class={floating_button_stylesheet}>
            <button class="border-3 border-black text-black font-bold hover:bg-black hover:text-white py-2 px-7 rounded-full">
                {"Light"}
            </button>
        </div>
        </>
    }
}