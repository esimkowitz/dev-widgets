use std::{fmt::Display, str::FromStr};

use chrono::{NaiveDateTime, TimeZone, Utc, Datelike, Timelike};
use chrono_tz::{ParseError, Tz, TZ_VARIANTS};
use dioxus::prelude::*;
use dioxus_free_icons::icons::bs_icons::BsClock;
use strum::IntoEnumIterator;

use crate::{
    components::inputs::{SelectForm, SelectFormEnum, TextInput, NumberInput},
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
                    date_state.write().time_zone = tz;
                },
                value: date_state.read().time_zone,
            }
            TextInput {
                label: "Date",
                value: "{date_time_str}",
                oninput: |_| {},
                readonly: true,
            }
            div {
                class: "ymd-selectors",
                NumberInput::<i32> {
                    class: "year",
                    label: "Year",
                    value: date_state.read().time.year(),
                    onchange: move |year| {
                        let datetime = date_state.read().time;
                        date_state.write().time = datetime.with_year(year).unwrap_or(datetime);
                    }
                }
                NumberInput::<u32> {
                    class: "month",
                    label: "Month",
                    value: date_state.read().time.month(),
                    onchange: move |month| {
                        let datetime = date_state.read().time;
                        date_state.write().time = datetime.with_month(month).unwrap_or(datetime);
                    }
                }
                NumberInput::<u32> {
                    class: "day",
                    label: "Day",
                    value: date_state.read().time.day(),
                    onchange: move |day| {
                        let datetime = date_state.read().time;
                        date_state.write().time = datetime.with_day(day).unwrap_or(datetime);
                    }
                }
                NumberInput::<u32> {
                    class: "hour",
                    label: "Hour",
                    value: date_state.read().time.hour(),
                    onchange: move |hour| {
                        let datetime = date_state.read().time;
                        date_state.write().time = datetime.with_hour(hour).unwrap_or(datetime);
                    }
                }
                NumberInput::<u32> {
                    class: "minute",
                    label: "Minute",
                    value: date_state.read().time.minute(),
                    onchange: move |minute| {
                        let datetime = date_state.read().time;
                        date_state.write().time = datetime.with_minute(minute).unwrap_or(datetime);
                    }
                }
                NumberInput::<u32> {
                    class: "second",
                    label: "Second",
                    value: date_state.read().time.second(),
                    onchange: move |second| {
                        let datetime = date_state.read().time;
                        date_state.write().time = datetime.with_second(second).unwrap_or(datetime);
                    }
                }
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
        write!(f, "{}", self.inner())
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
        val.inner().name()
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
