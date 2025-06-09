use std::collections::{HashMap, HashSet};

use crate::utils::capitalize;
use chrono::{DateTime, Datelike, Local, Timelike};
use chrono_tz::Tz;
use opening_hours::localization::TzLocation;
use opening_hours::{OpeningHours, RuleKind};
use opening_hours_syntax::ExtendedTime;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub oh: OpeningHours<TzLocation<Tz>>,
    pub dt: DateTime<Local>,
    pub tz: Tz,
}

#[function_component]
pub fn Schedule(props: &Props) -> Html {
    let dt = props.dt.with_timezone(&props.tz);
    let next_change_opt = props.oh.next_change(dt);
    let state = props.oh.state(dt);

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
      <section>
        <h2>{title}</h2>
        {draw_schedule_svg(&props.oh, dt)}
      </section>
    }
}

fn get_relevant_hours(times: impl Iterator<Item = ExtendedTime>) -> Vec<u8> {
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
        if hour == 0 || !result.contains(&(hour - 1)) {
            result.insert(hour);
        }
    }

    let mut result: Vec<_> = result.into_iter().collect();
    result.sort_unstable();
    result
}

fn draw_schedule_svg(oh: &OpeningHours<TzLocation<Tz>>, dt: DateTime<Tz>) -> Html {
    const HEADER_HEIGHT: f32 = 30.0;
    const WDAY_WIDTH: f32 = 30.0;
    const DAY_HEIGHT: f32 = 25.0;
    const DAY_WIDTH: f32 = 300.0;
    const DAY_GAP: f32 = 8.0;
    let y_pos = |idx: usize| HEADER_HEIGHT + idx as f32 * (DAY_HEIGHT + DAY_GAP);

    let days: Vec<_> = (0..7)
        .filter_map(|idx| dt.date_naive().checked_add_days(chrono::Days::new(idx)))
        .collect();

    let schedules: Vec<_> = days.iter().map(|day| oh.schedule_at(*day)).collect();

    let relevant_hours = get_relevant_hours(schedules.iter().flat_map(|schedule| {
        schedule
            .clone()
            .into_iter()
            .filter(|tr| tr.kind != RuleKind::Closed)
            .flat_map(|tr| [tr.range.start, tr.range.end])
    }));

    let needle_minutes = dt.time().hour() * 60 + dt.time().minute();
    let needle_x = WDAY_WIDTH + DAY_WIDTH * needle_minutes as f32 / (24. * 60.);

    html! {
      <svg
        class="schedule"
        viewBox="0 0 389.05 270"
        xmlns="http://www.w3.org/2000/svg"
      >
        {
          // States
          schedules.iter().enumerate().flat_map(|(idx, schedule)| {
            schedule.clone().into_iter()
              .filter(|tr| tr.kind != RuleKind::Closed)
              .map(move |tr| {
                let mins_x = tr.range.start.mins_from_midnight() as f32;
                let mins_w = tr.range.end.mins_from_midnight() as f32 - mins_x;
                let x = WDAY_WIDTH + DAY_WIDTH * mins_x / (24.0 * 60.0);
                let width = DAY_WIDTH * mins_w / (24.0 * 60.0);

                html!{
                  <rect
                    class={format!("state-{}", tr.kind)}
                    x={x.to_string()}
                    y={y_pos(idx).to_string()}
                    width={width.to_string()}
                    height={DAY_HEIGHT.to_string()}
                  />
                }
            })
          }).collect::<Html>()
        }

        {
          // Day lines
          schedules.iter().enumerate().map(|(idx, _)| {
            html!{
              <rect
                class="dayline"
                x={WDAY_WIDTH.to_string()}
                y={y_pos(idx).to_string()}
                width={DAY_WIDTH.to_string()}
                height={DAY_HEIGHT.to_string()}
                rx="2"
              />
            }
          }).collect::<Html>()
        }

        {
          // Week days
          days.iter().enumerate().map(|(idx, day)| {
            let y = y_pos(idx) + 0.5 * DAY_HEIGHT;
            let day = day.format("%A").to_string();

            html!{
              <text x="0" y={y.to_string()}>{&day[0..2]}</text>
            }
          }).collect::<Html>()
        }

        {
          // Dates
          days.iter().enumerate().map(|(idx, day)| {
            let x = WDAY_WIDTH + DAY_WIDTH + 10.0;
            let y = y_pos(idx) + 0.5 * DAY_HEIGHT;
            let day = day.format("%d/%m").to_string();

            html!{
              <text x={x.to_string()} y={y.to_string()}>{day}</text>
            }
          }).collect::<Html>()
        }

        {
          // Hours
          relevant_hours.iter().map(|hour| {
            let x = WDAY_WIDTH + DAY_WIDTH * f32::from(*hour) / 24.0;

            html!{
              <text class="hour" x={x.to_string()} y="10">{hour}</text>
            }
          }).collect::<Html>()
        }

        {
          // Hour guides
          relevant_hours
            .iter()
            .filter(|hour| **hour != 0 && **hour != 24)
            .map(|hour| {
              let x = WDAY_WIDTH + DAY_WIDTH * f32::from(*hour) / 24.0;

              html!{
                <line class="hour-guide" x1={x.to_string()} x2={x.to_string()} y1="20" y2="270" />
              }
            }).collect::<Html>()
        }

        // Time Needle
        <line
          class="time-needle"
          x1={needle_x.to_string()}
          x2={needle_x.to_string()}
          y1={y_pos(0).to_string()}
          y2={(y_pos(0) + DAY_HEIGHT).to_string()}
        />
      </svg>
    }
}
