use stylist::yew::styled_component;
use yew::prelude::*;
use crate::components::button::button::Button;

#[styled_component(Profile)]
pub fn profile() -> Html {
    html! {
        <>
            <div class=" flex flex-nowrap box-border px-110px min-h-screen">
              <div class="min-w-1/2 bg-red-700	">
                <img
                  class="object-cover w-100 h-85 pl-50"
                  src="assets/me_and_la_pecou.png"
                  alt="picture of thomas ung"
                />
              </div>
              <div class="pt-63 ml-76 mr-24 w-1/2 h-full">
                <div class="pb-3">
                  <h1 class="font-bold">
                    {"Thomas UNG"}
                  </h1>
                  <h2 class="pb-3">{"Research Engineer"}</h2>
                  <p>
                    {"
                        Bonjour, bonjour ! I'm a french research engineer that works in the
                        robotic field. On the side, I like learning various stuff and doing
                        some sport. Feel free to give a look around the website for more
                        information
                    "}
                  </p>
                </div>
                <div>
                  <Button name={"ABOUT"} />
                </div>
              </div>
            </div>
        </>
    }
}
