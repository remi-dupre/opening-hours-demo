use web_sys::HtmlTextAreaElement;
use yew::prelude::*;

use crate::EvalContext;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub(crate) ctx: UseStateHandle<EvalContext>,
    pub(crate) update_ctx: Callback<EvalContext, ()>,
    pub(crate) expression_ref: NodeRef,
}

#[function_component]
pub fn Expression(props: &Props) -> Html {
    let oninput = use_callback((), {
        let ctx = props.ctx.clone();
        let update_ctx = props.update_ctx.clone();

        move |input: InputEvent, ()| {
            let Some(el) = input.target_dyn_into::<HtmlTextAreaElement>() else {
                log::error!("Could not fetch text area");
                return;
            };

            let new_ctx = (*ctx).clone().with_raw_oh(el.value());
            update_ctx.emit(new_ctx)
        }
    });

    let invalid = {
        if props.ctx.raw_oh.is_empty() {
            ""
        } else if props.ctx.oh().is_some() {
            "false"
        } else {
            "true"
        }
    };

    html! {
      <textarea
        ref={props.expression_ref.clone()}
        value={props.ctx.raw_oh.clone()}
        {oninput}
        placeholder="Enter an opening hours expression"
        row="2"
        aria-invalid={invalid}
      />
    }
}
