use std::cell::OnceCell;

use chrono::{DateTime, Local};
use opening_hours::localization::{Coordinates, Country, TzLocation};
use serde::{Deserialize, Serialize};
use yew::Callback;
use yew_router::prelude::{Location, Navigator};

use crate::{OpeningHours, Route};

pub type CallbackUpdateCtx = Callback<Box<dyn FnMut(EvalContext) -> EvalContext>, ()>;

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
    pub fn oh(&self) -> Option<&OpeningHours> {
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

    pub fn local_dt(&self) -> DateTime<Local> {
        self.dt.unwrap_or_else(Local::now)
    }

    pub fn new_from_url(location: &Location) -> Self {
        location
            .query::<Self>()
            .inspect_err(|err| log::warn!("Could not parse evaluation state from URL: {err}"))
            .unwrap_or_default()
    }

    pub fn with_raw_oh(self, oh_raw: String) -> Self {
        Self {
            raw_oh: oh_raw,
            cache: EvalContextCache::default(),
            ..self
        }
    }

    pub fn with_dt(self, dt: DateTime<Local>) -> Self {
        Self {
            dt: Some(dt),
            cache: EvalContextCache::default(),
            ..self
        }
    }

    pub fn with_no_dt(self) -> Self {
        Self {
            dt: None,
            cache: EvalContextCache::default(),
            ..self
        }
    }

    pub fn update_url(&self, location: &Location, navigator: &Navigator) {
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
