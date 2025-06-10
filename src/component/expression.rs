use web_sys::HtmlTextAreaElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub expression_ref: NodeRef,
    pub set_raw_oh: Callback<String, ()>,
}

#[function_component]
pub fn Expression(props: &Props) -> Html {
    let set_raw_oh = props.set_raw_oh.clone();

    let oninput = move |input: InputEvent| {
        let Some(el) = input.target_dyn_into::<HtmlTextAreaElement>() else {
            log::error!("Could not fetch text area");
            return;
        };

        set_raw_oh.emit(el.value());
    };

    html! {
      <textarea id="expression" ref={props.expression_ref.clone()} {oninput}></textarea>
    }
}
