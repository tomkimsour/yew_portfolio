use stylist::yew::styled_component;
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct Props;

#[styled_component(Footer)]
pub fn footer(props: &Props) -> Html {
    let mail = "thomas.ung.pro@gmail.com";
    // const copyToClipboard = () => {
    //   navigator.clipboard.writeText(mail);
    // };
    html! {
     <footer class="footer bg-black text-white px-110px">
      <div class="container">
        <div id="contact-me" class="flex-col py-20">
          <h1>{"Contact me"}</h1>
          <div id="email flex-row">
            <a
              href="mailto:thomas.ung.pro@gmail.com"
              class="text-22px flex"
            >
              {mail}
            </a>

            <div class="flex hover:op80">
              <button
                class="i-carbon-copy-file text-22px"
                // onClick={copyToClipboard}
              />
            </div>
          </div>
          <hr class="h-1px bg-white w-64" />
        </div>
        <div id="footer bottom row" class="flex-row">
          <div class="cr">
            {"Copyright 2022 © Designed by Cathy"}
          </div>
          <div id="socials">
            <ul class="list-none">
              <li class="flex justify-left hover:op80">
                <a
                  class="i-carbon-logo-github text-inherit"
                  href="https://github.com/tomkimsour"
                  target="_blank"
                />
              </li>
              <li class="flex justify-left hover:op80">
                <a
                  class="i-carbon-logo-linkedin text-inherit"
                  href="https://linkedin.com/in/thomas-ung"
                  target="_blank"
                />
              </li>
            </ul>
          </div>
        </div>
      </div>
    </footer>

    }
}
