use yew::prelude::*;

use crate::component::icon::{GitHub, Icon};
use crate::links::URL_REPO;

#[function_component]
pub fn Footer() -> Html {
    html! {
      <footer>
        <a href={URL_REPO} target={"_blank"}>
          <Icon src={GitHub} />
          {" remi-dupre/opening-hours-rs"}
        </a>
      </footer>
    }
}
