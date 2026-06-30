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
            ctx={ctx.clone()}
            update_ctx={update_ctx.clone()}
            expression_ref={expression_ref}
          />

          <details name="example" open={false}>
            <summary>
              <em>{"Evaluate from 2026/06/25 12:00:00 in France "}</em>
            </summary>
            <article>
            <h4>{"Change current evaluation date"}</h4>
            <form>
              <fieldset>
                <label>
                  <input name="terms" type="checkbox" role="switch" />
                  {"Evaluate from current date"}
                </label>
              </fieldset>
              <fieldset role="group">
                <input type="submit" value="-1d" />
                <input type="datetime-local" class="outline" />
                <input type="submit" value="+1d" />
              </fieldset>
            </form>
            <h4>{"Change evaluation position"}</h4>
            <form>
              <fieldset role="group">
                <input type="number" step="0.01" placeholder="Latitude" />
                <input type="number" step="0.01" placeholder="Longitude" />
              </fieldset>
              <p>
                {"Detected country: France. Detected timezone: Europe/Paris."}
              </p>
            </form>
            </article>
          </details>

          <hr />

          if ctx.oh().is_some() {
            <section::title::Title ctx={ctx.clone()} />
            <section::properties::Properties ctx={ctx.clone()} />
            <section::schedule::Schedule ctx={ctx.clone()} />
          } else {
            <section::examples::Examples ctx={ctx} update_ctx={update_ctx} />
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
