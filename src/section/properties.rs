use std::ops::Deref;

use yew::prelude::*;

use crate::EvalContext;
use crate::utils::measure_time;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub ctx: UseStateHandle<EvalContext>,
}

#[function_component]
pub fn Properties(props: &Props) -> Html {
    let Some(oh) = props.ctx.oh() else {
        return html!();
    };

    let loc = &oh.get_context().locale;
    let dt = props.ctx.local_dt().with_timezone(loc.get_timezone());
    let pretified = oh.to_string();
    let normalized = oh.normalize().to_string();

    let (_time_eval, (state, comment, next_change_opt)) = measure_time(|| {
        let (state, comment) = oh.state(dt);
        let next_change_opt = oh.next_change(dt);
        (state, comment, next_change_opt)
    });

    let pretified_onclick = {
        let pretified = pretified.clone();
        let ctx = props.ctx.clone();
        move |_| ctx.set(ctx.deref().clone().with_raw_oh(pretified.clone()))
    };

    let normalize_onclick = {
        let normalized = normalized.clone();
        let ctx = props.ctx.clone();
        move |_| ctx.set(ctx.deref().clone().with_raw_oh(normalized.clone()))
    };

    let next_change = next_change_opt
        .map(|dt| dt.to_string())
        .unwrap_or_else(|| "never".to_string());

    html! {
      <>
        <table class="stripped">
        <tbody>
          <tr>
            <th>{"State"}</th>
            <td>{state.to_string()}</td>
          </tr>
          <tr>
            <th>{"Next change"}</th>
            <td>{next_change}</td>
          </tr>

          if !comment.is_empty() {
            <tr>
              <th>{"Comment: "}</th>
              <td>{comment}</td>
            </tr>
          }

          if pretified != props.ctx.raw_oh {
            <tr>
              <th>{"Pretified"}</th>
              <td>
                {pretified.clone()}
                {" "}
                <a onclick={pretified_onclick}>{"(apply)"}</a>
              </td>
            </tr>
          }

          if normalized != pretified {
            <tr>
              <th>{"Normalized"}</th>
              <td>
                {normalized}
                {" "}
                <a onclick={normalize_onclick}>{"(apply)"}</a>
              </td>
            </tr>
          }

          </tbody>
        </table>
      </>
    }
}
