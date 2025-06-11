pub mod component;
pub mod section;
pub mod utils;

use opening_hours::localization::{Coordinates, Country, TzLocation};
use opening_hours::{Context, OpeningHours};
use web_sys::HtmlTextAreaElement;
use yew::prelude::*;

#[function_component]
fn App() -> Html {
    let expression_ref = use_node_ref();
    let raw_oh_state = use_state(String::default);
    let oh_state = use_state(|| None);
    let dt_state = use_state(chrono::Local::now);

    let set_raw_oh = use_callback((), {
        let expression_ref = expression_ref.clone();
        let raw_oh_state = raw_oh_state.clone();
        let oh_state = oh_state.clone();

        move |value: String, _| {
            match OpeningHours::parse(&value) {
                Ok(parsed_oh) => {
                    let loc = TzLocation::new(chrono_tz::Europe::Paris)
                        .with_coords(Coordinates::new(48.85, 2.35).unwrap());

                    let ctx = Context::default()
                        .with_holidays(Country::US.holidays())
                        .with_locale(loc);

                    let parsed_oh = parsed_oh.with_context(ctx);
                    oh_state.set(Some(parsed_oh));
                }
                Err(_) => {
                    oh_state.set(None);
                }
            }

            if let Some(el) = expression_ref.cast::<HtmlTextAreaElement>() {
                if el.value() != value {
                    el.set_value(&value);
                }
            } else {
                log::warn!("Could not update text area content")
            }

            raw_oh_state.set(value);
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


        if raw_oh_state.is_empty() {
          <section::examples::Examples set_raw_oh={set_raw_oh.clone()} />
        }

        if let Some(oh) = &*oh_state {
          <section::properties::Properties
            raw_oh={raw_oh_state.to_string()}
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
