use yew::prelude::*;

#[function_component(Profile)]
pub fn profile() -> Html {
    html! {
        <>
            <div class=" flex flex-nowrap flex-col lg:flex-row box-border px-110px lg:min-h-screen">
              <div class="w-full lg:w-6/12 h-full">
                <div id="profile-image-wrapper" class="overflow-hidden pt-32 px-12">
                  <img
                    class="object-cover"
                    src="assets/IMG_20230429_112845__01.jpeg"
                    alt="picture of thomas ung"
                  />
                </div>
              </div>
              <div class="min-w-full lg:min-w-min  lg:w-6/12 h-full px-16 lg:px-0 py-20 lg:py-0">
                <div class="lg:pt-40 lg:pl-4">
                  <div class="">
                    <h1 class="font-bold">
                      {"Thomas UNG"}
                    </h1>
                    <p class="opacity-70">
                      {"
                          Bonjour, bonjour! I'm a French research software engineer working in the field of
                          robotics. On the side, I like to learn different things and do 
                          some sports. Feel free to browse the website for more
                          information.
                      "}
                    </p>
                  </div>
                </div>
              </div>
            </div>
        </>
    }
}
