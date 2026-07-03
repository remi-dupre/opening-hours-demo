use chrono::{DateTime, Local, NaiveDateTime, TimeDelta, TimeZone};
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::context::CallbackUpdateCtx;

const DATETIME_FORMAT: &str = "%Y-%m-%dT%H:%M";

#[derive(Properties, PartialEq)]
pub struct Props {
    pub cb_update_ctx: CallbackUpdateCtx,
    pub auto_date: bool,
    pub dt: DateTime<Local>,
}

#[function_component]
pub fn DatePicker(props: &Props) -> Html {
    let dt_str = props.dt.format(DATETIME_FORMAT).to_string();

    let time_add = |delta| {
        let dt = props.dt;
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
                    let now = ctx.get_local_dt();
                    ctx.with_dt(now)
                }
            }))
        },
    );

    html! {
      <>
        <fieldset>
          <label>
            <input name="terms" type="checkbox" role="switch" checked={props.auto_date} oninput={on_checkbox_click} />
            {"Evaluate from current date"}
          </label>
        </fieldset>
        <fieldset role="group">
          <input type="submit" value="-1d" onclick={time_add(TimeDelta::days(-1))} />
          <input type="datetime-local" class="outline" value={dt_str} oninput={on_dt_input} />
          <input type="submit" value="+1d" onclick={time_add(TimeDelta::days(1))} />
        </fieldset>
      </>
    }
}
