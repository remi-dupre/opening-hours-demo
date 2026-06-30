use yew::prelude::*;

use crate::{
    component::icon::{GitHub, Icon},
    links::{URL_JS_LIB, URL_REPO, URL_WIKI, URL_YEW},
};

#[function_component]
pub fn Information() -> Html {
    html! {
      <>
        <h3>{"What is this?"}</h3>
        <p>
          {"This is a demo website for the Rust & Python library "}
          <a href={URL_REPO}>{"opening-hours-rs"}</a>
          {". All computations are performed on your device, thanks to Rust and "}
          <a href={URL_YEW}>{"Yew"}</a>
          {"."}
        </p>
        <p>
          {"The syntax is documented on the OpenStreetMap wiki "}
          <Icon src={GitHub} />
          {" "}
          <a href={URL_WIKI}>{"OpenStreetMap's wiki"}</a>
          {". You may also be interested in the reference "}
          <a href={URL_JS_LIB}>{"JavaScript library"}</a>
          {"."}
        </p>
      </>
    }
}
