use yew::prelude::*;

use crate::eval::CallbackUpdateCtx;

const EXPRESSIONS: &[&str] = &[
    "Mo-Fr 10:00-20:00; PH off",
    "24/7",
    "22:00+; PH off",
    "Fr-Sa 18:00-06:00; PH off",
    "Mo 10:00-12:00,12:30-15:00",
    "Mo,Tu,Th,Fr 12:00-18:00; Sa,PH 12:00-17:00; Th[3],Th[-1] off",
    "Mo-Fr 08:00-11:00 || Tu-Th,PH open \"Emergency only\"",
    "Apr-Oct Su[2] 14:00-18:00; Aug Su[-1] -1 day 10:00-18:00; PH off",
];

#[derive(Properties, PartialEq)]
pub struct Props {
    pub cb_update_ctx: CallbackUpdateCtx,
}

#[function_component]
pub fn Examples(props: &Props) -> Html {
    let onclick_set = move |raw: &'static str| {
        let cb_update_ctx = props.cb_update_ctx.clone();
        move |_| cb_update_ctx.emit(Box::new(|ctx| ctx.with_raw_oh(raw.to_string())))
    };

    html! {
      <article>
        <header>
          {"Examples"}
        </header>
        <ul>
          {
            EXPRESSIONS.iter().map(|expr| {
              html!{
                <li>
                  <a onclick={onclick_set(expr)}>{expr}</a>
                </li>
              }
            }).collect::<Html>()
          }
        </ul>
      </article>
    }
}
