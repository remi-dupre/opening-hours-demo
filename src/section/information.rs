use yew::prelude::*;

const URL_REPO: &str = "https://github.com/remi-dupre/opening-hours-rs/";
const URL_WIKI: &str = "https://wiki.openstreetmap.org/wiki/Key:opening_hours/specification";

#[function_component]
pub fn Information() -> Html {
    let link_repo = html! {<a href={URL_REPO}>{"opening-hours-rs"}</a>};
    let link_wiki = html! {<a href={URL_WIKI}>{"here"}</a>};

    html! {
      <section>
        <h2>{"What is this?"}</h2>
        <p>
          {"This is a demo website for the Rust & Python library "}{link_repo}
          {". You can find documentation [here] and [here] (TODO)."}
        </p>
        <p>
          {"The syntax is documented on the OpenStreetMap wiki"}{link_wiki}{"."}
        </p>
      </section>
    }
}
