use chrono::{DateTime, Local};
use opening_hours::OpeningHours;
use opening_hours::localization::TzLocation;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub raw_oh: String,
    pub oh: OpeningHours<TzLocation<chrono_tz::Tz>>,
    pub dt: DateTime<Local>,
    pub set_raw_oh: Callback<String, ()>,
}

#[function_component]
pub fn Properties(props: &Props) -> Html {
    let loc = &props.oh.get_context().locale;
    let dt = props.dt.with_timezone(loc.get_timezone());
    let normalized = props.oh.normalize().to_string();
    let (state, comment) = props.oh.state(dt);
    let next_change_opt = props.oh.next_change(dt);

    let normalize_onclick = {
        let normalized = normalized.clone();
        let set_raw_oh = props.set_raw_oh.clone();
        move |_| set_raw_oh.emit(normalized.clone())
    };

    html! {
      <section>
        <ul>
          <li><strong>{"State: "}</strong> {state}</li>
          <li>
            <strong>{"Next change: "}</strong>

            if let Some(next_change) = next_change_opt {
              {next_change}
            } else {
              {"never"}
            }
          </li>

          if !comment.is_empty() {
            <li><strong>{"Comment: "}</strong> {comment}</li>
          }

          if normalized != *props.raw_oh {
            <li>
              <strong>{"Normalized: "}</strong>
              {normalized}
              <button onclick={normalize_onclick}>{"apply"}</button>
            </li>
          }
        </ul>
      </section>
    }
}
