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

macro_rules! declare_src {
    ( $( ( $name: ident, $file: literal ) , )* ) => {
        #[derive(Clone, Copy, PartialEq, Eq)]
        pub enum Src {
            $( $name ),*
        }

        impl Src {
            fn raw_html(self) -> &'static str {
                match self {
                    $( $name => include_str!(concat!("../../static/icons/", $file)) ),*
                }
            }

            fn html(self) -> Html {
                Html::from_html_unchecked(self.raw_html().into())
            }
        }
    };
}

declare_src! {
    // Pictogrammers: https://pictogrammers.com/library/mdi/
    (GitHub, "github.svg"),
    // Homemade
    (ArrowDropdown, "arrow-dropdown.svg"),
}
