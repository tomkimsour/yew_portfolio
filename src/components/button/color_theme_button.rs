use stylist::yew::styled_component;
use web_sys::window;
use yew::prelude::*;

#[derive(Copy, Clone)]
struct DarkMode {
    is_dark: bool,
}

impl DarkMode {
    fn new() -> Self {
        Self { is_dark: true }
    }

    fn is_dark(&self) -> bool {
        self.is_dark
    }

    fn set_to_browser_setting(mut self) -> Self {
        self.is_dark = window()
            .expect("window is undefined")
            .match_media("(prefers-color-scheme: dark)")
            .expect("media query is undefined")
            .unwrap()
            .matches();
        self
    }
}

#[styled_component(ColorThemeButton)]
pub fn color_theme_button() -> Html {
    let dark_mode = use_state(|| {
        let dark_mode = DarkMode::new().set_to_browser_setting();
        dark_mode
    });
    let on_click_toggle_dark_mode = {
        let dark_mode = dark_mode.clone();
        Callback::from(move |_| {
            let window = web_sys::window().expect("no global `window` exists");
            let document = window.document().expect("should have a document on window");
            let root = document
                .document_element()
                .expect("document should have a root element");

            if dark_mode.is_dark() {
                root.set_attribute("data-theme", "light")
                    .expect("Failed to set attribute");
            } else {
                root.set_attribute("data-theme", "dark")
                    .expect("Failed to set attribute");
            }
            dark_mode.set(DarkMode {
                is_dark: !dark_mode.is_dark(),
            })

            // @ts-expect-error: Transition API
            // let transition = document.start_view_transition(|| {
            //     dark_mode.set(DarkMode {
            //         is_dark: !dark_mode.is_dark(),
            //     });
            // });
            // const transition = document.startViewTransition(async () => {
            //     isDark.value = !isDark.value
            //     await nextTick()
            // })
            // transition.ready
            //     .then(() => {
            //     const clipPath = [
            //         `circle(0px at ${x}px ${y}px)`,
            //         `circle(${endRadius}px at ${x}px ${y}px)`,
            //     ]
            //     document.documentElement.animate(
            //         {
            //         clipPath: isDark.value
            //             ? [...clipPath].reverse()
            //             : clipPath,
            //         },
            //         {
            //         duration: 400,
            //         easing: 'ease-out',
            //         pseudoElement: isDark.value
            //             ? '::view-transition-old(root)'
            //             : '::view-transition-new(root)',
            //         },
            //     )
            //     })
            // dark_mode
        })
    };

    html! {
        <>
        <div class="hidden lg:flex fixed z-50 bottom-0 right-0 pr-24 pb-16">
                <button onclick={on_click_toggle_dark_mode} class="border border-content bg-transparent text-content hover:bg-content hover:text-bkg py-1 px-7 rounded-full">
                    if dark_mode.is_dark() {
                        {"Light"}
                    } else {
                        {"Dark"}
                    }
                </button>
        </div>
        </>
    }
}
