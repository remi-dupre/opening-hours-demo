use chrono_tz::Tz;

use opening_hours::localization::{Coordinates, Country, TzLocation};
use opening_hours::{Context, OpeningHours};

use crate::utils::{PrettyDuration, measure_time};

#[derive(Clone, PartialEq)]
pub struct ParsedOh {
    pub raw: String,
    pub oh: OpeningHours<TzLocation<Tz>>,
    pub time_parsing: PrettyDuration,
}

impl ParsedOh {
    pub fn new(raw: String) -> Result<Self, opening_hours_syntax::Error> {
        let loc = TzLocation::new(chrono_tz::Europe::Paris)
            .with_coords(Coordinates::new(48.85, 2.35).unwrap());

        let ctx = Context::default()
            .with_holidays(Country::US.holidays())
            .with_locale(loc);

        let (time_parsing, oh) = measure_time(|| OpeningHours::parse(&raw));
        let oh = oh?.with_context(ctx);

        Ok(Self {
            raw,
            oh,
            time_parsing,
        })
    }
}
