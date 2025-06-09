pub mod section;
pub mod utils;

use opening_hours::localization::TzLocation;
use opening_hours::{Context, OpeningHours};
use yew::prelude::*;

#[function_component]
fn App() -> Html {
    let raw_oh = use_state(String::new);
    let oh = use_state(|| None);
    let tz = use_state(|| chrono_tz::Europe::Paris);
    let dt = use_state(chrono::Local::now);

    let oninput = {
        let raw_oh_state = raw_oh.clone();
        let oh = oh.clone();
        let tz = tz.clone();

        move |input: InputEvent| {
            let raw_oh = input
                .target_dyn_into::<web_sys::HtmlTextAreaElement>()
                .map(|el| el.value())
                .unwrap_or_default();

            raw_oh_state.set(raw_oh.clone());

            match OpeningHours::parse(&raw_oh) {
                Ok(parsed_oh) => {
                    let ctx = Context::default().with_locale(TzLocation::new(*tz));
                    let parsed_oh = parsed_oh.with_context(ctx);
                    oh.set(Some(parsed_oh));
                }
                Err(_) => {
                    oh.set(None);
                }
            }
        }
    };

    html! {
      <>
        <section>
          <textarea id="expression" {oninput}></textarea>
        </section>

        if let Some(oh) = &*oh {
          <section::properties::Properties raw_oh={raw_oh} oh={oh.clone()} dt={*dt} tz={*tz} />
          <section::schedule::Schedule oh={(*oh).clone()} dt={*dt} tz={*tz} />
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
