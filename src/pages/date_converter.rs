use std::{fmt::Display, str::FromStr};

use chrono_tz::{ParseError, Tz, TZ_VARIANTS};
use dioxus::prelude::*;
use dioxus_free_icons::icons::bs_icons::BsClock;
use strum::IntoEnumIterator;

use crate::{
    components::inputs::{SelectForm, SelectFormEnum},
    widget_entry::{WidgetEntry, WidgetIcon},
};

pub const WIDGET_ENTRY: WidgetEntry = WidgetEntry {
    title: "Date Converter",
    short_title: "Date",
    description: "Convert dates between formats",
    path: "/date-converter",
    function: date_converter,
    icon: move |cx| ICON.icon(cx),
};

const ICON: WidgetIcon<BsClock> = WidgetIcon { icon: BsClock };

pub fn date_converter(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            class: "date-converter",
            SelectForm::<TimeZone> {
                label: "Time Zone",
                oninput: move |tz: TimeZone| {
                    println!("Time Zone: {}", tz);
                }
            }
        }
    })
}

#[derive(Copy, Clone, Debug, Hash)]
enum TimeZone {
    Base(Tz),
}

impl Default for TimeZone {
    fn default() -> Self {
        Self::Base(Tz::UTC)
    }
}

impl Display for TimeZone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Base(tz) => write!(f, "{}", tz),
        }
    }
}

impl FromStr for TimeZone {
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::Base(Tz::from_str(s)?))
    }

    type Err = ParseError;
}

impl IntoEnumIterator for TimeZone {
    fn iter() -> Self::Iterator {
        TZ_VARIANTS
            .iter()
            .map(|tz| Self::Base(*tz))
            .collect::<Vec<_>>()
            .into_iter()
    }

    type Iterator = std::vec::IntoIter<Self>;
}

impl From<TimeZone> for &'static str {
    fn from(val: TimeZone) -> Self {
        match val {
            TimeZone::Base(tz) => tz.name(),
        }
    }
}

impl SelectFormEnum for TimeZone {}
