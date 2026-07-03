use yew::prelude::*;

use crate::component::coords_picker::CoordsPicker;
use crate::component::date_picker::DatePicker;
use crate::eval::{CallbackUpdateCtx, EvalContext};

const DATETIME_FORMAT: &str = "%Y-%m-%d %H:%M";

#[derive(Properties, PartialEq)]
pub struct Props {
    pub ctx: UseStateHandle<EvalContext>,
    pub cb_update_ctx: CallbackUpdateCtx,
}

#[function_component]
pub fn ContextForm(props: &Props) -> Html {
    let auto_date = props.ctx.dt.is_none();
    let dt = props.ctx.get_local_dt();
    let coords = props.ctx.get_coords();
    let country = props.ctx.get_country();
    let timezone = *props.ctx.get_location().get_timezone();

    html! {
      <details open={false}>
        <summary>
          <em>
            {"Evaluate from "}
            {dt.format(DATETIME_FORMAT)}

            if let Some(country) = country {
                {" in "}
                {country.to_string()}
            }
          </em>
        </summary>
        <article>
          <h3>{"Change current evaluation date"}</h3>
          <DatePicker cb_update_ctx={props.cb_update_ctx.clone()} {auto_date} {dt} />
          <h3>{"Change evaluation position"}</h3>
          <CoordsPicker cb_update_ctx={props.cb_update_ctx.clone()} {coords} {country} {timezone} />
        </article>
      </details>
    }
}
