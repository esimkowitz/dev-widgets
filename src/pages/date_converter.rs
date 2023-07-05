use std::{fmt::Display, str::FromStr};

use chrono::{DateTime, Datelike, NaiveDateTime, TimeZone, Timelike, Utc};
use chrono_tz::{ParseError, Tz, TZ_VARIANTS};
use dioxus::prelude::*;
use dioxus_free_icons::icons::bs_icons::BsClock;
use strum::IntoEnumIterator;

use crate::{
    components::inputs::{NumberInput, SelectForm, SelectFormEnum, TextInput},
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

    let date_time_str = date_state.read().local_datetime().to_string();

    let unix_time_str = date_state.read().time.timestamp().to_string();

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
                readonly: true,
            }
            TextInput {
                label: "Unix Timestamp",
                value: "{unix_time_str}",
                onchange: move |event: Event<FormData>| {
                    let new_unix_time = event.value.clone();
                    if let Ok(unix_time) = i64::from_str(&new_unix_time) {
                        date_state.write().time = match Utc.timestamp_opt(unix_time, 0) {
                            chrono::LocalResult::Single(datetime) => datetime.naive_utc(),
                            _ => date_state.read().time,
                        };
                    }
                }
            }
            div {
                class: "ymd-selectors",
                NumberInput::<i32> {
                    class: "year",
                    label: "Year",
                    value: date_state.read().local_datetime().year(),
                    onchange: move |year| {
                        let datetime = date_state.read().local_datetime();
                        date_state.write().set_local_datetime(datetime.with_year(year).unwrap_or(datetime));
                    }
                }
                NumberInput::<u32> {
                    class: "month",
                    label: "Month",
                    value: date_state.read().local_datetime().month(),
                    onchange: move |month| {
                        let datetime = date_state.read().local_datetime();
                        date_state.write().set_local_datetime(datetime.with_month(month).unwrap_or(datetime));
                    }
                }
                NumberInput::<u32> {
                    class: "day",
                    label: "Day",
                    value: date_state.read().local_datetime().day(),
                    onchange: move |day| {
                        let datetime = date_state.read().local_datetime();
                        date_state.write().set_local_datetime(datetime.with_day(day).unwrap_or(datetime));
                    }
                }
                NumberInput::<u32> {
                    class: "hour",
                    label: "Hour",
                    value: date_state.read().local_datetime().hour(),
                    onchange: move |hour| {
                        let datetime = date_state.read().local_datetime();
                        date_state.write().set_local_datetime(datetime.with_hour(hour).unwrap_or(datetime));
                    }
                }
                NumberInput::<u32> {
                    class: "minute",
                    label: "Minute",
                    value: date_state.read().local_datetime().minute(),
                    onchange: move |minute| {
                        let datetime = date_state.read().local_datetime();
                        date_state.write().set_local_datetime(datetime.with_minute(minute).unwrap_or(datetime));
                    }
                }
                NumberInput::<u32> {
                    class: "second",
                    label: "Second",
                    value: date_state.read().local_datetime().second(),
                    onchange: move |second| {
                        let datetime = date_state.read().local_datetime();
                        date_state.write().set_local_datetime(datetime.with_second(second).unwrap_or(datetime));
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

impl DateConverterState {
    fn local_datetime(&self) -> DateTime<Tz> {
        self.time_zone.inner().from_utc_datetime(&self.time)
    }

    fn set_local_datetime(&mut self, datetime: DateTime<Tz>) {
        self.time = datetime.naive_utc();
    }
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
