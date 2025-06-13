pub mod component;
pub mod parse;
pub mod section;
pub mod utils;

use web_sys::HtmlTextAreaElement;
use yew::prelude::*;

use self::parse::ParsedOh;

#[function_component]
fn App() -> Html {
    let expression_ref = use_node_ref();
    let oh_state = use_state(|| None);
    let dt_state = use_state(chrono::Local::now);

    let set_raw_oh = use_callback((), {
        let expression_ref = expression_ref.clone();
        let oh_state = oh_state.clone();

        move |value: String, _| {
            if let Some(el) = expression_ref.cast::<HtmlTextAreaElement>() {
                if el.value() != value {
                    el.set_value(&value);
                }
            } else {
                log::warn!("Could not update text area content")
            }

            let parsed = ParsedOh::new(value.clone());
            oh_state.set(parsed.ok());
        }
    });

    html! {
      <>
        <section>
          <component::expression::Expression
            expression_ref={expression_ref}
            set_raw_oh={set_raw_oh.clone()}
          />

          <div class="expression-settings">
            <a class={"set-location"}>{"🇫🇷 Paris, France"}</a>
            <a class={"set-time"}>{"08/06/2025 10:54 🕐"}</a>
          </div>
        </section>


        if oh_state.is_none() {
          <section::examples::Examples set_raw_oh={set_raw_oh.clone()} />
        }

        if let Some(oh) = &*oh_state {
          <section::properties::Properties
            oh={oh.clone()}
            dt={*dt_state}
            set_raw_oh={set_raw_oh}
          />

          <section::schedule::Schedule oh={oh.clone()} dt={*dt_state} />
        }

        <section::information::Information />
      </>
    }
}

fn main() {
    let log_level = {
        if cfg!(debug_assertions) {
            log::Level::Debug
        } else {
            log::Level::Info
        }
    };

    wasm_logger::init(wasm_logger::Config::new(log_level));
    yew::Renderer::<App>::new().render();
}
