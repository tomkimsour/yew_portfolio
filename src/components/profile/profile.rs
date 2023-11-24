use yew::prelude::*;

#[function_component(Profile)]
pub fn profile() -> Html {
    html! {
        <>
            <div class=" flex flex-nowrap box-border px-110px min-h-screen">
              <div class="w-6/12 h-full">
                <div id="profile-image-wrapper" class="overflow-hidden pt-32 px-12">
                  <img
                    class="object-cover"
                    src="assets/IMG_20230429_112845__01.jpeg"
                    alt="picture of thomas ung"
                  />
                </div>
              </div>
              <div class="w-6/12 h-full">
                <div class="pt-40 pl-4">
                  <div class="">
                    <h1 class="font-bold">
                      {"Thomas UNG"}
                    </h1>
                    // <h2 class="pb-3">{"Research Engineer"}</h2>
                    <p class="opacity-70">
                      {"
                          Bonjour, bonjour! I'm a French research software engineer working in the field of
                          robotics. On the side, I like to learn different things and do 
                          some sports. Feel free to browse around the website for more
                          more information.
                      "}
                    </p>
                  </div>
                </div>
              </div>
            </div>
        </>
    }
}
