use chrono::{DateTime, Local};
use opening_hours::OpeningHours;
use opening_hours::localization::TzLocation;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub raw_oh: UseStateHandle<String>,
    pub oh: OpeningHours<TzLocation<chrono_tz::Tz>>,
    pub dt: DateTime<Local>,
    pub tz: chrono_tz::Tz,
}

#[function_component]
pub fn Properties(props: &Props) -> Html {
    let dt = props.dt.with_timezone(&props.tz);
    let normalized = props.oh.normalize().to_string();
    let is_normal = normalized == *props.raw_oh;
    let state = props.oh.state(dt);
    let next_change_opt = props.oh.next_change(dt);

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
          <li><strong>{"Comment: "}</strong> {"hide if no comment"}</li>
          <li>
            if is_normal {
              <strong>{"Already normalized"}</strong>
           }
            else {
              <strong>{"Normalized: "}</strong>
              {normalized}
           }
          </li>
        </ul>

        if !is_normal {
          <div class="button-box">
            <button>{"Apply Normalization"}</button>
          </div>
       }
      </section>
    }
}
