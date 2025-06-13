use chrono::{DateTime, Local};
use yew::prelude::*;

use crate::parse::ParsedOh;
use crate::utils::measure_time;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub oh: ParsedOh,
    pub dt: DateTime<Local>,
    pub set_raw_oh: Callback<String, ()>,
}

#[function_component]
pub fn Properties(props: &Props) -> Html {
    let loc = &props.oh.oh.get_context().locale;
    let dt = props.dt.with_timezone(loc.get_timezone());
    let normalized = props.oh.oh.normalize().to_string();

    let (time_eval, (state, comment, next_change_opt)) = measure_time(|| {
        let (state, comment) = props.oh.oh.state(dt);
        let next_change_opt = props.oh.oh.next_change(dt);
        (state, comment, next_change_opt)
    });

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
            <li>
              <strong>{"Comment: "}</strong>
              {comment}
            </li>
          }

          if normalized != *props.oh.raw {
            <li>
              <strong>{"Normalized: "}</strong>
              {normalized}
              <button onclick={normalize_onclick}>{"apply"}</button>
            </li>
          }

          <li>
            <strong>{"Time: "}</strong>
            {props.oh.time_parsing}
            {" (parsing) + "}
            {time_eval}
            {" (evaluating)"}
          </li>
        </ul>
      </section>
    }
}
