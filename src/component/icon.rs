use yew::prelude::*;

pub use Src::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub src: Src,
}

#[function_component]
pub fn Icon(props: &Props) -> Html {
    html! {
      <span class={"icon-container"}>
        {props.src.html()}
      </span>
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Src {
    // Icons from pictogrammers: https://pictogrammers.com/library/mdi/
    GitHub,
}

impl Src {
    fn raw_html(self) -> &'static str {
        match self {
            Self::GitHub => include_str!("../../static/icons/github.svg"),
        }
    }

    fn html(self) -> Html {
        Html::from_html_unchecked(self.raw_html().into())
    }
}
