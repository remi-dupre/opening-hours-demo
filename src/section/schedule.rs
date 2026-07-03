use std::collections::{HashMap, HashSet};

use opening_hours::RuleKind;
use opening_hours_syntax::ExtendedTime;
use yew::prelude::*;

use crate::EvalContext;
use crate::component::schedule_line::ScheduleLine;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub ctx: UseStateHandle<EvalContext>,
}

#[function_component]
pub fn Schedule(props: &Props) -> Html {
    let Some(oh) = props.ctx.get_oh() else {
        return html!();
    };

    let dt = props.ctx.get_local_dt();

    let dates = (0..7).filter_map(|idx| dt.date_naive().checked_add_days(chrono::Days::new(idx)));
    let schedules: Vec<_> = dates.clone().map(|date| oh.schedule_at(date)).collect();

    let relevant_hours = filter_relevant_hours(schedules.iter().flat_map(|schedule| {
        (schedule.clone())
            .into_iter()
            .filter(|tr| tr.kind != RuleKind::Closed)
            .flat_map(|tr| [tr.range.start, tr.range.end])
    }));

    let days: Vec<_> = dates
        .zip(schedules)
        .map(|(date, schedule)| {
            html! {
              <ScheduleLine {date} {schedule} />
            }
        })
        .collect();

    html! {
      <table class="schedule">
        {draw_header(&relevant_hours)}
        <tbody >
          {days}
        </tbody>
      </table>
    }
}

// --
// -- Draw parts
// --

fn draw_header(relevant_hours: &[u8]) -> Html {
    let cols: Vec<_> = (0..=24)
        .map(|h| {
            let text = {
                if relevant_hours.contains(&h) {
                    format!("{h:02}")
                } else {
                    String::new()
                }
            };

            html! {
              <div class="hour"><small>{text}</small></div>
            }
        })
        .collect();

    html! {
      <thead>
        <tr>
          <td />
          <td>{cols}</td>
          <td />
        </tr>
      </thead>
    }
}

// --
// -- Utils
// --

fn filter_relevant_hours(times: impl Iterator<Item = ExtendedTime>) -> Vec<u8> {
    let mut count_hours = HashMap::new();

    for time in times {
        let hour = {
            if time.minute() <= 30 {
                time.hour()
            } else {
                time.hour() + 1
            }
        };

        *count_hours.entry(hour).or_insert(0) += 1;
    }

    let mut count_hours: Vec<_> = count_hours
        .into_iter()
        .map(|(hour, count)| (count, hour))
        .collect();

    count_hours.sort_by_key(|(count, _)| -count);
    let mut result = HashSet::new();

    for (_, hour) in count_hours {
        if !result.contains(&hour.wrapping_sub(1)) && !result.contains(&(hour + 1)) {
            result.insert(hour);
        }
    }

    let mut result: Vec<_> = result.into_iter().collect();
    result.sort_unstable();
    result
}
