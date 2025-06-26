pub mod component;
pub mod links;
pub mod parse;
pub mod section;
pub mod utils;

use web_sys::HtmlTextAreaElement;
use yew::prelude::*;

use crate::component::icon::{ArrowDropdown, Icon};
use crate::parse::ParsedOh;

#[function_component]
fn App() -> Html {
    let expression_ref = use_node_ref();
    let oh_state = use_state(|| None);
    let dt_state = use_state(chrono::Local::now);
    let coord_popup_state = use_state(|| false);
    let date_popup_state = use_state(|| false);

    let coord_onclick = use_callback(coord_popup_state.clone(), |_, date_popup_state| {
        date_popup_state.set(!**date_popup_state);
    });

    let date_onclick = use_callback(date_popup_state.clone(), |_, date_popup_state| {
        date_popup_state.set(!**date_popup_state);
    });

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
            <a class={"set-location"} onclick={coord_onclick.clone()}>
              {"🇫🇷 Paris, France"}
              <Icon src={ArrowDropdown} />
            </a>

            if *coord_popup_state {
              <component::popup::Popup onclick_outside={coord_onclick}>
                <form>
                  <div class="form-row">
                    <div class="form-header">{"Coordinates"}</div>
                    <div>
                      <input type="number" value="48.86" step="0.01" min="-90" max="90" class="half-input" />
                      <input type="number" value="2.34" step="0.01" min="-180" max="180" class="half-input" />
                      <button>{"My position"}</button>
                    </div>
                  </div>
                  <div class="form-row">
                    <div class="form-header">{"TimeZone"}</div>
                    <div>{"Europe/Paris"}</div>
                  </div>
                  <div class="form-row">
                    <div class="form-header">{"Country"}</div>
                    <div>{"France"}<br />{"12,000 holidays"}</div>
                  </div>
                </form>
              </component::popup::Popup>
            }

            <a class={"set-timer right"} onclick={date_onclick.clone()}>
              <Icon src={ArrowDropdown} />
              {"08/06/2025 10:54 🕐"}
            </a>

            if *date_popup_state {
              <component::popup::Popup class={"right"} onclick_outside={date_onclick}>
                <form>
                  <div class="form-row">
                    <div class="form-header">{"Date & Time"}</div>
                    <div>
                      <input type="date" value="2025-05-12" />
                      <input type="time" value="13:45" />
                      <button>{"Now"}</button>
                    </div>
                  </div>
                </form>
              </component::popup::Popup>
            }
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
        <section::footer::Footer />
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
