use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub class: Classes,
    pub onclick_outside: Callback<MouseEvent>,
    pub children: Html,
}

#[function_component]
pub fn Popup(props: &Props) -> Html {
    html! {
      <div class={"popup-container"}>
        <div class="popup-blur" onclick={&props.onclick_outside} />
        <div class={classes!("popup", props.class.clone())}>
          {props.children.clone()}
        </div>
      </div>
    }
}
