use std::cell::OnceCell;

use chrono::{DateTime, Local};
use opening_hours::localization::{Coordinates, Country, TzLocation};
use opening_hours_syntax::Parser;
use serde::{Deserialize, Serialize};
use yew::Callback;
use yew_router::{
    AnyRoute,
    prelude::{Location, Navigator},
};

use crate::OpeningHours;

/// Paris
const DEFAULT_COORDS: Coordinates = Coordinates::new(48.85, 2.34).unwrap();

pub type CallbackUpdateCtx = Callback<Box<dyn FnMut(EvalContext) -> EvalContext>, ()>;

#[derive(Clone, Debug, Default)]
struct EvalContextCache {
    oh: OnceCell<Option<(OpeningHours, Vec<String>)>>,
    location: OnceCell<TzLocation<chrono_tz::Tz>>,
    country: OnceCell<Option<Country>>,
}

impl PartialEq for EvalContextCache {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq)]
pub struct EvalContext {
    #[serde(default, rename = "oh", skip_serializing_if = "String::is_empty")]
    pub raw_oh: String,
    pub dt: Option<DateTime<Local>>,
    #[serde(default, flatten, with = "serde_coord")]
    pub coords: Option<Coordinates>,
    #[serde(default, skip)]
    cache: EvalContextCache,
}

impl EvalContext {
    fn get_oh_and_warnings(&self) -> Option<(&OpeningHours, &[String])> {
        (self.cache.oh)
            .get_or_init(|| {
                let loc = TzLocation::new(chrono_tz::Europe::Paris).with_coords(self.get_coords());
                let mut ctx = opening_hours::Context::default().with_locale(loc);
                let mut warnings = Vec::new();

                if let Some(country) = Country::try_from_coords(self.get_coords()) {
                    ctx = ctx
                        .with_holidays(country.holidays())
                        .with_holidays_unknown(country.holidays_regional());
                }

                let mut parser = Parser::default()
                    .with_warning_handler(|warning| warnings.push(warning.to_string()));

                let oh = opening_hours::OpeningHours::parse_with(&mut parser, &self.raw_oh).ok()?;
                Some((oh.with_context(ctx), warnings))
            })
            .as_ref()
            .map(|(oh, w)| (oh, w.as_slice()))
    }

    pub fn get_oh(&self) -> Option<&OpeningHours> {
        self.get_oh_and_warnings().map(|(oh, _)| oh)
    }

    pub fn get_warnings(&self) -> &[String] {
        self.get_oh_and_warnings()
            .map(|(_, w)| w)
            .unwrap_or_default()
    }

    pub fn get_local_dt(&self) -> DateTime<Local> {
        self.dt.unwrap_or_else(Local::now)
    }

    pub fn get_coords(&self) -> Coordinates {
        self.coords.unwrap_or(DEFAULT_COORDS)
    }

    pub fn get_country(&self) -> Option<Country> {
        *(self.cache)
            .country
            .get_or_init(|| Country::try_from_coords(self.get_coords()))
    }

    pub fn get_location(&self) -> &TzLocation<chrono_tz::Tz> {
        (self.cache)
            .location
            .get_or_init(|| TzLocation::from_coords(self.get_coords()))
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

    pub fn with_coords(self, coords: Coordinates) -> Self {
        let coords = {
            if coords == DEFAULT_COORDS {
                None
            } else {
                Some(coords)
            }
        };

        Self {
            coords,
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
            if prev_val.get_oh().is_some() {
                log::info!("Push {:?}", self.raw_oh);
                navigator.push_with_query(&AnyRoute::new(location.path()), self)
            } else {
                log::info!("Replace {:?}", self.raw_oh);
                navigator.replace_with_query(&AnyRoute::new(location.path()), self)
            }
        };

        if let Err(err) = result {
            log::warn!("Could not update history: {err}")
        };
    }
}

/// --
/// -- Serde conversion for coordinates
/// --
mod serde_coord {
    use opening_hours::localization::Coordinates;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    #[derive(Debug, Deserialize, Serialize)]
    struct Wrapper {
        lat: String,
        lon: String,
    }

    impl From<Coordinates> for Wrapper {
        fn from(value: Coordinates) -> Self {
            Self {
                lat: value.lat().to_string(),
                lon: value.lon().to_string(),
            }
        }
    }

    impl TryInto<Coordinates> for Wrapper {
        type Error = ();

        fn try_into(self) -> Result<Coordinates, Self::Error> {
            Coordinates::new(
                self.lat.parse().map_err(|_| ())?,
                self.lon.parse().map_err(|_| ())?,
            )
            .ok_or(())
        }
    }

    pub(crate) fn serialize<S>(coords: &Option<Coordinates>, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        coords.map(Wrapper::from).serialize(s)
    }

    pub(crate) fn deserialize<'de, D>(d: D) -> Result<Option<Coordinates>, D::Error>
    where
        D: Deserializer<'de>,
    {
        <Option<Wrapper>>::deserialize(d).map(|opt| opt.and_then(|wrapped| wrapped.try_into().ok()))
    }
}
