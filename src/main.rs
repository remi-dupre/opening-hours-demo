use opening_hours::OpeningHours;
use yew::prelude::*;

pub mod components {
    pub mod normalized {
        use opening_hours::OpeningHours;
        use yew::prelude::*;

        #[derive(Properties, PartialEq)]
        pub struct Props {
            pub oh: UseStateHandle<OpeningHours>,
        }

        #[function_component]
        pub fn Normalized(props: &Props) -> Html {
            let stringified = props.oh.to_string();
            let normalized = props.oh.normalize().to_string();

            html! {
              <p>
                if normalized != stringified {
                  <strong>{"Normalized:"}</strong> {normalized}
                }
              </p>
            }
        }
    }
}

#[function_component]
fn App() -> Html {
    let oh = use_state(|| OpeningHours::parse("24/7").unwrap());

    let oninput = {
        let oh = oh.clone();

        move |input: InputEvent| {
            let raw_oh = input
                .target_dyn_into::<web_sys::HtmlInputElement>()
                .map(|el| el.value())
                .unwrap_or_default();

            match raw_oh.parse::<OpeningHours>() {
                Ok(parsed_oh) => {
                    log::debug!("New Expression: {}", *oh);
                    oh.set(parsed_oh);
                }
                Err(err) => {
                    log::debug!("Invalid Expression for {raw_oh:?}: {err}");
                }
            }
        }
    };

    html! {
        <div>
            <input {oninput} />
            <components::normalized::Normalized oh={oh} />
        </div>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
