use opening_hours::localization::TzLocation;
use opening_hours::{Context, OpeningHours};
use yew::prelude::*;

pub mod components {
    pub mod properties {
        use chrono::{DateTime, Local};
        use opening_hours::OpeningHours;
        use opening_hours::localization::TzLocation;
        use yew::prelude::*;

        #[derive(Properties, PartialEq)]
        pub struct Props {
            pub raw_oh: UseStateHandle<String>,
            pub oh: UseStateHandle<Option<OpeningHours<TzLocation<chrono_tz::Tz>>>>,
            pub dt: UseStateHandle<DateTime<Local>>,
            pub tz: UseStateHandle<chrono_tz::Tz>,
        }

        #[function_component]
        pub fn Properties(props: &Props) -> Html {
            let Some(oh) = props.oh.as_ref() else {
                return Html::default();
            };

            let dt = props.dt.with_timezone(&*props.tz);
            let normalized = oh.normalize().to_string();
            let is_normal = normalized == *props.raw_oh;
            let state = oh.state(dt);
            let next_change_opt = oh.next_change(dt);

            html! {
              <>
                <ul>
                  <li><strong>{ "State: " }</strong> {state}</li>
                  <li>
                    <strong>{ "Next change: " }</strong>

                    if let Some(next_change) = next_change_opt {
                      {next_change}
                    } else {
                      { "never" }
                    }
                  </li>
                  <li><strong>{ "Comment: " }</strong> { "hide if no comment" }</li>
                  <li>
                    if is_normal {
                      <strong>{ "Already normalized" }</strong>
                    }
                    else {
                      <strong>{ "Normalized: " }</strong>
                      { normalized }
                    }
                  </li>
                </ul>

                if !is_normal {
                  <div class="button-box">
                    <button>{ "Apply Normalization" }</button>
                  </div>
                }
              </>
            }
        }
    }
}

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
            log::debug!("{input:?}");

            let raw_oh = input
                .target_dyn_into::<web_sys::HtmlTextAreaElement>()
                .map(|el| el.value())
                .unwrap_or_default();

            raw_oh_state.set(raw_oh.clone());

            match raw_oh.parse::<OpeningHours>() {
                Ok(parsed_oh) => {
                    let parsed_oh = parsed_oh
                        .with_context(Context::default().with_locale(TzLocation::new(*tz)));

                    log::debug!("New Expression: {}", parsed_oh);
                    oh.set(Some(parsed_oh));
                }
                Err(err) => {
                    log::debug!("Invalid Expression for {raw_oh:?}: {err}");
                    oh.set(None);
                }
            }
        }
    };

    let repo_url = "https://github.com/remi-dupre/opening-hours-rs/";

    html! {
      <>
        <section>
          <textarea id="expression" {oninput}></textarea>
        </section>
        <section>
          <components::properties::Properties raw_oh={raw_oh} oh={oh} dt={dt} tz={tz} />
        </section>
        <section>
          <h2>{ "Open until 18:00" }</h2>
        </section>
        <section>
          <h2>{ "What is this?" }</h2>
          <p>
            { "This is a demo website for the Rust & Python library " }
            <a href={ repo_url }>{ "opening-hours-rs" }</a>
            { ". You can find documentation [here] and [here]." }
          </p>
          <p>{ "The syntax is documented on the OpenStreetMap wiki [here]." }</p>
        </section>
      </>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
