use yew::prelude::*;

use crate::links::{URL_JS_LIB, URL_REPO, URL_WIKI, URL_YEW};

#[function_component]
pub fn Information() -> Html {
    let link_js_lib = html! {<a href={URL_JS_LIB}>{"JavaScript library"}</a>};
    let link_repo = html! {<a href={URL_REPO}>{"opening-hours-rs"}</a>};
    let link_wiki = html! {<a href={URL_WIKI}>{"OpenStreetMap's wiki"}</a>};
    let link_yew = html! {<a href={URL_YEW}>{"Yew"}</a>};

    html! {
      <section>
        <h2>{"What is this?"}</h2>
        <p>
          {"This is a demo website for the Rust & Python library "}{link_repo}
          {". All computations are performed on your device, thanks to Rust and "}
          {link_yew}{"."}
        </p>
        <p>
          {"The syntax is documented on the OpenStreetMap wiki "}{link_wiki}
          {". You may also be interested in the reference "}{link_js_lib}{"."}
        </p>
      </section>
    }
}
