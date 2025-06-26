use yew::prelude::*;

use crate::component::icon::{GitHub, Icon};
use crate::links::{URL_REPO, URL_REPO_DEMO};

#[function_component]
pub fn Footer() -> Html {
    html! {
      <footer>
        <a href={URL_REPO} target={"_blank"}>
          <Icon src={GitHub} />
          {" remi-dupre/opening-hours-rs"}
        </a>
        {" - "}
        <a href={URL_REPO_DEMO} target={"_blank"}>
          <Icon src={GitHub} />
          {" demo"}
        </a>
      </footer>
    }
}
