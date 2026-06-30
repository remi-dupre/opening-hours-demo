use web_sys::HtmlTextAreaElement;
use yew::prelude::*;

use crate::eval::{CallbackUpdateCtx, EvalContext};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub raw_oh: String,
    pub is_valid: bool,
    pub cb_update_ctx: CallbackUpdateCtx,
    pub expression_ref: NodeRef,
}

#[function_component]
pub fn Expression(props: &Props) -> Html {
    let oninput = use_callback((), {
        let cb_update_ctx = props.cb_update_ctx.clone();

        move |input: InputEvent, ()| {
            let Some(el) = input.target_dyn_into::<HtmlTextAreaElement>() else {
                log::error!("Could not fetch text area");
                return;
            };

            cb_update_ctx.emit(Box::new(move |ctx: EvalContext| {
                ctx.with_raw_oh(el.value())
            }))
        }
    });

    let invalid = {
        if props.raw_oh.is_empty() {
            ""
        } else if props.is_valid {
            "false"
        } else {
            "true"
        }
    };

    html! {
      <textarea
        ref={props.expression_ref.clone()}
        value={props.raw_oh.clone()}
        aria-invalid={invalid}
        placeholder="Enter an opening hours expression"
        row="2"
        {oninput}
      />
    }
}
