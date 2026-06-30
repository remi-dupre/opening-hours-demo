use chrono::{NaiveDateTime, TimeDelta, TimeZone};
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::eval::{CallbackUpdateCtx, EvalContext};

const DATETIME_FORMAT: &str = "%Y-%m-%dT%H:%M";

#[derive(Properties, PartialEq)]
pub struct Props {
    pub ctx: UseStateHandle<EvalContext>,
    pub cb_update_ctx: CallbackUpdateCtx,
}

#[function_component]
pub fn ContextForm(props: &Props) -> Html {
    let auto_date = props.ctx.dt.is_none();
    let dt = props.ctx.local_dt();
    let dt_str = dt.format(DATETIME_FORMAT).to_string();

    let time_add = |delta| {
        let cb_update_ctx = props.cb_update_ctx.clone();

        move |_| {
            if let Some(new_dt) = dt.checked_add_signed(delta) {
                cb_update_ctx.emit(Box::new(move |ctx| ctx.with_dt(new_dt)))
            }
        }
    };

    let on_dt_input = use_callback(props.cb_update_ctx.clone(), {
        |input: InputEvent, cb_update_ctx| {
            let Some(el) = input.target_dyn_into::<HtmlInputElement>() else {
                log::error!("Could not fetch datetime input");
                return;
            };

            let new_dt_res = match NaiveDateTime::parse_from_str(&el.value(), DATETIME_FORMAT) {
                Ok(x) => chrono::Local.from_local_datetime(&x),
                Err(err) => {
                    log::error!("Could not parse datetime {}: {err}", el.value());
                    return;
                }
            };

            let Some(new_dt) = new_dt_res.single() else {
                log::error!("Ambiguous time for {}", el.value());
                return;
            };

            cb_update_ctx.emit(Box::new(move |ctx| ctx.with_dt(new_dt)))
        }
    });

    let on_checkbox_click = use_callback(
        props.cb_update_ctx.clone(),
        |input: InputEvent, cb_update_ctx| {
            let Some(el) = input.target_dyn_into::<HtmlInputElement>() else {
                log::error!("Could not fetch date checkbox");
                return;
            };

            let checked = el.checked();

            cb_update_ctx.emit(Box::new(move |ctx| {
                if checked {
                    ctx.with_no_dt()
                } else {
                    let now = ctx.local_dt();
                    ctx.with_dt(now)
                }
            }))
        },
    );

    html! {
      <details open={false}>
        <summary>
          <em>
            {"Evaluate from "}
            {dt.format(DATETIME_FORMAT)}
            {" in France "}
          </em>
        </summary>
        <article>
        <h4>{"Change current evaluation date"}</h4>
        <fieldset>
          <label>
            <input name="terms" type="checkbox" role="switch" checked={auto_date} oninput={on_checkbox_click} />
            {"Evaluate from current date"}
          </label>
        </fieldset>
        <fieldset role="group">
          <input type="submit" value="-1d" onclick={time_add(TimeDelta::days(-1))} />
          <input type="datetime-local" class="outline" value={dt_str} oninput={on_dt_input} />
          <input type="submit" value="+1d" onclick={time_add(TimeDelta::days(1))} />
        </fieldset>
        <h4>{"Change evaluation position"}</h4>
        <fieldset role="group">
          <input type="number" step="0.01" placeholder="Latitude" />
          <input type="number" step="0.01" placeholder="Longitude" />
        </fieldset>
        <p>
          {"Detected country: France. Detected timezone: Europe/Paris."}
        </p>
        </article>
      </details>
    }
}
