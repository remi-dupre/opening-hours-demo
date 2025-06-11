use yew::prelude::*;

const EXPRESSIONS: &[&str] = [
    "Mo-Fr 10:00-20:00; PH off",
    "24/7",
    "22:00+; PH off",
    "Fr-Sa 18:00-06:00; PH off",
    "Mo 10:00-12:00,12:30-15:00",
    "Mo,Tu,Th,Fr 12:00-18:00; Sa,PH 12:00-17:00; Th[3],Th[-1] off",
    "Mo-Fr 08:00-11:00 || Tu-Th,PH open \"Emergency only\"",
    "Apr-Oct Su[2] 14:00-18:00; Aug Su[-1] -1 day 10:00-18:00; PH off",
]
.as_slice();

#[derive(Properties, PartialEq)]
pub struct Props {
    pub set_raw_oh: Callback<String, ()>,
}

#[function_component]
pub fn Examples(props: &Props) -> Html {
    let onclick_set = |raw: &'static str| {
        let set_raw_oh = props.set_raw_oh.clone();
        move |_| set_raw_oh.emit(raw.to_string())
    };

    html! {
      <section>
        <p>
          {"Here are a few examples of valid expressions :"}
        </p>
        <p>
          {
            EXPRESSIONS.iter().map(|expr| {
              html!{<button onclick={onclick_set(expr)}>{expr}</button>}
            }).collect::<Html>()
          }
        </p>
      </section>
    }
}
