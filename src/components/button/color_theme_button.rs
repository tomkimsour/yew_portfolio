use stylist::{yew::styled_component, StyleSource};
use yew::prelude::*;

#[styled_component(ColorThemeButton)]
pub fn color_theme_button() -> Html {
    /*
    // On page load or when changing themes, best to add inline in `head` to avoid FOUC
    if (localStorage.theme === 'dark' || (!('theme' in localStorage) && window.matchMedia('(prefers-color-scheme: dark)').matches)) {
    document.documentElement.classList.add('dark')
    } else {
    document.documentElement.classList.remove('dark')
    }

    // Whenever the user explicitly chooses light mode
    localStorage.theme = 'light'

    // Whenever the user explicitly chooses dark mode
    localStorage.theme = 'dark'

    // Whenever the user explicitly chooses to respect the OS preference
    localStorage.removeItem('theme')
    */
    let floating_button_stylesheet: StyleSource = css!(
        r#"
            position: fixed;
            bottom: 7%;
            right: 5%;
            z-index: 50;
        "#
    );
    html! {
        <>
        // <div class="relative h-32 w-32">
        //     <div class="absolute bottom-0 right-0 h-16 w-16">{"09"}</div>
        // </div>
        <div class={floating_button_stylesheet}>
            <button class="border border-black bg-white text-black hover:bg-black hover:text-white py-2 px-7 rounded-full">
                {"Dark"}
            </button>
        </div>
        </>
    }
}
