pub(crate) mod component;
pub(crate) mod eval;
pub(crate) mod links;
pub(crate) mod section;
pub(crate) mod utils;

use chrono_tz::Tz;
use opening_hours::localization::TzLocation;
use yew::prelude::*;
use yew_router::history::{BrowserHistory, History};
use yew_router::prelude::*;

use crate::eval::EvalContext;

pub(crate) type OpeningHours = opening_hours::OpeningHours<TzLocation<Tz>>;

#[derive(Debug, Clone, Copy, PartialEq, Routable)]
enum Route {
    #[at("/")]
    Root,
    /// TODO
    #[at("/s/{blob}/")]
    ShortLink,
}

#[function_component]
fn App() -> Html {
    let expression_ref = use_node_ref();
    let navigator = use_navigator();
    let location = use_location();

    let ctx = use_state(|| {
        (location.as_ref())
            .map(EvalContext::new_from_url)
            .unwrap_or_default()
    });

    let cb_update_ctx = use_callback(ctx.clone(), {
        let navigator = navigator.clone();

        move |mut mutation: Box<dyn FnMut(EvalContext) -> EvalContext>, ctx| {
            let new_ctx = (mutation)((**ctx).clone());

            if **ctx == new_ctx {
                return;
            }

            // Don't fetch the location from hook because it might not have been updated if there
            // was no render.
            let location = BrowserHistory::new().location();

            if let Some(navigator) = &navigator {
                new_ctx.update_url(&location, navigator);
            }

            ctx.set(new_ctx)
        }
    });

    let update_ctx = use_callback(ctx.clone(), {
        let navigator = navigator.clone();

        move |new_ctx: EvalContext, ctx| {
            if **ctx == new_ctx {
                return;
            }

            // Don't fetch the location from hook because it might not have been updated if there
            // was no render.
            let location = BrowserHistory::new().location();

            if let Some(navigator) = &navigator {
                new_ctx.update_url(&location, navigator);
            }

            ctx.set(new_ctx)
        }
    });

    // Update the context any time the URL changes (eg. by click back button)
    use_effect_with(location, {
        let update_ctx = update_ctx.clone();

        move |location| {
            let new_ctx = {
                if let Some(location) = &location {
                    EvalContext::new_from_url(location)
                } else {
                    EvalContext::default()
                }
            };

            update_ctx.emit(new_ctx)
        }
    });

    html! {
      <>
        <main class="container">
          <component::expression::Expression
            raw_oh={ctx.raw_oh.clone()}
            is_valid={ctx.oh().is_some()}
            cb_update_ctx={cb_update_ctx.clone()}
            expression_ref={expression_ref}
          />

          <section::context_form::ContextForm
            ctx={ctx.clone()}
            cb_update_ctx={cb_update_ctx.clone()}
          />

          <hr />

          if ctx.oh().is_some() {
            <section::title::Title ctx={ctx.clone()} />
            <section::properties::Properties ctx={ctx.clone()} />
            <section::schedule::Schedule ctx={ctx.clone()} />
          } else {
            <section::examples::Examples cb_update_ctx={cb_update_ctx} />
          }

        </main>

        <footer class="container">
          <section::information::Information />
        </footer>
      </>
    }
}

#[function_component]
fn Root() -> Html {
    html! {
        <BrowserRouter>
          <App />
        </BrowserRouter>
    }
}

fn main() {
    let log_level = {
        if cfg!(debug_assertions) {
            log::Level::Debug
        } else {
            log::Level::Info
        }
    };

    wasm_logger::init(wasm_logger::Config::new(log_level));
    yew::Renderer::<Root>::new().render();
}
