pub mod component;
pub mod links;
pub mod parse;
pub mod section;
pub mod utils;

use std::cell::OnceCell;

use chrono::{DateTime, Local};
use chrono_tz::Tz;
use opening_hours::localization::{Coordinates, Country, TzLocation};
use serde::{Deserialize, Serialize};
use yew::prelude::*;
use yew_router::{
    BrowserRouter, Routable,
    history::{BrowserHistory, History},
    hooks::{use_location, use_navigator},
    prelude::{Location, Navigator},
};

pub(crate) type OpeningHours = opening_hours::OpeningHours<TzLocation<Tz>>;

#[derive(Clone, Debug, Default)]
struct EvalContextCache {
    oh: OnceCell<Option<OpeningHours>>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct EvalContext {
    #[serde(default, rename = "oh", skip_serializing_if = "String::is_empty")]
    pub raw_oh: String,
    pub dt: Option<DateTime<Local>>,
    #[serde(default, skip)]
    cache: EvalContextCache,
}

impl PartialEq for EvalContext {
    fn eq(&self, other: &Self) -> bool {
        self.raw_oh == other.raw_oh && self.dt == other.dt
    }
}

impl Eq for EvalContext {}

impl EvalContext {
    pub(crate) fn oh(&self) -> Option<&OpeningHours> {
        (self.cache.oh)
            .get_or_init(|| {
                let loc = TzLocation::new(chrono_tz::Europe::Paris)
                    .with_coords(Coordinates::new(48.85, 2.35).unwrap());

                let ctx = opening_hours::Context::default()
                    .with_holidays(Country::US.holidays())
                    .with_locale(loc);

                let oh: opening_hours::OpeningHours = self.raw_oh.parse().ok()?;
                Some(oh.with_context(ctx))
            })
            .as_ref()
    }

    pub(crate) fn dt(&self) -> DateTime<Local> {
        self.dt.unwrap_or_else(Local::now)
    }

    fn new_from_url(location: &Location) -> Self {
        location
            .query::<Self>()
            .inspect_err(|err| log::warn!("Could not parse evaluation state from URL: {err}"))
            .unwrap_or_default()
    }

    fn with_raw_oh(self, oh_raw: String) -> Self {
        Self {
            raw_oh: oh_raw,
            cache: EvalContextCache::default(),
            ..self
        }
    }

    fn update_url(&self, location: &Location, navigator: &Navigator) {
        let prev_val = Self::new_from_url(location);

        if &prev_val == self {
            return;
        }

        let result = {
            if prev_val.oh().is_some() {
                log::info!("Push {:?}", self.raw_oh);
                navigator.push_with_query(&Route::Root, self)
            } else {
                log::info!("Replace {:?}", self.raw_oh);
                navigator.replace_with_query(&Route::Root, self)
            }
        };

        if let Err(err) = result {
            log::warn!("Could not update history: {err}")
        };
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Routable)]
enum Route {
    #[at("/")]
    Root,
}

#[function_component]
fn App() -> Html {
    let expression_ref = use_node_ref();
    let ctx = use_state(EvalContext::default);
    let navigator = use_navigator();

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
    let location = use_location();

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
