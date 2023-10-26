use stylist::yew::styled_component;
use yew_hooks::prelude::*;
use yew::prelude::*;

#[styled_component(Footer)]
pub fn footer() -> Html {
    let mail = "thomas.ung.pro@gmail.com";
    // const copyToClipboard = () => {
    //   navigator.clipboard.writeText(mail);
    // };
    let clipboard = use_clipboard();

    let onclick_write_text = {
      let clipboard = clipboard.clone();
      Callback::from(move |_| {
          clipboard.write_text(mail.to_owned());
      })
  };
  let onclick_read_text = {
      let clipboard = clipboard.clone();
      Callback::from(move |_| {
          clipboard.read_text();
      })
  };

    html! {
     <footer id="contact" class="footer bg-black text-white px-110px px-24">
      <div class="container">
        <div>
            <button onclick={onclick_write_text}>{ "Write text to clipboard" }</button>
            <button onclick={onclick_read_text}>{ "Read text from clipboard" }</button>
            <p>{ format!("Current text: {:?}", *clipboard.text) }</p>
            <p>{ format!("Copied: {:?}", *clipboard.copied) }</p>
            <p>{ format!("Is supported: {:?}", *clipboard.is_supported) }</p>
        </div>
        <div id="contact-me" class="flex-col py-20">
          <h1>{"Contact me"}</h1>
          <div id="email flex-row">
            <a
              href="mailto:thomas.ung.pro@gmail.com"
              class="text-22px flex"
              // onclick={onclick_write_text}
            >
              {mail}
            </a>

            <div class="flex hover:op80">
              <button
                class=" text-22px"
                // onClick={copyToClipboard}
              >
                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6">
                  <path stroke-linecap="round" stroke-linejoin="round" d="M15.75 17.25v3.375c0 .621-.504 1.125-1.125 1.125h-9.75a1.125 1.125 0 01-1.125-1.125V7.875c0-.621.504-1.125 1.125-1.125H6.75a9.06 9.06 0 011.5.124m7.5 10.376h3.375c.621 0 1.125-.504 1.125-1.125V11.25c0-4.46-3.243-8.161-7.5-8.876a9.06 9.06 0 00-1.5-.124H9.375c-.621 0-1.125.504-1.125 1.125v3.5m7.5 10.375H9.375a1.125 1.125 0 01-1.125-1.125v-9.25m12 6.625v-1.875a3.375 3.375 0 00-3.375-3.375h-1.5a1.125 1.125 0 01-1.125-1.125v-1.5a3.375 3.375 0 00-3.375-3.375H9.75" />
                </svg>
              </button>
            </div>
          </div>
          <hr class="h-1px bg-white w-64" />
        </div>
        <div id="footer bottom row" class="flex-row">
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
