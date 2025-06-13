use std::fmt::Display;
use std::ops::Deref;

use chrono::{Duration, Utc};

// --
// -- Text Manipuation
// --

pub fn capitalize(s: &str) -> String {
    let mut s_chars = s.chars();
    let mut res = String::with_capacity(s.len());

    if let Some(c) = s_chars.next() {
        res.extend(c.to_uppercase());
    }

    res.push_str(s_chars.as_str());
    res
}

// --
// -- Pretty Duration
// --

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct PrettyDuration(pub Duration);

impl Deref for PrettyDuration {
    type Target = Duration;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for PrettyDuration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let suffixes = ["", "m", "μ", "n"];
        let mut curr = self.as_seconds_f64();

        for suffix in suffixes {
            if curr >= 10.0 {
                return write!(f, "{curr:.0}{suffix}s");
            } else if curr >= 1.0 {
                return write!(f, "{curr:.1}{suffix}s");
            }

            curr *= 1000.0;
        }

        write!(f, "0s")
    }
}

// --
// -- Time Measurement
// --

const MIN_MEASURE_TIME: Duration = Duration::milliseconds(10);

pub fn measure_time<T>(mut f: impl FnMut() -> T) -> (PrettyDuration, T) {
    let mut count = 1;
    let mut res = f();
    let start_time = Utc::now();

    while Utc::now() - start_time < MIN_MEASURE_TIME {
        count += 1;
        res = f();
    }

    let duration = (Utc::now() - start_time) / count;
    (PrettyDuration(duration), res)
}
