use std::{fmt::Display, str::FromStr};

use chrono::{NaiveDateTime, TimeZone, Utc};
use chrono_tz::{ParseError, Tz, TZ_VARIANTS};
use dioxus::prelude::*;
use dioxus_free_icons::icons::bs_icons::BsClock;
use strum::IntoEnumIterator;

use crate::{
    components::inputs::{SelectForm, SelectFormEnum, TextInput},
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
    use_shared_state_provider(cx, || DateConverterState {
        time_zone: DcTimeZone::default(),
        time: Utc::now().naive_utc(),
    });
    let date_state = use_shared_state::<DateConverterState>(cx).unwrap();

    let date_time_str = date_state
        .read()
        .time_zone
        .inner()
        .from_utc_datetime(&date_state.read().time)
        .to_string();

    cx.render(rsx! {
        div {
            class: "date-converter",
            SelectForm::<DcTimeZone> {
                label: "Time Zone",
                oninput: move |tz: DcTimeZone| {
                    println!("Time Zone: {}", tz);
                    date_state.write().time_zone = tz;
                }
                value: date_state.read().time_zone,
            }
            TextInput {
                label: "Date",
                value: "{date_time_str}",
                oninput: move |_| {}
                readonly: true
            }
        }
    })
}

#[derive(Clone, Copy)]
struct DateConverterState {
    time_zone: DcTimeZone,
    time: NaiveDateTime,
}

#[derive(Copy, Clone, Debug)]
enum DcTimeZone {
    Base(Tz),
}

impl Default for DcTimeZone {
    fn default() -> Self {
        Self::Base(Tz::UTC)
    }
}

impl Display for DcTimeZone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Base(tz) => write!(f, "{}", tz),
        }
    }
}

impl FromStr for DcTimeZone {
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::Base(Tz::from_str(s)?))
    }

    type Err = ParseError;
}

impl PartialEq for DcTimeZone {
    fn eq(&self, other: &Self) -> bool {
        self.inner() == other.inner()
    }
}

impl IntoEnumIterator for DcTimeZone {
    fn iter() -> Self::Iterator {
        TZ_VARIANTS
            .iter()
            .map(|tz| Self::Base(*tz))
            .collect::<Vec<_>>()
            .into_iter()
    }

    type Iterator = std::vec::IntoIter<Self>;
}

impl From<DcTimeZone> for &'static str {
    fn from(val: DcTimeZone) -> Self {
        match val {
            DcTimeZone::Base(tz) => tz.name(),
        }
    }
}

impl SelectFormEnum for DcTimeZone {}

impl DcTimeZone {
    fn inner(&self) -> Tz {
        match self {
            Self::Base(tz) => *tz,
        }
    }
}
