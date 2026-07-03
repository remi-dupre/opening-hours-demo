use chrono::{Datelike, NaiveDate};
use opening_hours::schedule::Schedule;
use opening_hours_syntax::ExtendedTime;
use yew::prelude::*;

use crate::component::icon::{Icon, Message};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub date: NaiveDate,
    pub schedule: Schedule,
}

#[function_component]
pub fn ScheduleLine(props: &Props) -> Html {
    let date = props.date.format("%d/%m").to_string();
    let mut wday = props.date.weekday().to_string();
    wday.truncate(2);

    let intervals: Vec<_> = (props.schedule.clone())
        .into_iter()
        .map(|interval| {
            let (kind, comment) = interval.as_state();
            let class = ["interval", kind.as_str()];

            let duration_minutes =
                interval.range.end.mins_from_midnight() - interval.range.start.mins_from_midnight();

            let width = 100.0 * f32::from(duration_minutes)
                / f32::from(ExtendedTime::MIDNIGHT_24.mins_from_midnight());

            let mut tooltip = format!("{}-{} {kind}", interval.range.start, interval.range.end);

            if !comment.is_empty() {
                tooltip.push_str(": ");
                tooltip.push_str(comment);
            }

            html! {
                <div
                  {class}
                  data-tooltip={tooltip}
                  style={format!("width: {width}%")}
                >
                  if !comment.is_empty() {
                    <Icon src={Message} />
                  }
                </div>
            }
        })
        .collect();

    html! {
      <tr>
        <td>{wday}</td>
        <td>
          <div class="bar">
            {intervals}
          </div>
        </td>
        <td>{date}</td>
      </tr>
    }
}
