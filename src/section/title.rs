use chrono::Datelike;
use yew::prelude::*;

use crate::EvalContext;
use crate::utils::capitalize;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub ctx: UseStateHandle<EvalContext>,
}

#[function_component]
pub fn Title(props: &Props) -> Html {
    let Some(oh) = props.ctx.oh() else {
        return html!();
    };

    let loc = &oh.get_context().locale;
    let dt = props.ctx.dt().with_timezone(loc.get_timezone());
    let next_change_opt = oh.next_change(dt);
    let (state, _comment) = oh.state(dt);

    let title = {
        if let Some(next_change) = next_change_opt {
            let readable_dt = {
                if next_change.date_naive() == dt.date_naive() {
                    next_change.format("%H:%M")
                } else if next_change.year() == dt.year() {
                    next_change.format("%B %d, %H:%M")
                } else {
                    next_change.format("%B %d %Y, %H:%M")
                }
            };

            format!("{} until {readable_dt}", capitalize(&state.to_string()))
        } else {
            format!("Always {state}")
        }
    };

    html! {
      <h3>{title}</h3>
    }
}
